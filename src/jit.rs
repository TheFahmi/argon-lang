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

// ============================================
// METHOD INLINING
// ============================================

/// Inlining configuration and state
pub struct InliningConfig {
    /// Maximum depth of inlining (prevent infinite recursion)
    pub max_depth: usize,
    /// Maximum size of function to inline (in operations)
    pub max_inline_size: usize,
    /// Minimum call count before considering inlining
    pub min_call_count: u64,
    /// Functions that have been inlined
    pub inlined_functions: HashMap<String, InlinedFunction>,
}

impl Default for InliningConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_inline_size: 20,
            min_call_count: 50,
            inlined_functions: HashMap::new(),
        }
    }
}

/// Represents an inlined function
pub struct InlinedFunction {
    pub name: String,
    pub inline_count: usize,
    pub size: usize,
    pub caller_sites: Vec<String>,
}

impl InliningConfig {
    /// Create new inlining config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Check if a function should be inlined
    pub fn should_inline(&self, name: &str, size: usize, call_count: u64) -> bool {
        size <= self.max_inline_size && call_count >= self.min_call_count
    }
    
    /// Mark a function as inlined
    pub fn mark_inlined(&mut self, name: &str, caller: &str, size: usize) {
        let entry = self.inlined_functions.entry(name.to_string()).or_insert(InlinedFunction {
            name: name.to_string(),
            inline_count: 0,
            size,
            caller_sites: Vec::new(),
        });
        entry.inline_count += 1;
        entry.caller_sites.push(caller.to_string());
    }
    
    /// Get inlining statistics
    pub fn get_stats(&self) -> InliningStats {
        let total_inlines: usize = self.inlined_functions.values().map(|f| f.inline_count).sum();
        InliningStats {
            functions_inlined: self.inlined_functions.len(),
            total_inline_sites: total_inlines,
            max_depth: self.max_depth,
            max_size: self.max_inline_size,
        }
    }
}

/// Inlining statistics
pub struct InliningStats {
    pub functions_inlined: usize,
    pub total_inline_sites: usize,
    pub max_depth: usize,
    pub max_size: usize,
}

// ============================================
// TYPE SPECIALIZATION
// ============================================

/// Represents specialized types for JIT
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecializedType {
    Int64,
    Float64,
    Bool,
    String,
    Array,
    Object,
    Unknown,
}

impl SpecializedType {
    /// Get Cranelift type for this specialized type
    pub fn to_cranelift_type(&self) -> Option<types::Type> {
        match self {
            SpecializedType::Int64 => Some(types::I64),
            SpecializedType::Float64 => Some(types::F64),
            SpecializedType::Bool => Some(types::I8),
            _ => None, // Complex types need pointer representation
        }
    }
    
    /// Check if this type can be unboxed for optimization
    pub fn is_unboxable(&self) -> bool {
        matches!(self, SpecializedType::Int64 | SpecializedType::Float64 | SpecializedType::Bool)
    }
}

/// Type specialization configuration
pub struct TypeSpecialization {
    /// Observed types for each function parameter
    pub observed_types: HashMap<String, Vec<SpecializedType>>,
    /// Specialized versions of functions
    pub specialized_versions: HashMap<String, Vec<SpecializedVersion>>,
    /// Enable type speculation
    pub enable_speculation: bool,
}

impl Default for TypeSpecialization {
    fn default() -> Self {
        Self {
            observed_types: HashMap::new(),
            specialized_versions: HashMap::new(),
            enable_speculation: true,
        }
    }
}

/// A specialized version of a function
pub struct SpecializedVersion {
    pub base_name: String,
    pub param_types: Vec<SpecializedType>,
    pub return_type: SpecializedType,
    pub compiled: bool,
}

impl TypeSpecialization {
    /// Create new type specialization config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record observed type for a function parameter
    pub fn record_type(&mut self, func_name: &str, param_index: usize, observed: SpecializedType) {
        let types = self.observed_types.entry(func_name.to_string()).or_insert_with(Vec::new);
        while types.len() <= param_index {
            types.push(SpecializedType::Unknown);
        }
        
        // If we see a consistent type, record it
        if types[param_index] == SpecializedType::Unknown {
            types[param_index] = observed;
        } else if types[param_index] != observed {
            // Mixed types - keep as unknown for now
            // In a full impl, we'd create multiple specialized versions
        }
    }
    
    /// Check if a function has stable types for specialization
    pub fn can_specialize(&self, func_name: &str) -> bool {
        if let Some(types) = self.observed_types.get(func_name) {
            types.iter().all(|t| t.is_unboxable())
        } else {
            false
        }
    }
    
    /// Get specialized function name
    pub fn get_specialized_name(&self, func_name: &str) -> Option<String> {
        if let Some(types) = self.observed_types.get(func_name) {
            let suffix: String = types.iter().map(|t| match t {
                SpecializedType::Int64 => "i",
                SpecializedType::Float64 => "f",
                SpecializedType::Bool => "b",
                _ => "o",
            }).collect();
            Some(format!("{}_{}", func_name, suffix))
        } else {
            None
        }
    }
}

// ============================================
// TRACE-BASED JIT
// ============================================

/// Represents a recorded trace for JIT compilation
pub struct Trace {
    pub id: usize,
    pub operations: Vec<TraceOp>,
    pub start_location: String,
    pub loop_count: usize,
    pub is_compiled: bool,
}

/// Operations in a trace
#[derive(Clone, Debug)]
pub enum TraceOp {
    LoadInt(i64),
    LoadFloat(f64),
    LoadVar(String),
    StoreVar(String),
    Add,
    Sub,
    Mul,
    Div,
    Compare(CompareOp),
    Jump(usize),
    ConditionalJump(usize),
    Call(String),
    Return,
    Guard(GuardType),
}

