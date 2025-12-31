# Argon Roadmap

Argon is evolving rapidly. This document outlines the current state and future milestones for the language.

## ✅ Phase 1: Foundation (v1.0 - v2.25) [COMPLETED]
The core infrastructure is now stable and performant.

- **Runtime**:
    - [x] Tree-walk Interpreter
    - [x] Bytecode VM (Register-based, ~16x faster)
    - [x] Optimized HashMaps (`FxHashMap`)
    - [x] Standard Library (Math, String, Array, IO, Net)
- **Compiler**:
    - [x] Self-hosted Compiler (`argonc`)
    - [x] LLVM IR Backend (`native` target)
    - [x] WebAssembly Backend (`wasm32` / `wasi` target)
- **Tooling**:
    - [x] Package Manager (`apm`)
    - [x] Build System (`build.sh`)
    - [x] VS Code Extension (Syntax Highlighting)
    - [x] REPL

---

## ✅ Phase 2: Language Features (v2.26 - v2.28) [COMPLETED]
Advanced language features for production software.

### ✅ 1. Traits & Interfaces [v2.26.0]
- [x] `TraitDef` in runtime
- [x] `impl Trait for Type` support
- [x] Method dispatch with polymorphism

### ✅ 2. FFI (Foreign Function Interface) [v2.27.0]
- [x] `libloading` crate integration
- [x] `ffi_load()` and `ffi_call()` built-ins
- [x] Load .dll/.so dynamically

### ✅ 3. Garbage Collection [v2.28.0]
- [x] Mark-and-Sweep GC module
- [x] `gc_collect()` and `gc_stats()` built-ins
- [x] Object header & heap arena

---

## ✅ Phase 3: Developer Experience (v2.29) [COMPLETED]
Focus on tooling and developer productivity.

### ✅ 1. Language Server Protocol (LSP)
- [x] Diagnostics (syntax errors)
- [x] Hover (function signatures)
- [x] Go to Definition (Ctrl+Click)
- [x] Find References (Shift+F12)
- [x] Autocomplete with snippets
- [x] Signature help (parameter hints)
- [x] Document formatting

### ✅ 2. Debugger Support
- [x] DWARF debug info in LLVM IR
- [x] `-g` / `--debug` compiler flag
- [x] GDB/LLDB integration
- [x] Breakpoints & variable inspection

---

## 🔮 Phase 4: Enterprise Features (v3.0+) [NEXT]
Focus on ecosystem and enterprise readiness.

- **Standard Library Expansion**:
    - [ ] `crypto` module (via FFI to OpenSSL)
    - [ ] `sql` module (SQLite/Postgres bindings)
    - [ ] `http` module (high-performance server)
- **Concurrency**:
    - [ ] True parallelism (M:N threading)
    - [ ] Channel-based communication
- **Tooling**:
    - [ ] Package registry (apm.argon.dev)
    - [ ] Documentation generator

---

## Release Schedule
| Version | Feature | Status |
|---------|---------|--------|
| v2.25.0 | Performance & Stdlib | ✅ |
| v2.26.0 | Traits & Interfaces | ✅ |
| v2.27.0 | FFI Support | ✅ |
| v2.28.0 | Garbage Collector | ✅ |
| v3.0.0 | LSP & Debugger | ✅ (Current) |
| v3.0.0  | Enterprise Features | 🔮 Next |
