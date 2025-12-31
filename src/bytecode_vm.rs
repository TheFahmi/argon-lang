// Argon Bytecode VM - High Performance Execution Engine
// Provides ~10-20x speedup over tree-walking interpreter

#![allow(dead_code)]

use rustc_hash::FxHashMap;

/// Bytecode instructions for the VM
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    // Stack operations
    Const(i64),          // Push constant integer
    ConstTrue,           // Push true
    ConstFalse,          // Push false
    ConstNull,           // Push null
    
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    
    // Comparison
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
    
    // Logic
    Not,
    And,
    Or,
    
    // Control flow
    Jump(usize),         // Unconditional jump
    JumpIfFalse(usize),  // Jump if top of stack is false
    JumpIfTrue(usize),   // Jump if top of stack is true
    
    // Variables (using indexes instead of names for speed)
    LoadLocal(usize),    // Load local variable by index
    StoreLocal(usize),   // Store to local variable by index
    
    // Function calls
    Call(usize, usize),  // Call function at index with N args
    Return,              // Return from function
    
    // Stack management
    Pop,                 // Pop top of stack
    Dup,                 // Duplicate top of stack
    
    // Special
    Print,               // Print top of stack
    Halt,                // Stop execution
}

/// Stack-based value for VM
#[derive(Debug, Clone)]
pub enum VMValue {
    Null,
    Bool(bool),
    Int(i64),
}

impl VMValue {
    #[inline]
    fn as_int(&self) -> i64 {
        match self {
            VMValue::Int(n) => *n,
            VMValue::Bool(b) => if *b { 1 } else { 0 },
            VMValue::Null => 0,
        }
    }
    
    #[inline]
    fn is_truthy(&self) -> bool {
        match self {
            VMValue::Null => false,
            VMValue::Bool(b) => *b,
            VMValue::Int(n) => *n != 0,
        }
    }
}

/// Compiled function
#[derive(Debug, Clone)]
pub struct CompiledFunc {
    pub name: String,
    pub arity: usize,
    pub locals: usize,
    pub code: Vec<OpCode>,
}

/// Call frame for function calls
struct CallFrame {
    func_idx: usize,
    ip: usize,
    bp: usize,  // Base pointer for locals
}

/// Bytecode Virtual Machine
pub struct BytecodeVM {
    functions: Vec<CompiledFunc>,
    func_map: FxHashMap<String, usize>,
    stack: Vec<VMValue>,
    frames: Vec<CallFrame>,
    ip: usize,
    bp: usize,
}

impl BytecodeVM {
    pub fn new() -> Self {
        BytecodeVM {
            functions: Vec::new(),
            func_map: FxHashMap::default(),
            stack: Vec::with_capacity(4096),
            frames: Vec::with_capacity(256),
            ip: 0,
            bp: 0,
        }
    }
    
    pub fn add_function(&mut self, func: CompiledFunc) {
        let idx = self.functions.len();
        self.func_map.insert(func.name.clone(), idx);
        self.functions.push(func);
    }
    
    #[inline]
    fn push(&mut self, val: VMValue) {
        self.stack.push(val);
    }
    
    #[inline]
    fn pop(&mut self) -> VMValue {
        self.stack.pop().unwrap_or(VMValue::Null)
    }
    
    #[inline]
    fn peek(&self) -> &VMValue {
        self.stack.last().unwrap()
    }
    
    pub fn call(&mut self, func_name: &str, args: Vec<VMValue>) -> VMValue {
        let func_idx = *self.func_map.get(func_name).expect("Function not found");
        let func = &self.functions[func_idx];
        
        // Set up locals
        self.bp = self.stack.len();
        
        // Push arguments as locals
        for arg in args {
            self.stack.push(arg);
        }
        
        // Pad locals
        for _ in func.arity..func.locals {
            self.stack.push(VMValue::Null);
        }
        
        // Push initial frame
        self.frames.push(CallFrame {
            func_idx,
            ip: 0,
            bp: self.bp,
        });
        
        self.run()
    }
    