/// Comparison operations
#[derive(Clone, Debug)]
pub enum CompareOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

/// Guard types for deoptimization
#[derive(Clone, Debug)]
pub enum GuardType {
    TypeCheck(SpecializedType),
    OverflowCheck,
    BoundsCheck(usize),
}

/// Trace recorder for trace-based JIT
pub struct TraceRecorder {
    /// Whether recording is active
    pub is_recording: bool,
    /// Current trace being recorded
    pub current_trace: Option<Trace>,
    /// All recorded traces
    pub traces: Vec<Trace>,
    /// Trace ID counter
    next_trace_id: usize,
    /// Hot loop threshold
    pub loop_threshold: usize,
    /// Loop iteration counts
    loop_counts: HashMap<String, usize>,
}

impl Default for TraceRecorder {
    fn default() -> Self {
        Self {
            is_recording: false,
            current_trace: None,
            traces: Vec::new(),
            next_trace_id: 0,
            loop_threshold: 10,
            loop_counts: HashMap::new(),
        }
    }
}

impl TraceRecorder {
    /// Create new trace recorder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record a loop iteration
    pub fn record_loop(&mut self, location: &str) -> bool {
        let count = self.loop_counts.entry(location.to_string()).or_insert(0);
        *count += 1;
        
        if *count >= self.loop_threshold && !self.is_recording {
            self.start_recording(location);
            true
        } else {
            false
        }
    }
    
    /// Start recording a trace
    pub fn start_recording(&mut self, location: &str) {
        self.is_recording = true;
        self.current_trace = Some(Trace {
            id: self.next_trace_id,
            operations: Vec::new(),
            start_location: location.to_string(),
            loop_count: 0,
            is_compiled: false,
        });
        self.next_trace_id += 1;
    }
    
    /// Record an operation
    pub fn record_op(&mut self, op: TraceOp) {
        if let Some(ref mut trace) = self.current_trace {
            trace.operations.push(op);
        }
    }
    
    /// Stop recording and save trace
    pub fn stop_recording(&mut self) -> Option<usize> {
        self.is_recording = false;
        if let Some(trace) = self.current_trace.take() {
            let id = trace.id;
            self.traces.push(trace);
            Some(id)
        } else {
            None
        }
    }
    
    /// Get trace by ID
    pub fn get_trace(&self, id: usize) -> Option<&Trace> {
        self.traces.iter().find(|t| t.id == id)
    }
    
    /// Get number of recorded traces
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }
    
    /// Check if a loop is hot
    pub fn is_hot_loop(&self, location: &str) -> bool {
        self.loop_counts.get(location).map(|&c| c >= self.loop_threshold).unwrap_or(false)
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
    
    #[test]
    fn test_inlining_config() {
        let mut config = InliningConfig::new();
        
        // Test should_inline
        assert!(!config.should_inline("small_fn", 10, 10)); // Not enough calls
        assert!(config.should_inline("small_fn", 10, 50));  // Enough calls, small size
        assert!(!config.should_inline("big_fn", 100, 50));  // Too big
        
        // Test mark_inlined
        config.mark_inlined("helper", "main", 5);
        config.mark_inlined("helper", "process", 5);
        
        let stats = config.get_stats();
        assert_eq!(stats.functions_inlined, 1);
        assert_eq!(stats.total_inline_sites, 2);
    }
    
    #[test]
    fn test_type_specialization() {
        let mut spec = TypeSpecialization::new();
        
        // Record consistent int types
        spec.record_type("add", 0, SpecializedType::Int64);
        spec.record_type("add", 1, SpecializedType::Int64);
        
        assert!(spec.can_specialize("add"));
        assert_eq!(spec.get_specialized_name("add"), Some("add_ii".to_string()));
        
        // Mixed types should not specialize easily
        spec.record_type("concat", 0, SpecializedType::String);
        assert!(!spec.can_specialize("concat"));
    }
    
    #[test]
    fn test_trace_recorder() {
        let mut recorder = TraceRecorder::new();
        recorder.loop_threshold = 3;
        
        // Record loop iterations
        assert!(!recorder.record_loop("main:10"));
        assert!(!recorder.record_loop("main:10"));
        assert!(recorder.record_loop("main:10")); // 3rd triggers recording
        
        assert!(recorder.is_recording);
        
        // Record some operations
        recorder.record_op(TraceOp::LoadInt(10));
        recorder.record_op(TraceOp::LoadVar("i".to_string()));
        recorder.record_op(TraceOp::Add);
        
        // Stop recording
        let trace_id = recorder.stop_recording();
        assert!(trace_id.is_some());
        assert!(!recorder.is_recording);
        
        // Check trace
        let trace = recorder.get_trace(trace_id.unwrap());
        assert!(trace.is_some());
        assert_eq!(trace.unwrap().operations.len(), 3);
        
        assert!(recorder.is_hot_loop("main:10"));
        assert!(!recorder.is_hot_loop("main:20"));
    }
    
    #[test]
    fn test_specialized_type() {
        assert!(SpecializedType::Int64.is_unboxable());
        assert!(SpecializedType::Float64.is_unboxable());
        assert!(SpecializedType::Bool.is_unboxable());
        assert!(!SpecializedType::String.is_unboxable());
        assert!(!SpecializedType::Array.is_unboxable());
        
        assert!(SpecializedType::Int64.to_cranelift_type().is_some());
        assert!(SpecializedType::String.to_cranelift_type().is_none());
    }
}
