// ============================================
// CRYO JIT COMPILER
// Cranelift-based Just-In-Time compilation
// ============================================

use std::collections::HashMap;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module, FuncId};
use cranelift_codegen::ir::AbiParam;
use cranelift_codegen::settings::{self, Configurable};

/// Represents a compiled function
pub struct CompiledFunction {
    pub name: String,
    pub func_id: FuncId,
    pub call_count: u64,
    pub is_hot: bool,
}

/// JIT Compiler using Cranelift
pub struct JitCompiler {
    /// The JIT module
    module: JITModule,
    /// Builder context
    builder_context: FunctionBuilderContext,
    /// Cranelift context
    ctx: codegen::Context,
    /// Data context for constants
    data_ctx: DataContext,
    /// Compiled functions cache
    compiled_functions: HashMap<String, CompiledFunction>,
    /// Hot path threshold
    hot_threshold: u64,
    /// Call counts for functions
    call_counts: HashMap<String, u64>,
    /// Whether JIT is enabled
    enabled: bool,
}

impl JitCompiler {
    /// Create a new JIT compiler
    pub fn new() -> Result<Self, String> {
        // Build settings
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").map_err(|e| format!("{}", e))?;
        flag_builder.set("is_pic", "false").map_err(|e| format!("{}", e))?;
        flag_builder.set("opt_level", "speed").map_err(|e| format!("{}", e))?;
        
        let isa_builder = cranelift_native::builder()
            .map_err(|msg| format!("Failed to create ISA builder: {}", msg))?;
        
        let flags = settings::Flags::new(flag_builder);
        let isa = isa_builder
            .finish(flags)
            .map_err(|e| format!("Failed to create ISA: {:?}", e))?;
        
        // Create JIT module
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        let ctx = module.make_context();
        
        Ok(Self {
            module,
            builder_context: FunctionBuilderContext::new(),
            ctx,
            data_ctx: DataContext::new(),
            compiled_functions: HashMap::new(),
            hot_threshold: 100,
            call_counts: HashMap::new(),
            enabled: true,
        })
    }
    
    /// Set the hot path threshold
    pub fn set_hot_threshold(&mut self, threshold: u64) {
        self.hot_threshold = threshold;
    }
    
    /// Enable or disable JIT
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Check if JIT is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Record a function call and check if it's hot
    pub fn record_call(&mut self, name: &str) -> bool {
        let count = self.call_counts.entry(name.to_string()).or_insert(0);
        *count += 1;
        *count >= self.hot_threshold
    }
    
    /// Check if a function should be JIT compiled
    pub fn should_compile(&self, name: &str) -> bool {
        if !self.enabled {
            return false;
        }
        
        // Already compiled?
        if self.compiled_functions.contains_key(name) {
            return false;
        }
        
        // Hot enough?
        if let Some(&count) = self.call_counts.get(name) {
            return count >= self.hot_threshold;
        }
        
        false
    }
    
    /// Check if a function is already compiled
    pub fn is_compiled(&self, name: &str) -> bool {
        self.compiled_functions.contains_key(name)
    }
    
    /// Compile a simple integer function: fn(i64) -> i64
    pub fn compile_simple_function(&mut self, name: &str, body: SimpleFunction) -> Result<*const u8, String> {
        // Clear context
        self.ctx.clear();
        
        // Define function signature: extern "C" fn(i64) -> i64
        let int_type = types::I64;
        // Use the platform's default C calling convention
        self.ctx.func.signature.call_conv = self.module.isa().default_call_conv();
        self.ctx.func.signature.params.push(AbiParam::new(int_type));
        self.ctx.func.signature.returns.push(AbiParam::new(int_type));
        
        // Declare function
        let func_id = self.module
            .declare_function(name, Linkage::Local, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;
        
        // Build function body
        {
            let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);
            
            // Get parameter
            let param = builder.block_params(entry_block)[0];
            
            // Generate code based on function type
            let result = match body {
                SimpleFunction::Identity => param,
                SimpleFunction::Double => {
                    let two = builder.ins().iconst(int_type, 2);
                    builder.ins().imul(param, two)
                },
                SimpleFunction::Square => {
                    builder.ins().imul(param, param)
                },
                SimpleFunction::Increment => {
                    let one = builder.ins().iconst(int_type, 1);
                    builder.ins().iadd(param, one)
                },
                SimpleFunction::Negate => {
                    builder.ins().ineg(param)
                },
                SimpleFunction::Custom(ops) => {
                    let mut current = param;
                    for op in ops {
                        current = match op {
                            JitOp::Add(n) => {
                                let val = builder.ins().iconst(int_type, n);
                                builder.ins().iadd(current, val)
                            },
                            JitOp::Sub(n) => {
                                let val = builder.ins().iconst(int_type, n);
                                builder.ins().isub(current, val)
                            },
                            JitOp::Mul(n) => {
                                let val = builder.ins().iconst(int_type, n);
                                builder.ins().imul(current, val)
                            },
                            JitOp::Div(n) => {
                                let val = builder.ins().iconst(int_type, n);
                                builder.ins().sdiv(current, val)
                            },
                        };
                    }
                    current
                },
            };
            
            builder.ins().return_(&[result]);
            builder.finalize();
        }
        
        // Define and compile
        self.module.define_function(func_id, &mut self.ctx)
            .map_err(|e| e.to_string())?;
        
        // Finalize
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions()
            .map_err(|e| e.to_string())?;
        
        // Get code pointer
        let code_ptr = self.module.get_finalized_function(func_id);
        
        // Store compiled function info
        self.compiled_functions.insert(name.to_string(), CompiledFunction {
            name: name.to_string(),
            func_id,
            call_count: *self.call_counts.get(name).unwrap_or(&0),
            is_hot: true,
        });
        
        Ok(code_ptr)
    }
    
