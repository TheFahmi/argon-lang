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

## 🚧 Phase 2: Language Features (v2.26 - v3.0) [CURRENT FOCUS]
Focus on advanced language features to make Argon viable for complex production software.

### ✅ 1. Traits & Interfaces [COMPLETED v2.26.0]
Enable polymorphism and better code reuse.
- [x] Add `TraitDef` to valid runtime types.
- [x] Register trait implementations in `impl Trait for Type`.
- [x] Method dispatch works with trait-based polymorphism.
- [x] Example: `examples/traits_test.ar`

### 2. Garbage Collection (GC) [NEXT]
Move beyond Reference Counting to handle complex memory graphs safely.
- **Goal**: Replace `Rc<RefCell<T>>` with a Mark-and-Sweep Garbage Collector.
- **Current Status**: Design doc exists. Using standard Rust `Rc` (leaks on cycles).
- **Tasks**:
    - [ ] Implement Object Header & Heap Arena.
    - [ ] Implement Tracing (Mark phase).
    - [ ] Implement Sweeping.
    - [ ] Integrate into Bytecode VM.

### ✅ 3. FFI (Foreign Function Interface) [COMPLETED v2.27.0]
Load and call C dynamic libraries directly from Argon.
- [x] Integrate `libloading` crate.
- [x] `ffi_load(libname)` - Load .dll/.so files.
- [x] `ffi_call(lib, func, args)` - Call C functions.
- [x] Example: `examples/ffi_test.ar`

---

## 🔮 Phase 3: Ecosystem & Stability (v3.0+)
Focus on developer experience and enterprise readiness.

- **Language Server Protocol (LSP)**: Real-time error checking and "Go to Definition".
- **Debugger**: Step-through debugging support.
- **Standard Library Expansion**:
    - `crypto` (OpenSSL binding via FFI)
    - `sql` (SQLite/Postgres binding)
    - `http` (Native high-performance server)
- **Concurrency**: True parallelism (M:N threading model) beyond current async/await.

---

## Release Schedule
- **v2.25.0**: Performance & Stdlib ✅
- **v2.26.0**: Traits & Interfaces ✅
- **v2.27.0**: FFI Support ✅ (Current)
- **v2.28.0**: Garbage Collector Integration