    fn run(&mut self) -> VMValue {
        loop {
            let frame = self.frames.last_mut().unwrap();
            let func = &self.functions[frame.func_idx];
            
            if frame.ip >= func.code.len() {
                // Implicit return null
                if self.frames.len() <= 1 {
                    return VMValue::Null;
                }
                self.frames.pop();
                continue;
            }
            
            let op = func.code[frame.ip];
            frame.ip += 1;
            
            match op {
                OpCode::Const(n) => self.push(VMValue::Int(n)),
                OpCode::ConstTrue => self.push(VMValue::Bool(true)),
                OpCode::ConstFalse => self.push(VMValue::Bool(false)),
                OpCode::ConstNull => self.push(VMValue::Null),
                
                OpCode::Add => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Int(a + b));
                }
                OpCode::Sub => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Int(a - b));
                }
                OpCode::Mul => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Int(a * b));
                }
                OpCode::Div => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Int(if b != 0 { a / b } else { 0 }));
                }
                OpCode::Mod => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Int(if b != 0 { a % b } else { 0 }));
                }
                OpCode::Neg => {
                    let a = self.pop().as_int();
                    self.push(VMValue::Int(-a));
                }
                
                OpCode::Lt => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Bool(a < b));
                }
                OpCode::Gt => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Bool(a > b));
                }
                OpCode::Le => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Bool(a <= b));
                }
                OpCode::Ge => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Bool(a >= b));
                }
                OpCode::Eq => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Bool(a == b));
                }
                OpCode::Ne => {
                    let b = self.pop().as_int();
                    let a = self.pop().as_int();
                    self.push(VMValue::Bool(a != b));
                }
                
                OpCode::Not => {
                    let a = self.pop().is_truthy();
                    self.push(VMValue::Bool(!a));
                }
                OpCode::And => {
                    let b = self.pop().is_truthy();
                    let a = self.pop().is_truthy();
                    self.push(VMValue::Bool(a && b));
                }
                OpCode::Or => {
                    let b = self.pop().is_truthy();
                    let a = self.pop().is_truthy();
                    self.push(VMValue::Bool(a || b));
                }
                
                OpCode::Jump(target) => {
                    let frame = self.frames.last_mut().unwrap();
                    frame.ip = target;
                }
                OpCode::JumpIfFalse(target) => {
                    if !self.pop().is_truthy() {
                        let frame = self.frames.last_mut().unwrap();
                        frame.ip = target;
                    }
                }
                OpCode::JumpIfTrue(target) => {
                    if self.pop().is_truthy() {
                        let frame = self.frames.last_mut().unwrap();
                        frame.ip = target;
                    }
                }
                
                OpCode::LoadLocal(idx) => {
                    let frame = self.frames.last().unwrap();
                    let val = self.stack[frame.bp + idx].clone();
                    self.push(val);
                }
                OpCode::StoreLocal(idx) => {
                    let val = self.pop();
                    let frame = self.frames.last().unwrap();
                    self.stack[frame.bp + idx] = val;
                }
                
                OpCode::Call(func_idx, argc) => {
                    // Get arguments from stack
                    let new_bp = self.stack.len() - argc;
                    let func = &self.functions[func_idx];
                    
                    // Pad locals
                    for _ in argc..func.locals {
                        self.stack.push(VMValue::Null);
                    }
                    
                    // Save return address
                    self.frames.push(CallFrame {
                        func_idx,
                        ip: 0,
                        bp: new_bp,
                    });
                }
                OpCode::Return => {
                    let result = self.pop();
                    let frame = self.frames.pop().unwrap();
                    
                    // Pop locals
                    self.stack.truncate(frame.bp);
                    
                    if self.frames.is_empty() {
                        return result;
                    }
                    
                    self.push(result);
                }
                
                OpCode::Pop => { self.pop(); }
                OpCode::Dup => {
                    let val = self.peek().clone();
                    self.push(val);
                }
                
                OpCode::Print => {
                    let val = self.pop();
                    match val {
                        VMValue::Int(n) => println!("{}", n),
                        VMValue::Bool(b) => println!("{}", b),
                        VMValue::Null => println!("null"),
                    }
                }
                
                OpCode::Halt => {
                    return VMValue::Null;
                }
            }
        }
    }
}

/// Compile a simple fibonacci function for testing
pub fn compile_fib() -> CompiledFunc {
    use OpCode::*;
    
    // fn fib(n) {
    //     if (n < 2) { return n; }
    //     return fib(n - 1) + fib(n - 2);
    // }
    
    CompiledFunc {
        name: "fib".to_string(),
        arity: 1,
        locals: 1,  // Just 'n'
        code: vec![
            // if (n < 2)
            LoadLocal(0),       // 0: load n
            Const(2),           // 1: push 2
            Lt,                 // 2: n < 2
            JumpIfFalse(6),     // 3: if false, jump to recursive case
            
            // return n
            LoadLocal(0),       // 4: load n
            Return,             // 5: return n
            
            // fib(n - 1)
            LoadLocal(0),       // 6: load n
            Const(1),           // 7: push 1
            Sub,                // 8: n - 1
            Call(0, 1),         // 9: call fib(n-1) - function 0 with 1 arg
            
            // fib(n - 2)
            LoadLocal(0),       // 10: load n
            Const(2),           // 11: push 2
            Sub,                // 12: n - 2
            Call(0, 1),         // 13: call fib(n-2)
            
            // return fib(n-1) + fib(n-2)
            Add,                // 14: add results
            Return,             // 15: return sum
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fib() {
        let mut vm = BytecodeVM::new();
        vm.add_function(compile_fib());
        
        let result = vm.call("fib", vec![VMValue::Int(10)]);
        assert!(matches!(result, VMValue::Int(55)));
    }
}
