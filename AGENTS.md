# Cryo Language - Agent Guide

Dokumentasi ini membantu AI coding agents memahami struktur dan cara kerja proyek Cryo.

## 🎯 Project Overview

**Cryo** adalah bahasa pemrograman systems-level yang:
- **Self-hosted**: Compiler ditulis dalam Cryo sendiri (`self-host/compiler.ar`)
- **High-performance**: Native compilation via LLVM (target: 40ms untuk Fib(35))
- **Multi-target**: Interpreter, Bytecode VM, LLVM IR, WASM

## 📁 Struktur Proyek

```
cryo/
├── src/                    # Rust implementation (native compiler)
│   ├── main.rs             # CLI entry point (v3.1.0)
│   ├── lexer.rs            # Tokenizer
│   ├── parser.rs           # AST parser (Expr, Stmt, TopLevel)
│   ├── interpreter.rs      # Tree-walking interpreter
│   ├── native_compiler.rs  # Cryo -> LLVM IR compiler
│   ├── bytecode_vm.rs      # Bytecode Virtual Machine
│   ├── fast_vm.rs          # Native Rust benchmarks
│   ├── optimizer.rs        # Constant folding, dead code elimination
│   ├── expander.rs         # Macro expansion
│   ├── ffi.rs              # Foreign Function Interface
│   └── gc.rs               # Reference counting GC
│
├── self-host/              # Self-hosted compiler (Cryo source)
│   ├── compiler.ar         # Main compiler in Cryo
│   └── runtime.rs          # Runtime support (Rust)
│
├── stdlib/                 # Standard Library
│   ├── std.ar              # Core (print, len, type)
│   ├── math.ar             # Math functions
│   ├── string.ar           # String operations
│   ├── array.ar            # Array utilities
│   ├── http.ar             # HTTP client/server
│   ├── crypto.ar           # Cryptography
│   └── ...                 # More modules
│
├── examples/               # Example programs
├── docs/                   # Design documents
├── benchmarks/             # Performance benchmarks
├── lsp/                    # Language Server Protocol
└── tools/                  # Development tools
```

## 🔧 Key Components

### 1. Lexer (`src/lexer.rs`)

Tokenizes Cryo source code into tokens.

```rust
pub enum Token {
    Fn, Let, Return, If, Else, While, Print, ...
    Number(i64), String(String), Identifier(String),
    Plus, Minus, Star, Slash, ...
}
```

### 2. Parser (`src/parser.rs`)

Parses tokens into AST.

**Key Types:**
```rust
pub enum Expr {
    Number(i64),
    String(String),
    Identifier(String),
    BinOp(Box<Expr>, String, Box<Expr>),
    Call(String, Vec<Expr>),
    ...
}

pub enum Stmt {
    Let(String, Option<String>, Expr),  // name, type, value
    Return(Option<Expr>),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    ...
}

pub enum TopLevel {
    Function(Function),
    Struct(StructDef),
    Trait(TraitDef),
    Impl(ImplDef),
    ...
}
```

### 3. Native Compiler (`src/native_compiler.rs`)

Compiles Cryo AST directly to LLVM IR.

```rust
pub fn compileToLlvm(source: &str) -> Result<String, String>
```

### 4. Interpreter (`src/interpreter.rs`)

Tree-walking interpreter for running Cryo code.

## 🚀 Build & Run Commands

```bash
# Build
cargo build --release

# Run file (native mode - default)
./cryo.exe examples/hello.ar

# Run with interpreter (fallback)
./cryo.exe --interpret examples/hello.ar

# Benchmark
./cryo.exe --native-bench 35    # Target: ~40ms
./cryo.exe --vm-bench 35        # Bytecode VM

# Generate LLVM IR
./cryo.exe --emit-llvm output.ll source.ar
```

## 📝 Cryo Language Syntax

### Basic
```javascript
fn main() {
    let x: int = 42;
    print(x);
}
```

### Functions
```javascript
fn add(a: int, b: int) -> int {
    return a + b;
}
```

### Structs
```javascript
struct Point {
    x: int,
    y: int
}

let p = Point { x: 10, y: 20 };
```

### Traits & Generics
```javascript
trait Printable {
    fn toString(self) -> string;
}

impl Printable for Point {
    fn toString(self) -> string {
        return "Point(" + self.x + ", " + self.y + ")";
    }
}

fn print_it<T: Printable>(obj: T) {
    print(obj.toString());
}
```

### Async/Await
```javascript
async fn fetchData() -> string {
    let response = await httpGet("https://api.example.com");
    return response;
}
```

### Macros
```javascript
macro route(app, method, path, handler) {
    $app.router.add($method, $path, $handler);
}
```

### Decorators
```javascript
@Controller("/api/users")
struct UsersController {
    service: UserService
}

impl UsersController {
    @Get("/:id")
    fn getUser(self, id: i32) -> User {
        return self.service.findOne(id);
    }
}
```

## 🧪 Testing

```bash
# Run stdlib tests
./cryo.exe test_stdlib.ar

# Run benchmarks
./build.sh bench 35

# Docker benchmarks
docker build -t cryo-bench .
docker run --rm cryo-bench
```

## 📚 Documentation Files

| File | Description |
|------|-------------|
| `README.md` | Project overview |
| `ROADMAP.md` | Development roadmap |
| `docs/running_native.md` | How to build & run native |
| `docs/cryo_design_doc.md` | Language design |
| `docs/stdlib_reference.md` | Standard library reference |
| `docs/traits_design.md` | Traits system design |
| `docs/async_design.md` | Async/await design |
| `docs/wasm_design.md` | WebAssembly compilation |
| `docs/ffi_design.md` | FFI design |

## ⚠️ Important Notes for Agents

1. **Default Mode**: Since v3.1.0, native mode is default. Use `--interpret` for interpreter.

2. **Parser Types**: 
   - `Expr::Number` (not `Expr::Int`)
   - `Expr::Identifier` (not `Expr::Ident`)
   - `Stmt::Let(name, type_opt, expr)` has 3 fields
   - `Stmt::Return(Option<Expr>)` - expr is optional
   - `Function` and `StructDef` have `decorators` field

3. **File Extensions**: Cryo files use `.ar` extension.

4. **Self-Hosting**: The compiler in `self-host/compiler.ar` is written in Cryo and can compile itself.

5. **Benchmark Target**: Fib(35) should run in ~40ms with native mode.

## 🔗 Quick Links

- [Build Native Compiler](docs/running_native.md)
- [Standard Library](docs/stdlib_reference.md)
- [Language Design](docs/cryo_design_doc.md)
- [Examples](examples/)
