// Argon Interpreter - Executes AST
// Compatible with compiler.ar v2.28.0 (GC + FFI)

#![allow(dead_code)]

use crate::parser::{Expr, Stmt, TopLevel, Function, Param, TraitDef};
use crate::ffi::FfiManager;
use crate::gc::GarbageCollector;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;
use std::cell::RefCell;
use std::net::{TcpListener, TcpStream};

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
    traits: HashMap<String, TraitDef>,
    trait_impls: HashMap<(String, String), bool>,
    loaded_modules: HashSet<String>,
    base_path: String,
    // Networking
    listeners: HashMap<i64, TcpListener>,
    sockets: HashMap<i64, TcpStream>,
    next_sock_id: i64,
    // FFI
    ffi: FfiManager,
    // GC
    gc: GarbageCollector,
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
            traits: HashMap::new(),
            trait_impls: HashMap::new(),
            loaded_modules: HashSet::new(),
            base_path: String::new(),
            listeners: HashMap::new(),
            sockets: HashMap::new(),
            next_sock_id: 1000,
            ffi: FfiManager::new(),
            gc: GarbageCollector::new(),
        }
    }
    
    pub fn set_base_path(&mut self, path: &str) {
        // Extract directory from file path
        if let Some(parent) = std::path::Path::new(path).parent() {
            self.base_path = parent.to_string_lossy().to_string();
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
        if let Some(func) = self.functions.get(name) {
            return Value::Function(func.name.clone(), func.params.clone(), func.body.clone());
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
    
    fn load_module(&mut self, path: &str) -> Result<(), String> {
        if self.loaded_modules.contains(path) { return Ok(()); }
        self.loaded_modules.insert(path.to_string());
        
        // Build search paths - include base_path for relative imports
        let mut possible_paths = vec![];
        
        // First priority: relative to main file's directory
        if !self.base_path.is_empty() {
            possible_paths.push(format!("{}/{}.ar", self.base_path, path));
        }
        
        // Standard paths
        possible_paths.push(format!("d:/rust/stdlib/{}.ar", path));
        possible_paths.push(format!("stdlib/{}.ar", path));
        possible_paths.push(format!("{}.ar", path));
        possible_paths.push(format!("examples/{}.ar", path));
        possible_paths.push(format!("libs/{}.ar", path));
        
        let mut source = String::new();
        let mut found = false;
        let mut used_path = String::new();
        
        for p in possible_paths {
            if std::path::Path::new(&p).exists() {
                source = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
                found = true;
                used_path = p;
                break;
            }
        }
        
        if !found { return Err(format!("Module not found: {}", path)); }
        
        if self.loaded_modules.contains(&used_path) {
             return Ok(());
        }
        self.loaded_modules.insert(used_path.clone());
        
        // Run Pipeline: Lexer -> Parser -> Expander -> Optimizer -> Interpreter
        let tokens = crate::lexer::tokenize(&source);
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse()?;
        
        let mut expander = crate::expander::Expander::new();
        let expanded = expander.expand(ast);
        
        let optimizer = crate::optimizer::Optimizer::new();
        let final_ast = optimizer.optimize(expanded);
        
        self.run(&final_ast)?;
        Ok(())
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
                    // Register trait implementation
                    if !impl_def.trait_name.is_empty() {
                        self.trait_impls.insert((impl_def.type_name.clone(), impl_def.trait_name.clone()), true);
                    }
                }
                TopLevel::Import(path, _) => {
                    self.load_module(path)?;
                }
                TopLevel::Macro(_) => {} // Macros already expanded
                TopLevel::Struct(_) | TopLevel::Enum(_) | TopLevel::Extern(_) => {}
                TopLevel::Trait(trait_def) => {
                    self.traits.insert(trait_def.name.clone(), trait_def.clone());
                }
            }
        }
        
        if self.functions.contains_key("main") {
            // Heuristic to prevent running main recursively? 
            // For now, assume modules don't have main.
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
            "argon_listen" => {
                if let Some(Value::Int(port)) = args.first() {
                     if let Ok(listener) = TcpListener::bind(format!("0.0.0.0:{}", port)) {
                         let id = self.next_sock_id;
                         self.next_sock_id += 1;
                         self.listeners.insert(id, listener);
                         return Ok(Value::Int(id));
                     }
                }
                return Ok(Value::Int(-1));
            }
            "argon_accept" => {
                if let Some(Value::Int(id)) = args.first() {
                    if let Some(listener) = self.listeners.get(id) {
                         if let Ok((stream, _)) = listener.accept() {
                             let client_id = self.next_sock_id;
                             self.next_sock_id += 1;
                             self.sockets.insert(client_id, stream);
                             return Ok(Value::Int(client_id));
                         }
                    }
                }
                return Ok(Value::Int(-1));
            }
            "argon_socket_read" => {
                if let Some(Value::Int(id)) = args.first() {
                    if let Some(stream) = self.sockets.get_mut(id) {
                        let mut buf = [0; 2048];
                        if let Ok(n) = stream.read(&mut buf) {
                            let s = String::from_utf8_lossy(&buf[..n]).to_string();
                            return Ok(Value::String(s));
                        }
                    }
                }
                return Ok(Value::String("".to_string()));
            }
            "argon_socket_write" => {
                 if args.len() >= 2 {
                     if let (Value::Int(id), Value::String(s)) = (&args[0], &args[1]) {
                         if let Some(stream) = self.sockets.get_mut(id) {
                             let _ = stream.write_all(s.as_bytes());
                         }
                     }
                 }
                 return Ok(Value::Null);
            }
            "argon_socket_close" => {
                if let Some(Value::Int(id)) = args.first() {
                    self.sockets.remove(id);
                    self.listeners.remove(id); 
                }
                return Ok(Value::Null);
            }
            "sleep" => {
                if let Some(Value::Int(ms)) = args.first() {
                    std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
                }
                return Ok(Value::Null);
            }
            "env" => {
                if let Some(Value::String(key)) = args.first() {
                    match std::env::var(key) {
                        Ok(val) => return Ok(Value::String(val)),
                        Err(_) => {
                            if args.len() > 1 {
                                return Ok(args[1].clone());
                            }
                            return Ok(Value::Null);
                        }
                    }
                }
                return Ok(Value::Null);
            }
            // ============================================
            // Crypto Built-ins (simplified for demo)
            // ============================================
            "bcrypt_hash" => {
                if let Some(Value::String(password)) = args.first() {
                    // Simplified hash: in production use actual bcrypt
                    let hash = format!("$2b$12${}", base64_simple(password));
                    return Ok(Value::String(hash));
                }
                return Ok(Value::Null);
            }
            "bcrypt_verify" => {
                if args.len() >= 2 {
                    if let (Value::String(password), Value::String(hash)) = (&args[0], &args[1]) {
                        // Simplified verify
                        let expected = format!("$2b$12${}", base64_simple(password));
                        return Ok(Value::Bool(&expected == hash));
                    }
                }
                return Ok(Value::Bool(false));
            }
            "jwt_sign" => {
                // jwt_sign(payload_json, secret) -> token string
                if args.len() >= 2 {
                    if let (Value::String(payload), Value::String(secret)) = (&args[0], &args[1]) {
                        let header = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"; // fixed header
                        let payload_b64 = base64_simple(payload);
                        let signature = base64_simple(&format!("{}.{}.{}", header, payload_b64, secret));
                        let token = format!("{}.{}.{}", header, payload_b64, signature);
                        return Ok(Value::String(token));
                    }
                }
                return Ok(Value::Null);
            }
            "jwt_verify" => {
                // jwt_verify(token, secret) -> payload string or null
                if args.len() >= 2 {
                    if let (Value::String(token), Value::String(_secret)) = (&args[0], &args[1]) {
                        let parts: Vec<&str> = token.split('.').collect();
                        if parts.len() == 3 {
                            // Simplified: just return payload without actual verification
                            if let Some(payload) = base64_decode_simple(parts[1]) {
                                return Ok(Value::String(payload));
                            }
                        }
                    }
                }
                return Ok(Value::Null);
            }
            "timestamp" | "now" => {
                use std::time::{SystemTime, UNIX_EPOCH};
                let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
                return Ok(Value::Int(duration.as_secs() as i64));
            }
            "timestamp_ms" => {
                use std::time::{SystemTime, UNIX_EPOCH};
                let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
                return Ok(Value::Int(duration.as_millis() as i64));
            }
            "date_now" => {
                // Returns ISO date string
                use std::time::{SystemTime, UNIX_EPOCH};
                let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
                // Simple date formatting (approximate)
                let days = secs / 86400;
                let years = 1970 + (days / 365);
                let day_of_year = days % 365;
                let month = (day_of_year / 30) + 1;
                let day = (day_of_year % 30) + 1;
                let date = format!("{:04}-{:02}-{:02}", years, month.min(12), day.min(31));
                return Ok(Value::String(date));
            }
            "uuid" | "generate_id" => {
                // Simple pseudo-random ID
                use std::time::{SystemTime, UNIX_EPOCH};
                let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
                let id = format!("{:x}-{:x}-{:x}", ts as u32, (ts >> 32) as u32, (ts >> 64) as u32);
                return Ok(Value::String(id));
            }
            "rand" | "random" => {
                // Simple pseudo-random number
                use std::time::{SystemTime, UNIX_EPOCH};
                let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
                return Ok(Value::Int((ts % 1000000) as i64));
            }
            // ============================================
            // Math Built-ins
            // ============================================
            "abs" => {
                if let Some(Value::Int(n)) = args.first() {
                    return Ok(Value::Int(n.abs()));
                }
                return Ok(Value::Int(0));
            }
            "max" => {
                if args.len() >= 2 {
                    if let (Value::Int(a), Value::Int(b)) = (&args[0], &args[1]) {
                        return Ok(Value::Int((*a).max(*b)));
                    }
                }
                return Ok(Value::Int(0));
            }
            "min" => {
                if args.len() >= 2 {
                    if let (Value::Int(a), Value::Int(b)) = (&args[0], &args[1]) {
                        return Ok(Value::Int((*a).min(*b)));
                    }
                }
                return Ok(Value::Int(0));
            }
            "rand_int" => {
                if args.len() >= 2 {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    if let (Value::Int(min_val), Value::Int(max_val)) = (&args[0], &args[1]) {
                        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
                        let range = (max_val - min_val + 1) as u128;
                        let result = min_val + (ts % range) as i64;
                        return Ok(Value::Int(result));
                    }
                }
                return Ok(Value::Int(0));
            }
            // ============================================
            // String Built-ins
            // ============================================
            "split" => {
                if args.len() >= 2 {
                    if let (Value::String(s), Value::String(delim)) = (&args[0], &args[1]) {
                        let parts: Vec<Value> = s.split(delim.as_str())
                            .map(|p| Value::String(p.to_string()))
                            .collect();
                        return Ok(Value::Array(Rc::new(RefCell::new(parts))));
                    }
                }
                return Ok(Value::Array(Rc::new(RefCell::new(vec![]))));
            }
            "join" => {
                if args.len() >= 2 {
                    if let (Value::Array(arr), Value::String(delim)) = (&args[0], &args[1]) {
                        let parts: Vec<String> = arr.borrow().iter()
                            .map(|v| v.to_string_val())
                            .collect();
                        return Ok(Value::String(parts.join(delim)));
                    }
                }
                return Ok(Value::String(String::new()));
            }
            "trim" => {
                if let Some(Value::String(s)) = args.first() {
                    return Ok(Value::String(s.trim().to_string()));
                }
                return Ok(Value::String(String::new()));
            }
            "to_upper" | "toUpperCase" | "upper" => {
                if let Some(Value::String(s)) = args.first() {
                    return Ok(Value::String(s.to_uppercase()));
                }
                return Ok(Value::String(String::new()));
            }
            "to_lower" | "toLowerCase" | "lower" => {
                if let Some(Value::String(s)) = args.first() {
                    return Ok(Value::String(s.to_lowercase()));
                }
                return Ok(Value::String(String::new()));
            }
            "contains" => {
                if args.len() >= 2 {
                    if let (Value::String(s), Value::String(sub)) = (&args[0], &args[1]) {
                        return Ok(Value::Bool(s.contains(sub.as_str())));
                    }
                    if let (Value::Array(arr), val) = (&args[0], &args[1]) {
                        let found = arr.borrow().iter().any(|v| v.to_string_val() == val.to_string_val());
                        return Ok(Value::Bool(found));
                    }
                }
                return Ok(Value::Bool(false));
            }
            "starts_with" | "startsWith" => {
                if args.len() >= 2 {
                    if let (Value::String(s), Value::String(prefix)) = (&args[0], &args[1]) {
                        return Ok(Value::Bool(s.starts_with(prefix.as_str())));
                    }
                }
                return Ok(Value::Bool(false));
            }
            "ends_with" | "endsWith" => {
                if args.len() >= 2 {
                    if let (Value::String(s), Value::String(suffix)) = (&args[0], &args[1]) {
                        return Ok(Value::Bool(s.ends_with(suffix.as_str())));
                    }
                }
                return Ok(Value::Bool(false));
            }
            "replace" => {
                if args.len() >= 3 {
                    if let (Value::String(s), Value::String(from), Value::String(to)) = 
                        (&args[0], &args[1], &args[2]) 
                    {
                        return Ok(Value::String(s.replace(from.as_str(), to.as_str())));
                    }
                }
                return Ok(Value::String(String::new()));
            }
            "char_at" | "charAt" => {
                if args.len() >= 2 {
                    if let (Value::String(s), Value::Int(idx)) = (&args[0], &args[1]) {
                        if let Some(c) = s.chars().nth(*idx as usize) {
                            return Ok(Value::String(c.to_string()));
                        }
                    }
                }
                return Ok(Value::String(String::new()));
            }
            "index_of" | "indexOf" => {
                if args.len() >= 2 {
                    if let (Value::String(s), Value::String(sub)) = (&args[0], &args[1]) {
                        if let Some(idx) = s.find(sub.as_str()) {
                            return Ok(Value::Int(idx as i64));
                        }
                        return Ok(Value::Int(-1));
                    }
                }
                return Ok(Value::Int(-1));
            }
            "repeat" => {
                if args.len() >= 2 {
                    if let (Value::String(s), Value::Int(n)) = (&args[0], &args[1]) {
                        return Ok(Value::String(s.repeat(*n as usize)));
                    }
                }
                return Ok(Value::String(String::new()));
            }
            // ============================================
            // Array Built-ins
            // ============================================
            "pop" => {
                if let Some(Value::Array(arr)) = args.first() {
                    if let Some(val) = arr.borrow_mut().pop() {
                        return Ok(val);
                    }
                }
                return Ok(Value::Null);
            }
            "shift" => {
                if let Some(Value::Array(arr)) = args.first() {
                    if !arr.borrow().is_empty() {
                        let val = arr.borrow_mut().remove(0);
                        return Ok(val);
                    }
                }
                return Ok(Value::Null);
            }
            "reverse" => {
                if let Some(Value::Array(arr)) = args.first() {
                    arr.borrow_mut().reverse();
                    return Ok(args[0].clone());
                }
                if let Some(Value::String(s)) = args.first() {
                    return Ok(Value::String(s.chars().rev().collect()));
                }
                return Ok(Value::Null);
            }
            "sort" => {
                if let Some(Value::Array(arr)) = args.first() {
                    arr.borrow_mut().sort_by(|a, b| {
                        a.to_string_val().cmp(&b.to_string_val())
                    });
                    return Ok(args[0].clone());
                }
                return Ok(Value::Null);
            }
            "slice" => {
                if args.len() >= 2 {
                    if let (Value::Array(arr), Value::Int(start)) = (&args[0], &args[1]) {
                        let start = *start as usize;
                        let end = if args.len() > 2 {
                            if let Value::Int(e) = &args[2] { *e as usize } else { arr.borrow().len() }
                        } else {
                            arr.borrow().len()
                        };
                        let sliced: Vec<Value> = arr.borrow().iter()
                            .skip(start)
                            .take(end.saturating_sub(start))
                            .cloned()
                            .collect();
                        return Ok(Value::Array(Rc::new(RefCell::new(sliced))));
                    }
                }
                return Ok(Value::Array(Rc::new(RefCell::new(vec![]))));
            }
            "range" => {
                if args.len() >= 2 {
                    if let (Value::Int(start), Value::Int(end)) = (&args[0], &args[1]) {
                        let step = if args.len() > 2 {
                            if let Value::Int(s) = &args[2] { *s } else { 1 }
                        } else { 1 };
                        let mut result = vec![];
                        let mut i = *start;
                        while i < *end {
                            result.push(Value::Int(i));
                            i += step;
                        }
                        return Ok(Value::Array(Rc::new(RefCell::new(result))));
                    }
                }
                return Ok(Value::Array(Rc::new(RefCell::new(vec![]))));
            }
            "find_index" | "findIndex" => {
                if args.len() >= 2 {
                    if let (Value::Array(arr), val) = (&args[0], &args[1]) {
                        for (i, v) in arr.borrow().iter().enumerate() {
                            if v.to_string_val() == val.to_string_val() {
                                return Ok(Value::Int(i as i64));
                            }
                        }
                    }
                }
                return Ok(Value::Int(-1));
            }
            // ============================================
            // Type Built-ins
            // ============================================
            "typeof" | "type_of" | "type" => {
                if let Some(val) = args.first() {
                    let type_name = match val {
                        Value::Null => "null",
                        Value::Int(_) => "int",
                        Value::Bool(_) => "bool",
                        Value::String(_) => "string",
                        Value::Array(_) => "array",
                        Value::Struct(_, _) => "struct",
                        Value::Function(_, _, _) => "function",
                    };
                    return Ok(Value::String(type_name.to_string()));
                }
                return Ok(Value::String("unknown".to_string()));
            }
            "is_null" | "isNull" => {
                if let Some(val) = args.first() {
                    return Ok(Value::Bool(matches!(val, Value::Null)));
                }
                return Ok(Value::Bool(true));
            }
            "is_array" | "isArray" => {
                if let Some(val) = args.first() {
                    return Ok(Value::Bool(matches!(val, Value::Array(_))));
                }
                return Ok(Value::Bool(false));
            }
            "is_string" | "isString" => {
                if let Some(val) = args.first() {
                    return Ok(Value::Bool(matches!(val, Value::String(_))));
                }
                return Ok(Value::Bool(false));
            }
            "is_int" | "isInt" | "is_number" | "isNumber" => {
                if let Some(val) = args.first() {
                    return Ok(Value::Bool(matches!(val, Value::Int(_))));
                }
                return Ok(Value::Bool(false));
            }
            // ============================================
            // Conversion Built-ins
            // ============================================
            "int" | "to_int" | "toInt" => {
                if let Some(val) = args.first() {
                    match val {
                        Value::Int(n) => return Ok(Value::Int(*n)),
                        Value::String(s) => return Ok(Value::Int(s.parse().unwrap_or(0))),
                        Value::Bool(b) => return Ok(Value::Int(if *b { 1 } else { 0 })),
                        _ => return Ok(Value::Int(0)),
                    }
                }
                return Ok(Value::Int(0));
            }
            "str" | "to_string" => {
                if let Some(val) = args.first() {
                    return Ok(Value::String(val.to_string_val()));
                }
                return Ok(Value::String(String::new()));
            }
            // ============================================
            // Console/Debug Built-ins
            // ============================================
            "debug" => {
                if let Some(val) = args.first() {
                    println!("[DEBUG] {:?}", val);
                }
                return Ok(Value::Null);
            }
            "assert" => {
                if let Some(Value::Bool(b)) = args.first() {
                    if !b {
                        let msg = if args.len() > 1 {
                            args[1].to_string_val()
                        } else {
                            "Assertion failed".to_string()
                        };
                        return Err(format!("Assertion Error: {}", msg));
                    }
                }
                return Ok(Value::Null);
            }
            "exit" => {
                let code = if let Some(Value::Int(n)) = args.first() {
                    *n as i32
                } else { 0 };
                std::process::exit(code);
            }
            "make_token" | "make_binop" | "make_unary" | "make_call" | 
            "make_if" | "make_while" | "make_func" | "make_return" | "make_let" | 
            "make_assign" | "make_block" | "make_print" | "make_ast_num" | 
            "make_ast_str" | "make_ast_id" | "make_ast_array" | "make_struct_def" |
            "make_struct_init" | "make_enum_def" | "make_match" | "make_index" => {
                return Ok(Value::Array(Rc::new(RefCell::new(args))));
            }
            // ============================================
            // FFI Built-ins
            // ============================================
            "ffi_load" => {
                // ffi_load("libname") - Load a dynamic library
                if let Some(Value::String(lib_name)) = args.first() {
                    match self.ffi.load_library(lib_name) {
                        Ok(()) => return Ok(Value::Bool(true)),
                        Err(e) => {
                            eprintln!("FFI Load Error: {}", e);
                            return Ok(Value::Bool(false));
                        }
                    }
                }
                return Ok(Value::Bool(false));
            }
            "ffi_call" => {
                // ffi_call("libname", "funcname", [arg1, arg2, ...]) - Call a function
                if args.len() >= 2 {
                    if let (Value::String(lib_name), Value::String(func_name)) = (&args[0], &args[1]) {
                        let call_args: Vec<i64> = if args.len() > 2 {
                            if let Value::Array(arr) = &args[2] {
                                arr.borrow().iter().map(|v| {
                                    match v {
                                        Value::Int(n) => *n,
                                        _ => 0,
                                    }
                                }).collect()
                            } else {
                                vec![]
                            }
                        } else {
                            vec![]
                        };
                        
                        match self.ffi.call_i64(lib_name, func_name, &call_args) {
                            Ok(result) => return Ok(Value::Int(result)),
                            Err(e) => {
                                eprintln!("FFI Call Error: {}", e);
                                return Ok(Value::Null);
                            }
                        }
                    }
                }
                return Ok(Value::Null);
            }
            // ============================================
            // GC Built-ins
            // ============================================
            "gc_collect" => {
                // Force garbage collection
                self.gc.collect();
                return Ok(Value::Null);
            }
            "gc_stats" => {
                // Return heap statistics [heap_size, allocated_since_last_gc]
                let (heap_size, allocated) = self.gc.stats();
                let stats = vec![
                    Value::Int(heap_size as i64),
                    Value::Int(allocated as i64),
                ];
                return Ok(Value::Array(Rc::new(RefCell::new(stats))));
            }
            _ => {}
        }
        
        let func = if let Some(f) = self.functions.get(name) {
            f.clone()
        } else {
            // Check if variable is a function
            match self.get_var(name) {
                Value::Function(n, p, b) => Function { name: n, params: p, body: b, is_async: false, return_type: None },
                _ => return Err(format!("Undefined function: {}", name)),
            }
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
             (Err(_e), _) => Ok(Value::Null), // Other control flows invalid in function
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
                let val = self.eval_expr(expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
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
                let val = self.eval_expr(expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
                self.set_var(name, val);
                Ok(())
            }
            Stmt::IndexAssign(arr_expr, idx_expr, val_expr) => {
                let arr_val = self.eval_expr(arr_expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
                let idx_val = self.eval_expr(idx_expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
                let val = self.eval_expr(val_expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
                
                match arr_val {
                    Value::Array(arr) => {
                        let idx = idx_val.as_int() as usize;
                        let mut vec = arr.borrow_mut();
                        if idx < vec.len() { 
                            vec[idx] = val; 
                        } else {
                            // Extend array if needed
                            while vec.len() <= idx {
                                vec.push(Value::Null);
                            }
                            vec[idx] = val;
                        }
                    }
                    Value::Struct(_, fields) => {
                        let key = idx_val.to_string_val();
                        fields.borrow_mut().insert(key, val);
                    }
                    _ => {}
                }
                Ok(())
            }
            Stmt::FieldAssign(obj_expr, field, val_expr) => {
                let obj_val = self.eval_expr(obj_expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
                let val = self.eval_expr(val_expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
                if let Value::Struct(_, fields) = obj_val {
                    fields.borrow_mut().insert(field.clone(), val);
                }
                Ok(())
            }
            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval_expr(e).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?
                } else { Value::Null };
                Err(ControlFlow::Return(val))
            }
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
                if self.emit_llvm {
                    self.llvm_buffer.push_str(&val.to_string_val());
                     self.llvm_buffer.push('\n');
                } else {
                    println!("{}", val.to_string_val());
                }
                Ok(())
            }
            Stmt::If(cond, then_block, else_block) => {
                let cond_val = self.eval_expr(cond).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
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
                    let cond_val = self.eval_expr(cond).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
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
                self.eval_expr(expr).map_err(|e| { println!("Runtime Error: {}", e); ControlFlow::Return(Value::Null) })?;
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
                     if let Some(func) = self.methods.get(&(type_name.clone(), method.clone())) {
                        return self.execute_function(func.clone(), arg_vals);
                    }
                }
                // Try global function? No, methods are specific.
                Err(format!("Undefined method: '{}' on type '{}'", method, type_name))
            },
            Expr::StaticMethodCall(type_name, method, args) => {
                 let arg_vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect::<Result<_,_>>()?;
                 if let Some(func) = self.methods.get(&(type_name.clone(), method.clone())) {
                      return self.execute_function(func.clone(), arg_vals);
                 }
                 Err(format!("Undefined static method: '{}' on type '{}'", method, type_name))
            },
            Expr::Await(inner) => self.eval_expr(inner),
            Expr::StructInit(name, fields) => {
                let mut field_map = HashMap::new();
                for (fname, fexpr) in fields {
                    let val = self.eval_expr(fexpr)?;
                    field_map.insert(fname.clone(), val);
                }
                Ok(Value::Struct(name.clone(), Rc::new(RefCell::new(field_map))))
            },
            Expr::Array(elems) => {
                let vals: Vec<Value> = elems.iter().map(|e| self.eval_expr(e)).collect::<Result<_,_>>()?;
                Ok(Value::Array(Rc::new(RefCell::new(vals))))
            },
            Expr::Index(arr_expr, idx_expr) => {
                let arr_val = self.eval_expr(arr_expr)?;
                let idx_val = self.eval_expr(idx_expr)?;
                match arr_val {
                    Value::Array(arr) => {
                        let idx = idx_val.as_int() as usize;
                        Ok(arr.borrow().get(idx).cloned().unwrap_or(Value::Null))
                    },
                    Value::Struct(_, fields) => {
                        let key = idx_val.to_string_val();
                        Ok(fields.borrow().get(&key).cloned().unwrap_or(Value::Null))
                    },
                    Value::String(s) => {
                         let idx = idx_val.as_int() as usize;
                         Ok(Value::String(s.chars().nth(idx).map(|c| c.to_string()).unwrap_or_default()))
                    },
                    _ => Ok(Value::Null),
                }
            },
            Expr::Field(obj_expr, field) => {
                let obj_val = self.eval_expr(obj_expr)?;
                if let Value::Struct(_, fields) = obj_val {
                     let f = fields.borrow();
                     if let Some(val) = f.get(field) {
                        Ok(val.clone())
                     } else {
                         println!("Runtime Error: Missing field '{}'. Available: {:?}", field, f.keys().collect::<Vec<_>>());
                         Ok(Value::Null)
                     }
                } else if let Value::Array(arr) = obj_val {
                     if let Ok(idx) = field.parse::<usize>() {
                         Ok(arr.borrow().get(idx).cloned().unwrap_or(Value::Null))
                     } else { Ok(Value::Null) }
                } else { Ok(Value::Null) }
            },
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

// Helper functions for crypto
fn base64_simple(s: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = s.as_bytes();
    let mut result = String::new();
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }
        result.push(CHARS[(buf[0] >> 2) as usize] as char);
        result.push(CHARS[(((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[(((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize] as char);
        }
        if chunk.len() > 2 {
            result.push(CHARS[(buf[2] & 0x3f) as usize] as char);
        }
    }
    result
}

fn base64_decode_simple(s: &str) -> Option<String> {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();
    let bytes: Vec<u8> = s.bytes().filter_map(|b| {
        CHARS.iter().position(|&c| c == b).map(|p| p as u8)
    }).collect();
    
    for chunk in bytes.chunks(4) {
        if chunk.len() >= 2 {
            result.push((chunk[0] << 2) | (chunk[1] >> 4));
        }
        if chunk.len() >= 3 {
            result.push((chunk[1] << 4) | (chunk[2] >> 2));
        }
        if chunk.len() >= 4 {
            result.push((chunk[2] << 6) | chunk[3]);
        }
    }
    
    String::from_utf8(result).ok()
}
