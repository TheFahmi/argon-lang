# Cryo Performance Optimization Guide

This document describes the performance optimizations implemented in the Cryo interpreter and compiler.

## Overview

Cryo v2.24.0 includes significant performance improvements:

| Mode | Performance | Use Case |
|------|-------------|----------|
| **Tree-walking Interpreter** | Baseline | Development, debugging |
| **Bytecode VM** | ~16x faster | General execution |
| **Native (LLVM)** | ~1000x faster | Production, benchmarks |

---

## 1. Interpreter Optimizations

### 1.1 FxHashMap (Faster Hash Maps)

Replaced `std::collections::HashMap` with `FxHashMap` from `rustc-hash` crate for ~2-3x faster lookups.

```rust
// Before
use std::collections::HashMap;

// After
use rustc_hash::FxHashMap;
```

**Files modified:**
- `src/interpreter.rs` - All HashMap/HashSet replaced

### 1.2 Scope Management Optimization

```rust
// Pre-allocate scope with capacity
fn pushScopeWithCapacity(&mut self, cap: usize) {
    self.stack.push(ScopeFrame::withCapacity(cap));
}

// Fast path for pop_scope when no deferred statements
#[inline]
fn popScope(&mut self) -> Result<(), ControlFlow> {
    if let Some(scope) = self.stack.last() {
        if scope.deferred.is_empty() {
            self.stack.pop();
            return Ok(());
        }
    }
    // ... handle deferred
}
```

### 1.3 Inline Hints on Hot Paths

Added `#[inline]` to frequently called functions:

- `getVar()` - Variable lookup
- `setVar()` - Variable assignment
- `declareVar()` - Variable declaration
- `pushScope()` / `popScope()` - Scope management
- `executeFunction()` - Function execution
- `execStmts()` - Statement execution
- `evalExpr()` - Expression evaluation
- `evalBinop()` - Binary operations

### 1.4 Pre-allocated Stacks

```rust
impl BytecodeVM {
    pub fn new() -> Self {
        BytecodeVM {
            stack: Vec::withCapacity(4096),  // Pre-allocate
            frames: Vec::withCapacity(256),
            // ...
        }
    }
}
```

---

## 2. Bytecode VM

A new stack-based bytecode VM provides ~16x speedup over tree-walking interpretation.

### Architecture

```
Source (.cryo) -> AST -> Bytecode -> VM Execution
```

### Key Features

- **Stack-based execution** - No HashMap lookups during execution
- **Pre-compiled bytecode** - Parsed once, executed many times
- **Direct variable indexing** - Variables accessed by index, not name
- **Compact opcodes** - Efficient instruction dispatch

### Usage

```bash
# Run fibonacci via bytecode VM
./cryo.exe --vm-bench 35

# Output:
# Cryo VM: Running Fib(35)...
# Cryo VM: Result = 9227465
# Cryo VM: Time = 3015ms
```

### OpCode Set

```rust
pub enum OpCode {
    Const(i64),         // Push constant
    Add, Sub, Mul, Div, // Arithmetic
    Lt, Gt, Eq, Ne,     // Comparison
    LoadLocal(usize),   // Load variable by index
    Call(usize, usize), // Call function
    Return,             // Return from function
    JumpIfFalse(usize), // Conditional jump
    // ...
}
```

---

## 3. Native Compilation (LLVM)

For maximum performance, Cryo can be compiled to native code via LLVM IR.

### Compilation Pipeline

```
Source (.cryo) -> Parser -> AST -> LLVM IR -> Native Binary
```

### Self-Hosted Compiler

The `self-host/compiler.cryo` generates optimized LLVM IR:

```bash
# Compile Cryo to LLVM IR
./cryo.exe self-host/compiler.cryo myprogram.cryo -o myprogram.ll

# Compile LLVM IR to native
clang -O3 myprogram.ll -o myprogram
```

### Docker Benchmark

```bash
# Build and run comprehensive benchmarks
docker build -t cryo-bench .
docker run --rm cryo-bench
```

---

## 4. Build Configuration

### Cargo.toml Optimizations

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

[dependencies]
rustc-hash = "1.1"
```

### Compiler Flags

For LLVM IR compilation:

```bash
clang -O3 -march=native -mtune=native -funroll-loops program.ll -o program
```

---

## 5. Benchmark Results

### Fibonacci(35) - Recursive

| Mode | Time | Speedup |
|------|------|---------|
| Interpreter | ~50s | 1x |
| Bytecode VM | ~3s | 16x |
| Native (LLVM) | ~45ms | 1000x |

### Fibonacci(45) - Heavy Recursion

| Language | Time |
|----------|------|
| C++ (native) | 4.1s |
| **Cryo (native)** | **5.1s** |
| Rust (native) | 6.3s |

### Ackermann(3, 11) - Deep Recursion

| Language | Time |
|----------|------|
| C++ (native) | 136ms |
| **Cryo (native)** | **232ms** |
| Rust (native) | 261ms |

---

## 6. Future Optimizations

- [ ] JIT Compilation with Cranelift
- [ ] Tail Call Optimization
- [ ] Memoization for pure functions
- [ ] SIMD vectorization
- [ ] Profile-Guided Optimization (PGO)

---

## References

- `src/interpreter.rs` - Optimized interpreter
- `src/bytecode_vm.rs` - Bytecode virtual machine
- `self-host/compiler.cryo` - Self-hosted LLVM compiler
- `benchmarks/comparison/run.sh` - Benchmark suite