    /// Call a compiled function
    pub unsafe fn call_compiled(&self, name: &str, arg: i64) -> Option<i64> {
        if let Some(func) = self.compiled_functions.get(name) {
            let code_ptr = self.module.get_finalized_function(func.func_id);
            let func: extern "C" fn(i64) -> i64 = std::mem::transmute(code_ptr);
            Some(func(arg))
        } else {
            None
        }
    }
    
    /// Get compiled function count
    pub fn compiled_count(&self) -> usize {
        self.compiled_functions.len()
    }
    
    /// Get hot function names
    pub fn get_hot_functions(&self) -> Vec<String> {
        self.call_counts
            .iter()
            .filter(|(_, &count)| count >= self.hot_threshold)
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Get call statistics
    pub fn get_stats(&self) -> JitStats {
        JitStats {
            enabled: self.enabled,
            hot_threshold: self.hot_threshold,
            total_functions_tracked: self.call_counts.len(),
            compiled_functions: self.compiled_functions.len(),
            hot_functions: self.get_hot_functions().len(),
        }
    }
    
    /// Reset all tracking
    pub fn reset(&mut self) {
        self.call_counts.clear();
        // Note: compiled functions remain in memory
    }
}

/// Simple function types for JIT compilation
pub enum SimpleFunction {
    Identity,
    Double,
    Square,
    Increment,
    Negate,
    Custom(Vec<JitOp>),
}

/// JIT operations for custom functions
pub enum JitOp {
    Add(i64),
    Sub(i64),
    Mul(i64),
    Div(i64),
}

/// JIT statistics
pub struct JitStats {
    pub enabled: bool,
    pub hot_threshold: u64,
    pub total_functions_tracked: usize,
    pub compiled_functions: usize,
    pub hot_functions: usize,
}

impl std::fmt::Display for JitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JIT Stats: enabled={}, threshold={}, tracked={}, compiled={}, hot={}",
            self.enabled,
            self.hot_threshold,
            self.total_functions_tracked,
            self.compiled_functions,
            self.hot_functions
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jit_compile_double() {
        let mut jit = JitCompiler::new().expect("Failed to create JIT");
        
        let code_ptr = jit.compile_simple_function("double", SimpleFunction::Double)
            .expect("Failed to compile");
        
        assert!(!code_ptr.is_null());
        
        unsafe {
            let result = jit.call_compiled("double", 21).unwrap();
            assert_eq!(result, 42);
        }
    }
    
    #[test]
    fn test_jit_compile_square() {
        let mut jit = JitCompiler::new().expect("Failed to create JIT");
        
        jit.compile_simple_function("square", SimpleFunction::Square)
            .expect("Failed to compile");
        
        unsafe {
            let result = jit.call_compiled("square", 5).unwrap();
            assert_eq!(result, 25);
        }
    }
    
    #[test]
    fn test_jit_hot_path_detection() {
        let mut jit = JitCompiler::new().expect("Failed to create JIT");
        jit.set_hot_threshold(5);
        
        for _ in 0..4 {
            assert!(!jit.record_call("test_fn"));
        }
        
        assert!(jit.record_call("test_fn")); // 5th call triggers hot
        assert!(jit.should_compile("test_fn"));
    }
}
