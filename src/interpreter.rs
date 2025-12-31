// Argon Interpreter - Executes AST
// Compatible with compiler.ar v2.24.0 (GC + Defer)

#![allow(dead_code)]

use crate::parser::{Expr, Stmt, TopLevel, Function, Param};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Struct(String, Rc<RefCell<HashMap<String, Value>>>),
    Function(String, Vec<Param>, Option<Vec<Stmt>>),
}

impl Value {
    pub fn to_string_val(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.borrow().iter().map(|v| v.to_string_val()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Struct(name, fields) => {
                let items: Vec<String> = fields.borrow().iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string_val()))
                    .collect();
                format!("{} {{ {} }}", name, items.join(", "))
            }
            Value::Function(name, _, _) => format!("<fn {}>", name),
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.borrow().is_empty(),
            _ => true,
        }
    }
    
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(n) => *n,
            Value::Bool(b) => if *b { 1 } else { 0 },
            Value::String(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
}

struct ScopeFrame {
    vars: HashMap<String, Value>,
    deferred: Vec<Stmt>,
}

impl ScopeFrame {
    fn new() -> Self {
        Self { vars: HashMap::new(), deferred: Vec::new() }
    }
}

pub struct Interpreter {
    globals: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    stack: Vec<ScopeFrame>,
    emit_llvm: bool,
    llvm_output: String,
    llvm_buffer: String,
    program_args: Vec<String>,
    methods: HashMap<(String, String), Function>,
}

