# Argon Roadmap

**Current Version: v3.2.1** (2026-01-01)

Argon is evolving rapidly. This document outlines the current state and future milestones for the language.

---

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

## ✅ Phase 4: Enterprise Features (v3.0 - v3.1) [COMPLETED]
Focus on ecosystem and enterprise readiness.

### ✅ Standard Library Expansion [v3.0.0]
- [x] `crypto` module (randomBytes, UUID, hash, HMAC, base64)
- [x] `http` module (Router, Request/Response, CORS, cookies)
- [x] `sql` module (in-memory database with CRUD operations)
- [x] `async` module (Future, async utilities)

### ✅ Concurrency [v3.0.1 - v3.1.0]
- [x] Channel-based communication (`channel` module)
- [x] Worker-based parallelism (`worker` module)
- [x] Spawn/Join semantics
- [x] Work-stealing queues
- [x] Pipeline patterns
- [x] **True OS Threading** (native `std::thread`)
  - [x] `thread_spawn()` / `thread_join()` built-ins
  - [x] `channel_new()` / `channel_send()` / `channel_recv()` built-ins
  - [x] Non-blocking `channel_try_recv()` and `channel_recv_timeout()`

### ✅ Tooling
- [x] Documentation generator (`argondoc`)
- [x] Code formatter (`argonfmt`)

---

## ✅ Phase 5: Ecosystem (v3.1.1 - v3.2.1) [COMPLETED]
Building a thriving developer ecosystem.

### ✅ Web Framework (`argonweb`) [v3.1.1]
- [x] Express-like HTTP server
- [x] NestJS-style architecture
- [x] Router with route parameters (`:id`)
- [x] Query string parsing
- [x] Middleware pipeline
- [x] Built-in middleware (Logger, CORS, JSON parser)
- [x] Response helpers (responseOk, responseError, etc.)
- [x] Context API (json, html, redirect, params)
- [x] Template Engine (EJS/Jinja2-style)
  - [x] Variable interpolation `{{ name }}`
  - [x] Conditionals `{% if %}...{% endif %}`
  - [x] Loops `{% for item in items %}`
  - [x] Includes `{% include "partial" %}`
  - [x] Filters `{{ name | upper }}`
  - [x] Layout inheritance `{% extends "base" %}`
- [x] WebSocket support
  - [x] WebSocket server
  - [x] Frame encoding/decoding
  - [x] Handshake protocol
  - [x] Broadcast messaging

### ✅ Native Database Connectors [v3.2.0 - v3.2.1]
All database clients are implemented **100% in native Argon** without external library dependencies.

#### ✅ PostgreSQL (`stdlib/postgres_native.ar`)
- [x] Wire Protocol v3.0 implementation
- [x] Trust auth mode support
- [x] TCP connection management
- [x] Binary protocol message parsing
- [x] `pg_connect()`, `pg_query()`, `pg_close()` API
- [x] Full CRUD operations (CREATE, INSERT, UPDATE, DELETE, SELECT)
- [x] Transaction support (BEGIN, COMMIT, ROLLBACK)

#### ✅ MySQL/MariaDB (`stdlib/mysql_native.ar`)
- [x] MySQL Wire Protocol implementation
- [x] **SHA1-based mysql_native_password authentication**
- [x] Auth switch protocol handling
- [x] Full handshake parsing (scramble extraction)
- [x] `mysql_connect()`, `mysql_query()`, `mysql_close()` API
- [x] Full CRUD operations

#### ✅ Redis (TCP-based)
- [x] RESP Protocol implementation
- [x] TCP socket-based connection
- [x] String commands (GET, SET, INCR, DEL)
- [x] List commands (LPUSH, LLEN, LRANGE)
- [x] Key expiry (EXPIRE, TTL)

### ✅ New Built-in Functions [v3.2.1]

#### TCP Client Functions
| Function | Description |
|----------|-------------|
| `@tcp_connect(host, port)` | Connect to remote server |
| `@tcp_write(conn, data)` | Write string with CRLF |
| `@tcp_read_line(conn)` | Read until newline |
| `@tcp_write_raw(conn, bytes)` | Write raw byte array |
| `@tcp_read_raw(conn, count)` | Read exact bytes as array |
| `@tcp_read_available(conn)` | Read all available bytes |
| `@argon_socket_close(conn)` | Close connection |

#### Crypto Functions
| Function | Description |
|----------|-------------|
| `@sha1(string)` | SHA1 hash → hex string |
| `@sha1_bytes(data)` | SHA1 hash → 20-byte array |
| `@xor_bytes(a, b)` | XOR two byte arrays |
| `@concat_bytes(a, b)` | Concatenate byte arrays |

#### Encoding Functions
| Function | Description |
|----------|-------------|
| `@chr(n)` | Integer to character |
| `@ord(s)` | Character to integer |
| `@bytes_to_string(arr)` | Byte array to string |
| `@string_to_bytes(s)` | String to byte array |

---

## � Phase 6: Package Ecosystem (v3.3) [IN PROGRESS]
Building the package ecosystem.

