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

### 1. Traits & Interfaces (Next Milestone)
Enable polymorphism and better code reuse.
- **Goal**: Allow defining contracts (`trait`) that structs can implement.
- **Current Status**: Parser supports `trait` keyword, but Runtime ignores it.
- **Tasks**:
    - [ ] Add `TraitDef` to valid runtime types.
    - [ ] Implement V-Table / Dispatch mechanism in Interpreter.
    - [ ] Add `impl Trait for Struct` logic.

### 2. Garbage Collection (GC)
Move beyond Reference Counting to handle complex memory graphs safely.
- **Goal**: Replace `Rc<RefCell<T>>` with a Mark-and-Sweep Garbage Collector.
- **Current Status**: Design doc exists. Using standard Rust `Rc` (leaks on cycles).
- **Tasks**:
    - [ ] Implement Object Header & Heap Arena.
    - [ ] Implement Tracing (Mark phase).
    - [ ] Implement Sweeping.
    - [ ] Integrate into Bytecode VM.

### 3. FFI (Foreign Function Interface)
Unlock the ecosystem of C dynamic libraries.
- **Goal**: Load `.dll` / `.so` files and call C functions directly.
- **Current Status**: Parser supports `extern`, Runtime unimplemented.
- **Tasks**:
    - [ ] Integrate `libloading` crate.
    - [ ] Implement dynamic symbol lookup.
    - [ ] Map Argon types <-> C ABI.

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
- **v2.25.0**: Performance & Stdlib (Current)
- **v2.26.0**: Traits & Interfaces Implementation
- **v2.27.0**: FFI Support
- **v2.30.0**: Garbage Collector Integration