#[derive(Debug)]
pub enum ControlFlow {
    Return(Value),
    Break,
    Continue,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
            functions: HashMap::new(),
            stack: vec![ScopeFrame::new()],
            emit_llvm: false,
            llvm_output: String::new(),
            llvm_buffer: String::new(),
            program_args: Vec::new(),
            methods: HashMap::new(),
        }
    }
    
    pub fn set_emit_llvm(&mut self, emit: bool, output: &str) {
        self.emit_llvm = emit;
        self.llvm_output = output.to_string();
    }
    
    pub fn set_args(&mut self, args: Vec<String>) {
        self.program_args = args;
    }
    
    fn get_var(&self, name: &str) -> Value {
        for scope in self.stack.iter().rev() {
            if let Some(val) = scope.vars.get(name) {
                return val.clone();
            }
        }
        if let Some(val) = self.globals.get(name) {
            return val.clone();
        }
        Value::Null
    }
    
    fn set_var(&mut self, name: &str, val: Value) {
        for scope in self.stack.iter_mut().rev() {
            if scope.vars.contains_key(name) {
                scope.vars.insert(name.to_string(), val);
                return;
            }
        }
        if self.globals.contains_key(name) {
            self.globals.insert(name.to_string(), val);
            return;
        }
        if let Some(scope) = self.stack.last_mut() {
            scope.vars.insert(name.to_string(), val);
        }
    }
    
    fn declare_var(&mut self, name: &str, val: Value) {
        if let Some(scope) = self.stack.last_mut() {
            scope.vars.insert(name.to_string(), val);
        }
    }
    
    fn push_scope(&mut self) {
        self.stack.push(ScopeFrame::new());
    }
    
    fn pop_scope(&mut self) -> Result<(), ControlFlow> {
        if self.stack.len() <= 1 { return Ok(()); }
        
        let mut final_result = Ok(());
        
        // Execute deferred statements
        let deferred_stmts = if let Some(scope) = self.stack.last_mut() {
             std::mem::take(&mut scope.deferred)
        } else {
             Vec::new()
        };
        
        for stmt in deferred_stmts.into_iter().rev() {
            match self.exec_stmt(&stmt) {
                 Ok(_) => {},
                 Err(e) => {
                     final_result = Err(e);
                 }
            }
        }
        
        self.stack.pop();
        final_result
    }
    
    pub fn run(&mut self, ast: &[TopLevel]) -> Result<Value, String> {
        for item in ast {
            match item {
                TopLevel::Function(f) => {
                    self.functions.insert(f.name.clone(), f.clone());
                }
                TopLevel::Let(name, expr) => {
                    let val = self.eval_expr(expr)?;
                    self.globals.insert(name.clone(), val);
                }
                TopLevel::Impl(impl_def) => {
                    for method in &impl_def.methods {
                        self.methods.insert((impl_def.type_name.clone(), method.name.clone()), method.clone());
                    }
                }
                _ => {}
            }
        }
        
        if self.functions.contains_key("main") {
            return self.call_function("main", vec![]);
        }
        
        Ok(Value::Null)
    }
    
    fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "print" => {
               if let Some(val) = args.first() {
                   if self.emit_llvm {
                       self.llvm_buffer.push_str(&val.to_string_val());
                       self.llvm_buffer.push('\n');
                   } else {
                       println!("{}", val.to_string_val());
                   }
               }
               return Ok(Value::Null);
            }
            "len" => {
                if let Some(val) = args.first() {
                    match val {
                        Value::String(s) => return Ok(Value::Int(s.len() as i64)),
                        Value::Array(arr) => return Ok(Value::Int(arr.borrow().len() as i64)),
                        _ => return Ok(Value::Int(0)),
                    }
                }
                return Ok(Value::Int(0));
            }
            "push" => {
                if args.len() >= 2 {
                    if let Value::Array(arr) = &args[0] {
                         arr.borrow_mut().push(args[1].clone());
                         return Ok(args[0].clone());
                    }
                }
                return Ok(Value::Null);
            }
            "substr" => {
                if args.len() >= 3 {
                    if let (Value::String(s), Value::Int(start), Value::Int(len)) = 
                        (&args[0], &args[1], &args[2]) 
                    {
                        let start = *start as usize;
                        let len = *len as usize;
                        let result: String = s.chars().skip(start).take(len).collect();
                        return Ok(Value::String(result));
                    }
                }
                return Ok(Value::String(String::new()));
            }
            "readFile" => {
                if let Some(Value::String(path)) = args.first() {
                    match std::fs::read_to_string(path) {
                        Ok(content) => return Ok(Value::String(content)),
                        Err(_) => return Ok(Value::String(String::new())),
                    }
                }
                return Ok(Value::String(String::new()));
            }
            "writeFile" => {
                if args.len() >= 2 {
                    if let (Value::String(path), Value::String(content)) = (&args[0], &args[1]) {
                        if let Ok(mut file) = File::create(path) {
                            let _ = file.write_all(content.as_bytes());
                        }
                    }
                }
                return Ok(Value::Null);
            }
            "fileExists" => {
                if let Some(Value::String(path)) = args.first() {
                    return Ok(Value::Bool(std::path::Path::new(path).exists()));
                }
                return Ok(Value::Bool(false));
            }
            "parseInt" => {
                if let Some(Value::String(s)) = args.first() {
                    return Ok(Value::Int(s.parse().unwrap_or(0)));
                }
                return Ok(Value::Int(0));
            }
            "toString" => {
                if let Some(val) = args.first() {
                    return Ok(Value::String(val.to_string_val()));
                }
                return Ok(Value::String(String::new()));
            }
            "get_args" | "getArgs" => {
                let arg_vals: Vec<Value> = self.program_args.iter().map(|s| Value::String(s.clone())).collect();
                return Ok(Value::Array(Rc::new(RefCell::new(arg_vals))));
            }
            "make_token" | "make_binop" | "make_unary" | "make_call" | 
            "make_if" | "make_while" | "make_func" | "make_return" | "make_let" | 
            "make_assign" | "make_block" | "make_print" | "make_ast_num" | 
            "make_ast_str" | "make_ast_id" | "make_ast_array" | "make_struct_def" |
            "make_struct_init" | "make_enum_def" | "make_match" | "make_index" => {
                return Ok(Value::Array(Rc::new(RefCell::new(args))));
            }
            _ => {}
        }
        
        let func = match self.functions.get(name) {
            Some(f) => f.clone(),
            None => return Err(format!("Undefined function: {}", name)),
        };
        
        self.execute_function(func, args)
    }
    
    fn execute_function(&mut self, func: Function, args: Vec<Value>) -> Result<Value, String> {
        self.push_scope();
        for (i, param) in func.params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or(Value::Null);
            self.declare_var(&param.name, val);
        }
        
        let result = if let Some(body) = &func.body {
            self.exec_stmts(body)
        } else {
            Ok(())
        };
        
        let pop_res = self.pop_scope();
        
        match (result, pop_res) {
             (Err(ControlFlow::Return(val)), _) => Ok(val), 
             (Ok(_), Err(ControlFlow::Return(val))) => Ok(val), 
             (Err(e), _) => Ok(Value::Null), // Other control flows invalid in function
             _ => Ok(Value::Null)
        }
    }
    
    fn exec_stmts(&mut self, stmts: &[Stmt]) -> Result<(), ControlFlow> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }
    
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<(), ControlFlow> {
        match stmt {
            Stmt::Let(name, _, expr) => {
                let val = self.eval_expr(expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                self.declare_var(name, val);
                Ok(())
            }
            Stmt::Defer(d_stmt) => {
                 if let Some(scope) = self.stack.last_mut() {
                     scope.deferred.push(*d_stmt.clone());
                 }
                 Ok(())
            }
            Stmt::Assign(name, expr) => {
                let val = self.eval_expr(expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                self.set_var(name, val);
                Ok(())
            }
            Stmt::IndexAssign(arr_expr, idx_expr, val_expr) => {
                let arr_val = self.eval_expr(arr_expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                let idx = self.eval_expr(idx_expr).map_err(|_| ControlFlow::Return(Value::Null))?.as_int() as usize;
                let val = self.eval_expr(val_expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                
                if let Value::Array(arr) = arr_val {
                    let mut vec = arr.borrow_mut();
                    if idx < vec.len() { vec[idx] = val; }
                }
                Ok(())
            }
            Stmt::FieldAssign(obj_expr, field, val_expr) => {
                let obj_val = self.eval_expr(obj_expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                let val = self.eval_expr(val_expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                if let Value::Struct(_, fields) = obj_val {
                    fields.borrow_mut().insert(field.clone(), val);
                }
                Ok(())
            }
            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval_expr(e).map_err(|_| ControlFlow::Return(Value::Null))?
                } else { Value::Null };
                Err(ControlFlow::Return(val))
            }
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                if self.emit_llvm {
                    self.llvm_buffer.push_str(&val.to_string_val());
                     self.llvm_buffer.push('\n');
                } else {
                    println!("{}", val.to_string_val());
                }
                Ok(())
            }
            Stmt::If(cond, then_block, else_block) => {
                let cond_val = self.eval_expr(cond).map_err(|_| ControlFlow::Return(Value::Null))?;
                if cond_val.is_truthy() {
                    self.push_scope();
                    let res = self.exec_stmts(then_block);
                    let pop = self.pop_scope(); 
                    if res.is_err() { return res; }
                    if pop.is_err() { return pop; }
                    Ok(())
                } else if let Some(else_stmts) = else_block {
                    self.push_scope();
                    let res = self.exec_stmts(else_stmts);
                    let pop = self.pop_scope();
                    if res.is_err() { return res; }
                    if pop.is_err() { return pop; }
                    Ok(())
                } else { Ok(()) }
            }
            Stmt::While(cond, body) => {
                loop {
                    let cond_val = self.eval_expr(cond).map_err(|_| ControlFlow::Return(Value::Null))?;
                    if !cond_val.is_truthy() { break; }
                    
                    self.push_scope();
                    let res = self.exec_stmts(body);
                    let pop = self.pop_scope(); 
                    if let Err(e) = pop { return Err(e); }

                    match res {
                        Ok(()) => {},
                        Err(ControlFlow::Break) => break,
                        Err(ControlFlow::Continue) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            Stmt::Break => Err(ControlFlow::Break),
            Stmt::Continue => Err(ControlFlow::Continue),
            Stmt::Expr(expr) => {
                self.eval_expr(expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                Ok(())
            }
            Stmt::Block(stmts) => {
                self.push_scope();
                let res = self.exec_stmts(stmts);
                let pop = self.pop_scope();
                if res.is_err() { return res; }
                pop
            }
        }
    }
    
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Int(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::Identifier(name) => Ok(self.get_var(name)),
            Expr::BinOp(left, op, right) => {
                 let l = self.eval_expr(left)?;
                 let r = self.eval_expr(right)?;
                 self.eval_binop(l, op, r)
            },
            Expr::UnaryOp(op, inner) => {
                 let val = self.eval_expr(inner)?;
                 if op == "!" { Ok(Value::Bool(!val.is_truthy())) } 
                 else { Ok(Value::Int(-val.as_int())) }
            },
            Expr::Call(name, args) => {
                let arg_vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect::<Result<_,_>>()?;
                self.call_function(name, arg_vals)
            },
            Expr::MethodCall(obj, method, args) => {
                let obj_val = self.eval_expr(obj)?;
                let mut arg_vals = vec![obj_val.clone()];
                for a in args { arg_vals.push(self.eval_expr(a)?); }
                let type_name = match &obj_val {
                    Value::Struct(name, _) => name.clone(),
                     Value::Array(_) => "Array".to_string(),
                     Value::String(_) => "string".to_string(),
                     Value::Int(_) => "i32".to_string(),
                    _ => "".to_string(),
                };
                if !type_name.is_empty() {
                    if let Some(func) = self.methods.get(&(type_name, method.clone())) {
                        return self.execute_function(func.clone(), arg_vals);
                    }
                }
                self.call_function(method, arg_vals)
            }
             Expr::Index(arr, idx) => {
                let arr_val = self.eval_expr(arr)?;
                let idx_val = self.eval_expr(idx)?.as_int() as usize;
                match arr_val {
                    Value::Array(arr) => Ok(arr.borrow().get(idx_val).cloned().unwrap_or(Value::Null)),
                    Value::String(s) => {
                        let c = s.chars().nth(idx_val).map(|c| c.to_string()).unwrap_or_default();
                        Ok(Value::String(c))
                    }
                     _ => Ok(Value::Null)
                }
            }
            Expr::Field(obj, field) => {
                let obj_val = self.eval_expr(obj)?;
                if let Value::Struct(_, fields) = obj_val {
                    Ok(fields.borrow().get(field).cloned().unwrap_or(Value::Null))
                } else if let Value::Array(arr) = obj_val {
                    // Array treated as tuple
                    let idx: usize = field.parse().unwrap_or(0);
                    Ok(arr.borrow().get(idx).cloned().unwrap_or(Value::Null))
                } else { Ok(Value::Null) }
            }
             Expr::Array(elements) => {
                let vals: Vec<Value> = elements.iter().map(|e| self.eval_expr(e)).collect::<Result<_, _>>()?;
                Ok(Value::Array(Rc::new(RefCell::new(vals))))
            }
            Expr::StructInit(name, fields) => {
                let mut map = HashMap::new();
                for (k, v) in fields { map.insert(k.clone(), self.eval_expr(v)?); }
                Ok(Value::Struct(name.clone(), Rc::new(RefCell::new(map))))
            }
            Expr::Await(inner) => self.eval_expr(inner),
        }
    }
    
    fn eval_binop(&self, left: Value, op: &str, right: Value) -> Result<Value, String> {
        match op {
            "+" => {
                match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a+b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                    (Value::String(a), _) => Ok(Value::String(format!("{}{}", a, right.to_string_val()))),
                    (_, Value::String(b)) => Ok(Value::String(format!("{}{}", left.to_string_val(), b))),
                    _ => Ok(Value::Int(left.as_int() + right.as_int()))
                }
            },
            "*" => Ok(Value::Int(left.as_int() * right.as_int())),
            "-" => Ok(Value::Int(left.as_int() - right.as_int())),
            "/" => {
                 let r = right.as_int();
                 if r == 0 { Ok(Value::Int(0)) } else { Ok(Value::Int(left.as_int() / r)) }
            },
            "%" => {
                 let r = right.as_int();
                 if r == 0 { Ok(Value::Int(0)) } else { Ok(Value::Int(left.as_int() % r)) }
            },
             "==" => {
                match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
                    _ => Ok(Value::Bool(false)),
                }
             },
             "!=" => {
                match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
                    _ => Ok(Value::Bool(true)),
                }
             },
             "<" => Ok(Value::Bool(left.as_int() < right.as_int())),
             ">" => Ok(Value::Bool(left.as_int() > right.as_int())),
             "<=" => Ok(Value::Bool(left.as_int() <= right.as_int())),
             ">=" => Ok(Value::Bool(left.as_int() >= right.as_int())),
             "&&" => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
             "||" => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
            _ => Err(format!("Unknown operator: {}", op))
        }
    }
}