### Package Registry (`apm.argon.dev`)
- [ ] Web portal for package discovery
- [ ] Package publishing workflow (`apm publish`)
- [ ] Version management & semver
- [ ] Dependency resolution algorithm
- [ ] Private package support
- [ ] Package statistics & downloads

### Package Format
- [ ] `argon.toml` manifest file
- [ ] Lock file (`argon.lock`)
- [ ] Workspace support (monorepo)
- [ ] Build scripts

---

## 🔮 Phase 7: Performance (v3.4)
Maximizing runtime performance.

### JIT Compilation
- [ ] Compile hot paths to native code
- [ ] Method inlining
- [ ] Type specialization
- [ ] Trace-based JIT

### Async I/O
- [ ] Non-blocking socket I/O
- [ ] Event loop (`libuv` style)
- [ ] `async`/`await` for I/O
- [ ] Concurrent request handling

### Optimizations
- [ ] SIMD operations
- [ ] Inline caching
- [ ] Escape analysis
- [ ] Dead code elimination

---

## 🔮 Phase 8: Security (v3.5)
Enterprise-grade security features.

### TLS/SSL Support
- [ ] TLS 1.3 for database connections
- [ ] HTTPS server support
- [ ] Certificate management
- [ ] SNI support

### Authentication
- [ ] bcrypt password hashing
- [ ] Argon2 password hashing
- [ ] JWT token creation/validation
- [ ] OAuth2 client

### Security Utilities
- [ ] Constant-time comparison
- [ ] Secure random generation
- [ ] CSRF token generation
- [ ] Rate limiting utilities

---

## 🔮 Phase 9: Cloud Native (v3.6)
Ready for cloud deployment.

### Container Support
- [ ] Optimized Alpine Docker image
- [ ] Multi-stage builds
- [ ] Health check endpoints
- [ ] Graceful shutdown

### Observability
- [ ] OpenTelemetry integration
- [ ] Structured logging (JSON)
- [ ] Metrics collection
- [ ] Distributed tracing

### Cloud Services
- [ ] AWS SDK bindings
- [ ] GCP SDK bindings
- [ ] Azure SDK bindings
- [ ] S3-compatible storage

---

## 🔮 Phase 10: Advanced Features (v4.0)
Next-generation language features.

### Type System
- [ ] Optional static typing
- [ ] Generics (`fn map<T>(arr: [T], f: fn(T) -> T)`)
- [ ] Union types (`int | string`)
- [ ] Null safety (`?` operator)

### Metaprogramming
- [ ] Compile-time macros
- [ ] Decorators with code generation
- [ ] Reflection API
- [ ] Source maps

### Interoperability
- [ ] C ABI compatibility
- [ ] Python bindings
- [ ] Node.js N-API bindings
- [ ] gRPC support

---

## Docker Database Setup

Quick start for database testing:

```bash
# Start all databases
docker-compose -f docker-compose.db.yml up -d

# Containers started:
# ├── PostgreSQL : localhost:5432 (user: argon, db: argondb, trust auth)
# ├── MariaDB    : localhost:3307 (user: argon, pass: argon123, db: argondb)
# └── Redis      : localhost:6379 (no auth)

# Run database tests
./target/release/argon.exe examples/test_redis_real.ar      # ✅ All pass
./target/release/argon.exe examples/test_postgres_real.ar   # ✅ All pass
./target/release/argon.exe examples/test_mysql_real.ar      # ✅ All pass

# Stop databases
docker-compose -f docker-compose.db.yml down
```

---

## Release History

| Version | Date | Feature | Status |
|---------|------|---------|--------|
| v1.0.0 | 2025-01 | Initial release | ✅ |
| v2.0.0 | 2025-03 | Bytecode VM | ✅ |
| v2.25.0 | 2025-06 | Performance & Stdlib | ✅ |
| v2.26.0 | 2025-07 | Traits & Interfaces | ✅ |
| v2.27.0 | 2025-08 | FFI Support | ✅ |
| v2.28.0 | 2025-09 | Garbage Collector | ✅ |
| v2.29.0 | 2025-10 | LSP & Debugger | ✅ |
| v3.0.0 | 2025-11 | Enterprise Stdlib | ✅ |
| v3.0.1 | 2025-11 | Channels & Workers | ✅ |
| v3.1.0 | 2025-12 | True OS Threading | ✅ |
| v3.1.1 | 2025-12 | ArgonWeb Framework | ✅ |
| v3.2.0 | 2025-12 | Native Database Connectors | ✅ |
| v3.2.1 | 2026-01 | SHA1 Auth & Binary Protocols | ✅ Current |
| v3.3.0 | 2026-Q1 | Package Registry | � In Progress |
| v3.4.0 | 2026-Q2 | JIT & Async I/O | 🔮 Planned |
| v3.5.0 | 2026-Q3 | TLS/SSL & Security | 🔮 Planned |
| v3.6.0 | 2026-Q4 | Cloud Native | 🔮 Planned |
| v4.0.0 | 2027-Q1 | Static Types & Generics | 🔮 Planned |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to Argon.

## License

Argon is open source under the MIT License. See [LICENSE](LICENSE).

---

*Last updated: 2026-01-01*
