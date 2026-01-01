# Cryo Programming Language

<p align="center">
  <img src="logo.svg" alt="Cryo Logo" width="150" height="auto">
</p>

<p align="center">
  <strong>Version 3.2.1</strong> | High-Performance Systems Programming Language
</p>

<p align="center">
  <a href="http://makeapullrequest.com"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" alt="PRs Welcome"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
  <img src="https://img.shields.io/badge/Cryo-v3.2.1-crimson?style=flat-square" alt="Version">
  <img src="https://img.shields.io/badge/build-passing-brightgreen?style=flat-square" alt="Build Status">
  <img src="https://img.shields.io/badge/platform-win%20%7C%20linux%20%7C%20macos-lightgrey?style=flat-square" alt="Platform">
</p>

---

Cryo is a self-hosted, high-performance systems programming language designed for modern development. Since v3.2.1, Cryo uses **native compilation by default** for maximum performance, achieving near C++ speeds.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Language Guide](#language-guide)
- [Standard Library](#standard-library)
- [Performance](#performance)
- [CryoWeb Framework](#cryoweb-framework)
- [Tooling](#tooling)
- [Version History](#version-history)
- [Documentation](#documentation)
- [License](#license)

---

## Features

### Core Language
- **Native Compilation**: Compiles to LLVM IR for near-C++ performance
- **Self-Hosted**: Compiler written in Cryo itself
- **Strong Type System**: Static typing with type inference
- **Traits & Generics**: Rust-inspired trait system with generic programming
- **Async/Await**: First-class asynchronous programming support
- **Pattern Matching**: Powerful match expressions for control flow

### Memory & Safety
- **Garbage Collection**: Automatic memory management via reference counting
- **Defer Statement**: RAII-style resource cleanup
- **Memory Safety**: No null pointer exceptions, safe array access

### Interoperability
- **FFI**: Call C functions with `extern "C"` declarations
- **WebAssembly**: Compile to WASM for browser deployment
- **JSON**: Built-in JSON parsing and serialization

### Developer Experience
- **LSP Support**: Full VS Code integration with autocomplete
- **Package Manager**: APM for dependency management
- **CLI Tools**: Project scaffolding and build tools
- **Macros**: Hygienic macro system for metaprogramming
- **Swagger/OpenAPI**: Built-in API documentation generator
- **Object Literals**: JavaScript-style `{ key: value }` syntax

---

## Installation

### Requirements
- Rust 1.70+ (for building)
- Clang/LLVM (optional, for native binary compilation)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/TheFahmi/cryo-lang.git
cd cryo-lang

# Build release binary
cargo build --release

# Verify installation
./target/release/cryo --version
# Output: Cryo Native v3.2.1

# (Optional) Copy to project root
cp target/release/cryo.exe cryo.exe   # Windows
cp target/release/cryo ./cryo         # Linux/Mac
```

### Using Docker

```bash
docker build -t cryo .
docker run --rm cryo ./cryo --version
```

---

## Quick Start

### Hello World

Create `hello.ar`:

```javascript
fn main() {
    print("Hello, Cryo!");
}
```

Run:

```bash
./cryo hello.ar
```

### Variables and Types

```javascript
fn main() {
    // Type inference
    let name = "Cryo";
    let version = 3;
    let pi = 3.14159;
    let active = true;
    
    // Explicit types
    let count: int = 42;
    let message: string = "Hello";
    
    // Arrays
    let numbers = [1, 2, 3, 4, 5];
    print(numbers[0]);  // 1
}
```

### Functions

```javascript
fn add(a: int, b: int) -> int {
    return a + b;
}

fn greet(name: string) {
    print("Hello, " + name + "!");
}

fn main() {
    let result = add(10, 20);
    print(result);  // 30
    greet("World");
}
```

### Control Flow

```javascript
fn main() {
    let x = 10;
    
    // If-else
    if (x > 5) {
        print("Greater");
    } else {
        print("Smaller");
    }
    
    // While loop
    let i = 0;
    while (i < 5) {
        print(i);
        i = i + 1;
    }
    
    // Match expression
    match x {
        0 => print("Zero"),
        1 => print("One"),
        _ => print("Other")
    }
}
```

---

## Language Guide

### Structs

```javascript
struct Point {
    x: int,
    y: int
}

struct Rectangle {
    origin: Point,
    width: int,
    height: int
}

fn main() {
    let p = Point { x: 10, y: 20 };
    print(p.x);  // 10
    
    let rect = Rectangle {
        origin: Point { x: 0, y: 0 },
        width: 100,
        height: 50
    };
}
```

### Traits and Generics

```javascript
trait Printable {
    fn toString(self) -> string;
}

struct Point { x: int, y: int }

impl Printable for Point {
    fn toString(self) -> string {
        return "Point(" + self.x + ", " + self.y + ")";
    }
}

fn printAny<T: Printable>(item: T) {
    print(item.toString());
}

fn main() {
    let p = Point { x: 5, y: 10 };
    printAny(p);  // Point(5, 10)
}
```

### Async/Await

```javascript
async fn fetchData(url: string) -> string {
    let response = await httpGet(url);
    return response.body;
}

async fn main() {
    let data = await fetchData("https://api.example.com/data");
    print(data);
}
```

### Error Handling with Defer

```javascript
fn readFile(path: string) -> string {
    let file = open(path);
    defer close(file);  // Always executed when scope ends
    
    let content = file.readAll();
    return content;
}
```

### Macros

```javascript
macro log(level, message) {
    print("[" + $level + "] " + $message);
}

macro unless(condition, body) {
    if (!$condition) {
        $body
    }
}

fn main() {
    log!("INFO", "Application started");
    
    let x = 5;
    unless!(x > 10, {
        print("x is not greater than 10");
    });
}
```

### FFI (Foreign Function Interface)

```javascript
extern "C" {
    fn puts(s: *i8) -> i32;
    fn malloc(size: usize) -> *void;
    fn free(ptr: *void);
}

fn main() {
    puts("Hello from C!");
}
```

---

## Standard Library

### Core Modules

| Module | Description |
|--------|-------------|
| `std` | Core functions (print, len, type, assert) |
| `math` | Mathematical operations (abs, pow, sqrt, sin, cos) |
| `string` | String manipulation (substr, split, trim, replace) |
| `array` | Array utilities (push, pop, map, filter, reduce) |
| `io` | File I/O operations |
| `json` | JSON parsing and serialization |

### Network & Web

| Module | Description |
|--------|-------------|
| `http` | HTTP client and server |
| `tcp` | Low-level TCP sockets |
| `websocket` | WebSocket client/server |

### Security

| Module | Description |
|--------|-------------|
| `crypto` | Hashing, encoding, encryption |
| `jwt` | JSON Web Token support |
| `bcrypt` | Password hashing |

### Database (Native Implementations)

| Module | Description |
|--------|-------------|
| `postgres_native` | PostgreSQL Wire Protocol v3.0 |
| `mysql_native` | MySQL/MariaDB with SHA1 auth |
| `redis` | Redis RESP protocol via TCP |

```javascript
// PostgreSQL example
import "postgres_native"
let conn = pgConnect("localhost", 5432, "user", "mydb");
pgQuery(conn, "SELECT * FROM users");
pgClose(conn);

// Redis example
let redis = @tcpConnect("localhost", 6379);
@tcpWrite(redis, "SET key value");
@tcpReadLine(redis);  // +OK
```

### Built-in Functions

```javascript
// I/O
print(value)              // Print to stdout
input(prompt)             // Read from stdin

// Type conversion
toString(value)           // Convert to string
toInt(value)              // Convert to integer

// Collections
len(collection)           // Get length
push(array, item)         // Add to array
pop(array)                // Remove last item

// Time
now()                     // Current Unix timestamp
sleep(ms)                 // Pause execution

// Utilities
uuid()                    // Generate UUID v4
env(key, default)         // Get environment variable
typeof(value)             // Get type name
```

---

## Performance

Cryo achieves near-C++ performance through LLVM compilation.

### Benchmark Results

Tested on Intel Xeon E5-2660 v4 @ 2.00GHz:

| Benchmark | C++ | Cryo | Rust | Go | Python |
|-----------|-----|-------|------|-----|--------|
| Fibonacci(35) | 35ms | **40ms** | 50ms | 65ms | 2800ms |
| Fibonacci(45) | 4.1s | 5.1s | 6.3s | 8.2s | 280s |
| Ackermann(3,11) | 136ms | 232ms | 261ms | 380ms | N/A |
| Sum Loop (1B) | 798ms | 0ms* | 1526ms | 890ms | 45000ms |

*LLVM optimizes constant loops at compile time

### Running Benchmarks

```bash
# Native Rust baseline (target: ~40ms for Fib35)
./cryo --native-bench 35

# Bytecode VM benchmark
./cryo --vm-bench 35

# Full comparison suite (requires Docker)
docker build -t cryo-bench .
docker run --rm cryo-bench
```

---

## CryoWeb Framework

A NestJS-inspired web framework for building REST APIs.

### Create New Project

```bash
./cryoweb-cli.sh new my-api
cd my-api
../cryo src/main.ar
```

### Project Structure

```
my-api/
├── src/
│   ├── main.ar              # Entry point
│   ├── app.module.ar        # Route registration
│   ├── config/
│   │   └── env.ar           # Environment config
│   ├── common/
│   │   ├── middleware/      # HTTP middleware
│   │   ├── guards/          # Auth guards
│   │   └── utils/           # Utility functions
│   └── modules/
│       ├── users/
│       │   ├── users.controller.ar
│       │   ├── users.service.ar
│       │   └── users.entity.ar
│       └── auth/
│           ├── auth.controller.ar
│           └── auth.service.ar
└── README.md
```

### Example Controller

```javascript
import { Controller, Get, Post } from "cryoweb";

@Controller("/users")
struct UsersController {
    service: UsersService
}

impl UsersController {
    @Get("/")
    fn list(self, req: Request) -> Response {
        let users = self.service.findAll();
        return json(users);
    }
    
    @Post("/")
    fn create(self, req: Request) -> Response {
        let user = self.service.create(req.body);
        return json(user, 201);
    }
}
```

### Web Built-in Functions

| Function | Description |
|----------|-------------|
| `env(key, default)` | Get environment variable |
| `bcryptHash(password)` | Hash password |
| `bcryptVerify(password, hash)` | Verify password |
| `jwtSign(payload, secret)` | Create JWT |
| `jwtVerify(token, secret)` | Verify JWT |
| `jsonParse(string)` | Parse JSON |
| `jsonStringify(value)` | Serialize to JSON |

### Swagger / OpenAPI Documentation

Generate interactive API documentation with built-in Swagger UI.

```javascript
import "swagger"

// Create API spec
let api = swaggerNew("My API", "1.0.0");
swaggerSetDescription(api, "REST API documentation");
swaggerAddServer(api, "http://localhost:8080", "Dev server");

// Define endpoint
let ep = endpointNew("GET", "/api/users/{id}", "Get user");
endpointParamPath(ep, "id", "User ID", "integer");
endpointResponse(ep, 200, "User found", "application/json");
endpointResponse(ep, 404, "Not found", "application/json");
swaggerAddEndpoint(api, ep);

// Generate OpenAPI JSON
let json = swaggerToJson(api);
```

**Run Swagger UI Server:**

```bash
./target/release/cryo.exe examples/swagger_server.ar
# Open: http://localhost:8888/docs
```

| Function | Description |
|----------|-------------|
| `swaggerNew(title, version)` | Create API specification |
| `swaggerAddServer(api, url, desc)` | Add server |
| `swaggerAddTag(api, name, desc)` | Add tag |
| `endpointNew(method, path, summary)` | Create endpoint |
| `endpointParamPath(ep, name, desc, type)` | Add path parameter |
| `endpointParamQuery(ep, name, desc, type, required)` | Add query parameter |
| `endpointResponse(ep, code, desc, contentType)` | Add response |
| `swaggerToJson(api)` | Generate OpenAPI 3.0 JSON |
| `swaggerUiHtml(json, title)` | Generate Swagger UI HTML |

---

## Tooling

### Language Server (LSP)

Full VS Code integration with:
- Syntax highlighting
- Autocomplete
- Go to definition
- Find references
- Error diagnostics

Install the Cryo extension from the `lsp/vscode-extension` directory.

### Package Manager (APM)

```bash
# Initialize new package
./apm.sh init my-package

# Install dependencies
./apm.sh install crypto
./apm.sh install http@2.0.0

# Run project
./apm.sh run
```

### Build Script

```bash
# Show help
./build.sh --help

# Run file
./build.sh run examples/hello.ar

# Compile to LLVM IR
./build.sh compile examples/fib.ar

# Compile to native binary
./build.sh native examples/fib.ar

# Run tests
./build.sh test

# Run benchmark
./build.sh bench 35
```

---

## Version History

| Version | Release | Key Features |
|---------|---------|--------------|
| v3.2.1 | 2026-01 | Native mode default, Decorators support, AI agent documentation |
| v3.0.0 | 2025-12 | LSP, Debugger, Bytecode VM, LLVM compilation |
| v2.24.0 | 2025-11 | Macros, CryoWeb CLI, Crypto module |
| v2.23.0 | 2025-10 | Defer statement |
| v2.22.0 | 2025-09 | Optimization pass (constant folding) |
| v2.21.0 | 2025-08 | Garbage collection |
| v2.20.0 | 2025-07 | FFI, Traits system |
| v2.19.0 | 2025-06 | WebAssembly target |
| v2.18.0 | 2025-05 | Async/Await |
| v2.10.0 | 2025-03 | Package Manager (APM) |
| v2.0.0 | 2025-01 | Self-hosting compiler |
| v1.0.0 | 2024-10 | Initial release |

---

## Documentation

| Document | Description |
|----------|-------------|
| [AGENTS.md](./AGENTS.md) | Guide for AI coding assistants |
| [ROADMAP.md](./ROADMAP.md) | Development roadmap |
| [docs/running_native.md](./docs/running_native.md) | Native compilation guide |
| [docs/stdlib_reference.md](./docs/stdlib_reference.md) | Standard library reference |
| [docs/traits_design.md](./docs/traits_design.md) | Traits system design |
| [docs/async_design.md](./docs/async_design.md) | Async/await design |
| [docs/wasm_design.md](./docs/wasm_design.md) | WebAssembly compilation |
| [docs/ffi_design.md](./docs/ffi_design.md) | FFI design |
| [docs/macros_design.md](./docs/macros_design.md) | Macro system design |

---

## Project Status

| Component | Status |
|-----------|--------|
| Compiler | Stable |
| Interpreter | Stable |
| Standard Library | Stable |
| LSP | Stable |
| Package Manager | Stable |
| WebAssembly | Stable |
| FFI | Stable |
| Documentation | Complete |

---

## License

MIT License - See [LICENSE](./LICENSE) for details.

---

## Contributing

Contributions are welcome! Please read the contributing guidelines before submitting pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
