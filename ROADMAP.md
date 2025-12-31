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
    - [x] VS Code Extension
    - [x] REPL

---

## ✅ Phase 2: Primitive Features (v2.26 - v2.28) [COMPLETED]
Advanced language features for production software.

### ✅ 1. Traits & Interfaces [v2.26.0]
- [x] `TraitDef` in runtime
- [x] `impl Trait for Type` support
- [x] Method dispatch with polymorphism

### ✅ 2. FFI (Foreign Function Interface) [v2.27.0]
- [x] `libloading` integration
- [x] load .dll/.so dynamically
- [x] `ffi_call()` for C interop

### ✅ 3. Garbage Collection [v2.28.0]
- [x] Mark-and-Sweep GC
- [x] `gc_collect()` and `gc_stats()`
- [x] Safe memory management

---

## ✅ Phase 3: Developer Experience (v2.29) [COMPLETED]
Focus on tooling and productivity.

### ✅ LSP & Debugger [v2.29.0]
- [x] Language Server Protocol (Diagnostics, Hover, Go-to-Def)
- [x] Debugger Adapter (DAP) with GDB support
- [x] DWARF debug info generation
- [x] Source map support

---

## ✅ Phase 4: Enterprise (v3.0) [COMPLETED]
Enterprise-grade standard library modules.

### ✅ Enterprise Modules [v3.0.0]
- [x] `crypto` (Hashing, Encryption, JWT, UUID)
- [x] `http` (Server, Router, Client, Cookies)
- [x] `sql` (In-memory DB, Query Builder)

---

## 🔮 Phase 5: The Next Horizon (v3.1 - v4.0) [NEXT]
Expanding Argon to new platforms and paradigms.

### 1. True Concurrency (Multithreading)
Unlock multicore performance.
- [ ] M:N Threading Model (fiber-based like Go routines)
- [ ] Channels for message passing (`chan<T>`)
- [ ] Parallel iterators (`par_iter`)
- [ ] Atomic primitives

### 2. Desktop & GUI Support
Build cross-platform desktop apps with Argon.
- [ ] Bindings for raylib or SDL2 (`argon-gfx`)
- [ ] Declarative UI Framework (React-like syntax)
- [ ] Window management & Input handling

### 3. Mobile Support
Run Argon on iOS and Android.
- [ ] ARM64 Native Compilation target
- [ ] Android NDK integration
- [ ] iOS Framework bundling

### 4. Package Registry (apm.argon.dev)
Centralized package management.
- [ ] `apm publish` command
- [ ] Semantic versioning resolution
- [ ] Lockfile generation (`argon.lock`)

---

## Release Schedule
| Version | Feature | Status |
|---------|---------|--------|
| v2.25.0 | Performance & Stdlib | ✅ |
| v2.26.0 | Traits & Interfaces | ✅ |
| v2.27.0 | FFI Support | ✅ |
| v2.28.0 | Garbage Collector | ✅ |
| v2.29.0 | LSP & Debugger | ✅ |
| v3.0.0  | Enterprise Features | ✅ (Current) |
| v3.1.0  | Concurrency & Channels | 🔮 Next |
| v3.2.0  | GUI Framework | 🔮 Planned |
