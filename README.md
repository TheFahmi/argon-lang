# Argon Programming Language (v2.5)
![Argon Logo](logo.png)

Argon is a high-performance, **self-hosted** systems programming language that compiles directly to LLVM IR and Native Machine Code.

## ✨ Highlights
- **Self-Hosted**: Compiler written in Argon itself (`self-host/compiler.ar`)
- **Verified**: Stage 1 (self-compiled) produces identical output when compiling itself
- **Native Backend**: Uses LLVM for optimized native binary generation
- **Methods**: Support for methods on structs (v2.5)
- **Structs**: Full struct support with definitions, instantiation, and field access (v2.4)
- **Networking**: Built-in TCP Socket support (v2.1)
- **Multi-threading**: Atomics, Mutex, and Thread support (v2.3)
- **High Performance**: Tagged pointer optimization for fast integer arithmetic

## Quick Start
Argon Toolchain uses Docker to ensure a consistent build environment.

### 1. Create a New Project
Generate a full MVC Backend skeleton:
```bash
# Git Bash / Linux / Mac
./argon new my_api

# Windows CMD
argon new my_api
```

### 2. Run
Compile and run the project immediately (starts HTTP server on port 3000):
```bash
./argon run my_api
```

### 3. Build only
Produces a native executable inside `dist/`:
```bash
./argon build my_api
```

## Project Structure
An Argon MVC project looks like this:
```text
my_api/
├── dist/            # Compiled Binaries & LLVM IR
├── src/
│   ├── main.ar      # Entry Point
│   ├── server.ar    # HTTP Server Loop
│   ├── controllers/ # Request Handlers
│   ├── services/    # Business Logic
│   └── models/      # Data Models
└── tests/           # Unit Tests
```

## Language Features

### Methods (v2.5)
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

### Structs (v2.4)
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

### Networking (v2.1)
| Function | Description |
|----------|-------------|
| `argon_listen(port)` | Bind to 0.0.0.0:port, returns server ID |
| `argon_accept(server)` | Accept connection, returns client ID |
| `argon_socket_read(client)` | Read data from client |
| `argon_socket_write(client, data)` | Write string to client |
| `argon_socket_close(client)` | Close connection |

### Multi-threading (v2.3)
| Function | Description |
|----------|-------------|
| `argon_thread_spawn(fn)` | Spawn thread with function |
| `argon_thread_join(id)` | Wait for thread completion |
| `argon_mutex_new()` | Create mutex |
| `argon_mutex_lock(id)` | Lock mutex |
| `argon_mutex_unlock(id)` | Unlock mutex |
| `argon_atomic_new(v)` | Create atomic integer |
| `argon_atomic_load(id)` | Load atomic value |
| `argon_atomic_store(id, v)` | Store atomic value |
| `argon_atomic_add(id, v)` | Atomic add, returns old value |
| `argon_atomic_cas(id, exp, new)` | Compare-and-swap |
| `argon_sleep(ms)` | Sleep for milliseconds |

## Requirements
- **Docker**: The toolchain runs inside the `argon-toolchain` image.

## Version History
- **v2.5**: Methods on structs (`p.get_x()`, `p.sum()`)
- **v2.4**: Struct support (definitions, instantiation, field access)
- **v2.3**: Multi-threading support (Atomics, Mutex, Sleep)
- **v2.2**: Verified Self-Hosting (Stage 1 == Stage 2)
- **v2.1**: Native Networking (TCP Sockets)
- **v1.0**: Self-Hosting Compiler

## Roadmap
- [x] Methods on structs (`p.method()`) ✅
- [ ] Enum types with pattern matching
- [ ] Generic types (`Array<T>`)
- [ ] Module system / imports
- [ ] Standard library
