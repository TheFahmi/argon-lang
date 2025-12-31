# Argon Programming Language (v2.20.0)
![Argon Logo](logo.png)

Argon is a high-performance, **self-hosted** systems programming language that compiles directly to using custom Rust Interpreter.

## ✨ Highlights
- **Self-Hosted**: Compiler written in Argon itself (`self-host/compiler.ar`)
- **Verified**: Stage 1 (self-compiled) produces identical output when compiling itself
- **Native Backend**: Uses custom Rust-based interpreter/codegen for optimized execution
- **Foreign Function Interface**: `extern "C"` support and raw pointers (`*i32`) (v2.20.0)
- **Traits System**: Interface definitions (`trait`) and implementations (`impl Trait`) with polymorphism (v2.20.0)
- **WebAssembly**: Compile to WASM for browser deployment (v2.19.0)
- **Async/Await**: Asynchronous functions with `async fn` and `await` expressions (v2.18.0)
- **Debugger**: Full GDB/LLDB support with DWARF debug info (v2.17.0)
- **Generic Types**: Full support for `struct Box<T>`, `fn map<T>(...)` with monomorphization (v2.16.0)
- **REPL**: Interactive mode for quick experimentation (v2.14.0)
- **IDE Support**: VS Code extension with full Language Server Protocol (v2.0.0)
- **Package Manager**: APM with registry, git deps, and lock files (v2.10.0)
- **Standard Library**: 21 modules (math, string, array, async, wasm, collections, etc)
- **Enums & Match**: Enum types with pattern matching (v2.6.0)
- **Structs & Methods**: OO-like programming support (v2.5.0)
- **Networking & Threads**: Built-in TCP Socket and Multi-threading support

## Quick Start
```bash
# Build Rust Interpreter (v2.20.0)
cargo build --release
# Copy binary
cp target/release/argon.exe argon_v220.exe

# Run Hello World
./argon_v220.exe examples/hello.ar
# Run FFI Example
./argon_v220.exe examples/ffi_example.ar
# Run Traits Example
./argon_v220.exe examples/traits_example.ar
```

## Language Features

### Traits (v2.20.0)
```javascript
trait Printable {
    fn to_string(self) -> string;
}

struct Point { x: int, y: int }

impl Printable for Point {
    fn to_string(self) -> string {
        return "Point(" + self.x + ", " + self.y + ")";
    }
}

fn print_it<T: Printable>(obj: T) {
    print(obj.to_string());
}
```

### FFI (Foreign Function Interface) (v2.20.0)
```javascript
extern "C" {
    fn malloc(size: i64) -> *void;
    fn free(ptr: *void);
}

fn main() {
    let ptr = malloc(1024);
    // ... use ptr ...
    free(ptr);
}
```

### Modules & Imports (v2.7.0)
```javascript
// math_utils.ar
fn math_add(a, b) { return a + b; }

// main.ar
import "math_utils.ar";
fn main() { print(math_add(5, 3)); }
```

### Methods (v2.5.0)
```javascript
struct Circle { radius: int }
impl Circle {
    fn area(self) { return 3 * self.radius * self.radius; }
}
```

## Version History
- **v2.21.0**: Garbage Collection (Reference Counting), Reference Semantics for Arrays/Objects.
- **v2.20.0**: FFI (extern, pointers) and Traits (trait, impl, dynamic dispatch) support. New Rust Interpreter.
- **v2.19.0**: WebAssembly target (compile to WASM, browser deployment, WASI support)
- **v2.18.0**: Async/await support (`async fn`, `await` expressions)
- **v2.17.0**: Debugger support (DWARF debug info, GDB integration, -g flag)
- **v2.16.0**: Generic types with full monomorphization
- **v2.15.0**: Generic type syntax support
- **v2.14.0**: Full LSP implementation
- **v2.10.0**: Package Manager (APM)
- **v2.7.0**: Module system
- **v2.6.0**: Enum types
- **v2.5.0**: Struct Methods
- **v2.3.0**: Multi-threading
- **v2.1.0**: Networking

## Roadmap
- [x] Self-Hosting Compiler ✅
- [x] Networking & Multi-threading ✅
- [x] Structs, Methods, Enums ✅
- [x] Standard library (21 modules) ✅
- [x] Package Manager (APM) ✅
- [x] LSP (Language Server Protocol) ✅
- [x] Generic types ✅
- [x] Async/await ✅
- [x] WebAssembly target ✅
- [x] FFI (Foreign Function Interface) ✅ (v2.20.0)
- [x] Traits/Interfaces ✅ (v2.20.0)
- [x] Garbage Collection (RC) ✅ (v2.21.0)
- [ ] Optimization (LTO, Constant Propagation)
- [ ] Destructors / RAII (Auto-cleanup for FFI/Files)
- [ ] Macros / Metaprogramming
- [ ] Ecosystem Demo (Web Framework / Game)

