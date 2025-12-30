# Argon Programming Language (v2.15.0)
![Argon Logo](logo.png)

Argon is a high-performance, **self-hosted** systems programming language that compiles directly to LLVM IR and Native Machine Code.

## ✨ Highlights
- **Self-Hosted**: Compiler written in Argon itself (`self-host/compiler.ar`)
- **Verified**: Stage 1 (self-compiled) produces identical output when compiling itself
- **Native Backend**: Uses LLVM for optimized native binary generation
- **Generic Types**: `struct Box<T>`, `fn map<T>(...)` syntax support (v2.15.0)
- **REPL**: Interactive mode for quick experimentation (v2.14.0)
- **IDE Support**: VS Code extension with full Language Server Protocol (v2.0.0)
- **Package Manager**: APM with registry, git deps, and lock files (v2.10.0)
- **Standard Library**: 19 modules (math, string, array, collections, etc)
- **Methods**: Support for methods on structs (v2.5.0)
- **Enums**: Enum types with pattern matching (v2.6.0)
- **Modules**: Import system for code organization (v2.7.0)
- **Structs**: Full struct support with definitions, instantiation, and field access (v2.4.0)
- **Networking**: Built-in TCP Socket support (v2.1.0)
- **Multi-threading**: Atomics, Mutex, and Thread support (v2.3.0)
- **High Performance**: Tagged pointer optimization for fast integer arithmetic

## Quick Start

### Using APM (Recommended)
```bash
# Create new project
./apm.sh init my-app
cd my-app

# Install dependencies
./apm.sh install

# Build and run
./apm.sh run
```

### Using Docker Toolchain
```bash
# Create MVC Backend skeleton
./argon new my_api

# Run (starts HTTP server on port 3000)
./argon run my_api

# Build only
./argon build my_api
```

## 📦 Package Manager (APM)

### Commands
| Command | Description |
|---------|-------------|
| `apm init <name>` | Create new project |
| `apm build` | Compile project |
| `apm run` | Build and run |
| `apm install` | Install dependencies |
| `apm add <pkg>` | Add from registry |
| `apm add user/repo --git` | Add from GitHub |
| `apm add ../lib --path` | Add local dependency |
| `apm search` | List available packages |
| `apm publish` | Publish package |

### Example: Adding Dependencies
```bash
# From GitHub
apm add TheFahmi/json-utils --git

# Local path
apm add ../my-lib --path

# Then import in your code
import "deps/json-utils/lib/lib.ar";
```

## Project Structure
```text
my_app/
├── argon.toml       # Project manifest
├── argon.lock       # Lock file (generated)
├── src/main.ar      # Entry point
├── lib/             # Library code
├── deps/            # Dependencies (installed)
└── tests/           # Unit tests
```

## Language Features

### Modules & Imports (v2.7.0)
```javascript
// math_utils.ar
fn math_add(a, b) { return a + b; }
fn math_mul(a, b) { return a * b; }

// main.ar - Import all
import "math_utils.ar";

// Or selective import
import {math_add} from "math_utils.ar";

fn main() {
    print(math_add(5, 3)); // 8
}
```

### Enums & Pattern Matching (v2.6.0)
```javascript
enum Result {
    Ok(val),
    Err(msg)
}

fn main() {
    let res = Ok(42);
    match res {
        Ok(n) => {
            print("Value: " + n);
        },
        Err(e) => {
            print("Error: " + e);
        }
    }
}
```

### Methods (v2.5.0)
```javascript
struct Circle {
    radius: int
}

impl Circle {
    fn area(self) {
        return 3 * self.radius * self.radius;
    }
}

fn main() {
    let c = Circle { radius: 10 };
    print(c.area()); // 300
}
```

### Structs (v2.4.0)
```javascript
struct Point {
    x: int,
    y: int
}

fn main() {
    let p = Point { x: 10, y: 20 };
    print(p.x);  // 10
    print(p.y);  // 20
}
```

### Standard Library (v2.7.2)
| Module | Functions |
|--------|-----------|
| math | abs, min, max, pow, sqrt, gcd, lcm |
| string | trim, split, join, replace, to_upper |
| array | push, pop, map, filter, reduce |
| datetime | now, format, is_leap_year |
| regex | match, replace, is_email, glob |
| process | command, spawn, execute |
| crypto | hash, base64, hex |
| ... | 18 modules total |

### Networking (v2.1.0)
| Function | Description |
|----------|-------------|
| `argon_listen(port)` | Bind to 0.0.0.0:port |
| `argon_accept(server)` | Accept connection |
| `argon_socket_read(client)` | Read data |
| `argon_socket_write(client, data)` | Write data |
| `argon_socket_close(client)` | Close connection |

### Multi-threading (v2.3.0)
| Function | Description |
|----------|-------------|
| `argon_thread_spawn(fn)` | Spawn thread |
| `argon_thread_join(id)` | Wait for thread |
| `argon_mutex_new()` | Create mutex |
| `argon_atomic_new(v)` | Create atomic |
| `argon_sleep(ms)` | Sleep |

## Requirements
- **Docker**: The toolchain runs inside the `argon-toolchain` image.
- **Node.js** (optional): For LSP/VS Code extension

## Version History
- **v2.15.0**: Generic type syntax support (`struct Box<T>`, `fn map<T>`)
- **v2.14.1**: REPL interactive mode, system(), input() functions
- **v2.14.0**: Full LSP implementation (Navigation, Editing, Autocomplete)
- **v2.13.1**: Fixed chained field access and array indexing bugs
- **v2.13.0**: Fixed NOT operator and runtime functions
- **v2.12.0**: LSP basics and VS Code extension
- **v2.11.0**: Collections module (Optional, Pair, Range, Stack, Queue)
- **v2.10.0**: Package Manager with Central Registry
- **v2.9.0**: APM with git dependencies and lock files
- **v2.8.0**: APM basics (init, build, run, local deps)
- **v2.7.2**: Added process and regex modules (18 stdlib modules)
- **v2.7.0**: Module system / imports
- **v2.6.0**: Enum types with pattern matching
- **v2.5.0**: Methods on structs
- **v2.4.0**: Struct support
- **v2.3.0**: Multi-threading support
- **v2.2.0**: Verified Self-Hosting
- **v2.1.0**: Native Networking
- **v1.0.0**: Self-Hosting Compiler

## Roadmap
- [x] Self-Hosting Compiler ✅
- [x] Networking & Multi-threading ✅
- [x] Structs, Methods, Enums ✅
- [x] Module system / imports ✅
- [x] Standard library (19 modules) ✅
- [x] Package Manager (APM) ✅
- [x] LSP (Language Server Protocol) ✅
- [x] REPL (interactive mode) ✅
- [ ] Generic types (`Array<T>`)
- [ ] Debugger support
