// Argon Interpreter - Executes AST
// Compatible with compiler.ar v2.19.0

#![allow(dead_code)]

use crate::parser::{Expr, Stmt, TopLevel, Function, Param};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
    Array(Vec<Value>),
    Struct(String, HashMap<String, Value>),
    Function(String, Vec<Param>, Vec<Stmt>),
}

impl Value {
    pub fn to_string_val(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_val()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Struct(name, fields) => {
                let items: Vec<String> = fields.iter()
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
            Value::Array(arr) => !arr.is_empty(),
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

pub struct Interpreter {
    globals: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    stack: Vec<HashMap<String, Value>>,
    emit_llvm: bool,
    llvm_output: String,
    llvm_buffer: String,
    program_args: Vec<String>,
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
            stack: vec![HashMap::new()],
            emit_llvm: false,
            llvm_output: String::new(),
            llvm_buffer: String::new(),
            program_args: Vec::new(),
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
        // Check local scopes first (from innermost to outermost)
        for scope in self.stack.iter().rev() {
            if let Some(val) = scope.get(name) {
                return val.clone();
            }
        }
        // Check globals
        if let Some(val) = self.globals.get(name) {
            return val.clone();
        }
        Value::Null
    }
    
    fn set_var(&mut self, name: &str, val: Value) {
        // Check if variable exists in any scope
        for scope in self.stack.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return;
            }
        }
        if self.globals.contains_key(name) {
            self.globals.insert(name.to_string(), val);
            return;
        }
        // New variable in current scope
        if let Some(scope) = self.stack.last_mut() {
            scope.insert(name.to_string(), val);
        }
    }
    
    fn declare_var(&mut self, name: &str, val: Value) {
        if let Some(scope) = self.stack.last_mut() {
            scope.insert(name.to_string(), val);
        }
    }
    
    fn push_scope(&mut self) {
        self.stack.push(HashMap::new());
    }
    
    fn pop_scope(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }
    
    pub fn run(&mut self, ast: &[TopLevel]) -> Result<Value, String> {
        // First pass: collect functions and global variables
        for item in ast {
            match item {
                TopLevel::Function(f) => {
                    self.functions.insert(f.name.clone(), f.clone());
                }
                TopLevel::Let(name, expr) => {
                    let val = self.eval_expr(expr)?;
                    self.globals.insert(name.clone(), val);
                }
                _ => {}
            }
        }
        
        // Call main if it exists
        if self.functions.contains_key("main") {
            return self.call_function("main", vec![]);
        }
        
        Ok(Value::Null)
    }
    
    fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        // Built-in functions
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
                        Value::Array(arr) => return Ok(Value::Int(arr.len() as i64)),
                        _ => return Ok(Value::Int(0)),
                    }
                }
                return Ok(Value::Int(0));
            }
            "push" => {
                if args.len() >= 2 {
                    if let Value::Array(mut arr) = args[0].clone() {
                        arr.push(args[1].clone());
                        return Ok(Value::Array(arr));
                    }
                }
                return Ok(args.first().cloned().unwrap_or(Value::Null));
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
            "get_args" | "getArgs" | "argon_get_args" => {
                let arg_vals: Vec<Value> = self.program_args.iter()
                    .map(|s| Value::String(s.clone()))
                    .collect();
                return Ok(Value::Array(arg_vals));
            }
            "make_token" | "make_binop" | "make_unary" | "make_call" | 
            "make_if" | "make_while" | "make_func" | "make_return" | "make_let" | 
            "make_assign" | "make_block" | "make_print" | "make_ast_num" | 
            "make_ast_str" | "make_ast_id" | "make_ast_array" | "make_struct_def" |
            "make_struct_init" | "make_enum_def" | "make_match" | "make_index" => {
                // AST construction functions - return array
                return Ok(Value::Array(args));
            }
            _ => {}
        }
        
        // User-defined function
        let func = match self.functions.get(name) {
            Some(f) => f.clone(),
            None => return Err(format!("Undefined function: {}", name)),
        };
        
        self.push_scope();
        
        // Bind parameters
        for (i, param) in func.params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or(Value::Null);
            self.declare_var(&param.name, val);
        }
        
        // Execute body
        let result = self.exec_stmts(&func.body);
        
        self.pop_scope();
        
        match result {
            Ok(_) => Ok(Value::Null),
            Err(ControlFlow::Return(val)) => Ok(val),
            Err(ControlFlow::Break) => Ok(Value::Null),
            Err(ControlFlow::Continue) => Ok(Value::Null),
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
            Stmt::Assign(name, expr) => {
                let val = self.eval_expr(expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                self.set_var(name, val);
                Ok(())
            }
            Stmt::IndexAssign(arr_expr, idx_expr, val_expr) => {
                let idx = self.eval_expr(idx_expr).map_err(|_| ControlFlow::Return(Value::Null))?.as_int() as usize;
                let val = self.eval_expr(val_expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                if let Expr::Identifier(name) = arr_expr {
                    if let Value::Array(mut arr) = self.get_var(name) {
                        if idx < arr.len() {
                            arr[idx] = val;
                            self.set_var(name, Value::Array(arr));
                        }
                    }
                }
                Ok(())
            }
            Stmt::FieldAssign(obj_expr, field, val_expr) => {
                let val = self.eval_expr(val_expr).map_err(|_| ControlFlow::Return(Value::Null))?;
                if let Expr::Identifier(name) = obj_expr {
                    if let Value::Struct(sname, mut fields) = self.get_var(name) {
                        fields.insert(field.clone(), val);
                        self.set_var(name, Value::Struct(sname, fields));
                    }
                }
                Ok(())
            }
            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval_expr(e).map_err(|_| ControlFlow::Return(Value::Null))?
                } else {
                    Value::Null
                };
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
                    let result = self.exec_stmts(then_block);
                    self.pop_scope();
                    result?;
                } else if let Some(else_stmts) = else_block {
                    self.push_scope();
                    let result = self.exec_stmts(else_stmts);
                    self.pop_scope();
                    result?;
                }
                Ok(())
            }
            Stmt::While(cond, body) => {
                loop {
                    let cond_val = self.eval_expr(cond).map_err(|_| ControlFlow::Return(Value::Null))?;
                    if !cond_val.is_truthy() {
                        break;
                    }
                    self.push_scope();
                    match self.exec_stmts(body) {
                        Ok(()) => {}
                        Err(ControlFlow::Break) => {
                            self.pop_scope();
                            break;
                        }
                        Err(ControlFlow::Continue) => {
                            self.pop_scope();
                            continue;
                        }
                        Err(e) => {
                            self.pop_scope();
                            return Err(e);
                        }
                    }
                    self.pop_scope();
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
                let result = self.exec_stmts(stmts);
                self.pop_scope();
                result
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
            }
            Expr::UnaryOp(op, expr) => {
                let val = self.eval_expr(expr)?;
                match op.as_str() {
                    "!" => Ok(Value::Bool(!val.is_truthy())),
                    "-" => Ok(Value::Int(-val.as_int())),
                    _ => Ok(val),
                }
            }
            Expr::Call(name, args) => {
                let arg_vals: Vec<Value> = args.iter()
                    .map(|a| self.eval_expr(a))
                    .collect::<Result<Vec<_>, _>>()?;
                self.call_function(name, arg_vals)
            }
            Expr::MethodCall(obj, method, args) => {
                let obj_val = self.eval_expr(obj)?;
                let mut arg_vals: Vec<Value> = vec![obj_val];
                for a in args {
                    arg_vals.push(self.eval_expr(a)?);
                }
                self.call_function(method, arg_vals)
            }
            Expr::Index(arr, idx) => {
                let arr_val = self.eval_expr(arr)?;
                let idx_val = self.eval_expr(idx)?.as_int() as usize;
                match arr_val {
                    Value::Array(arr) => Ok(arr.get(idx_val).cloned().unwrap_or(Value::Null)),
                    Value::String(s) => {
                        let c = s.chars().nth(idx_val).map(|c| c.to_string()).unwrap_or_default();
                        Ok(Value::String(c))
                    }
                    _ => Ok(Value::Null),
                }
            }
            Expr::Field(obj, field) => {
                let obj_val = self.eval_expr(obj)?;
                if let Value::Struct(_, fields) = obj_val {
                    Ok(fields.get(field).cloned().unwrap_or(Value::Null))
                } else if let Value::Array(arr) = obj_val {
                    // Array treated as tuple - field as index
                    let idx: usize = field.parse().unwrap_or(0);
                    Ok(arr.get(idx).cloned().unwrap_or(Value::Null))
                } else {
                    Ok(Value::Null)
                }
            }
            Expr::Array(elements) => {
                let vals: Vec<Value> = elements.iter()
                    .map(|e| self.eval_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Value::Array(vals))
            }
            Expr::StructInit(name, fields) => {
                let mut field_map = HashMap::new();
                for (fname, fexpr) in fields {
                    let val = self.eval_expr(fexpr)?;
                    field_map.insert(fname.clone(), val);
                }
                Ok(Value::Struct(name.clone(), field_map))
            }
            Expr::Await(inner) => {
                // For now, just evaluate the inner expression
                self.eval_expr(inner)
            }
        }
    }
    
    fn eval_binop(&self, left: Value, op: &str, right: Value) -> Result<Value, String> {
        match op {
            "+" => {
                match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                    (Value::String(a), Value::Int(b)) => Ok(Value::String(format!("{}{}", a, b))),
                    (Value::Int(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                    _ => Ok(Value::Int(left.as_int() + right.as_int())),
                }
            }
            "-" => Ok(Value::Int(left.as_int() - right.as_int())),
            "*" => Ok(Value::Int(left.as_int() * right.as_int())),
            "/" => {
                let r = right.as_int();
                if r == 0 { Ok(Value::Int(0)) } else { Ok(Value::Int(left.as_int() / r)) }
            }
            "%" => {
                let r = right.as_int();
                if r == 0 { Ok(Value::Int(0)) } else { Ok(Value::Int(left.as_int() % r)) }
            }
            "==" => {
                match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
                    _ => Ok(Value::Bool(false)),
                }
            }
            "!=" => {
                match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
                    _ => Ok(Value::Bool(true)),
                }
            }
            "<" => Ok(Value::Bool(left.as_int() < right.as_int())),
            ">" => Ok(Value::Bool(left.as_int() > right.as_int())),
            "<=" => Ok(Value::Bool(left.as_int() <= right.as_int())),
            ">=" => Ok(Value::Bool(left.as_int() >= right.as_int())),
            "&&" => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            "||" => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
            _ => Err(format!("Unknown operator: {}", op)),
        }
    }
}
