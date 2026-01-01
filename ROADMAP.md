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

## ✅ Phase 4: Enterprise Features (v3.0+) [COMPLETED]
Focus on ecosystem and enterprise readiness.

### ✅ Standard Library Expansion
- [x] `crypto` module (randomBytes, UUID, hash, HMAC, base64)
- [x] `http` module (Router, Request/Response, CORS, cookies)
- [x] `sql` module (in-memory database with CRUD operations)
- [x] `async` module (Future, async utilities)

### ✅ Concurrency
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
- [ ] Package registry (apm.argon.dev)

---

## ✅ Phase 5: Ecosystem (v3.1 - v3.2) [COMPLETED]
Building a thriving developer ecosystem.

### ✅ Web Framework (`argonweb`)
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

### ✅ Database Connectors (Native Argon Implementation)
All database clients are implemented **100% in native Argon** without external library dependencies.

#### ✅ PostgreSQL (`stdlib/postgres_native.ar`)
- [x] Wire Protocol v3.0 implementation
- [x] Trust auth mode support
- [x] TCP connection management
- [x] Binary protocol parsing
- [x] `pg_connect()`, `pg_query()`, `pg_close()`
- [x] Query execution (CREATE, INSERT, UPDATE, DELETE, SELECT)

#### ✅ MySQL/MariaDB (`stdlib/mysql_native.ar`)
- [x] MySQL Wire Protocol implementation
- [x] **SHA1-based mysql_native_password authentication**
- [x] Auth switch handling
- [x] Full handshake parsing (scramble extraction)
- [x] `mysql_connect()`, `mysql_query()`, `mysql_close()`
- [x] Query execution (USE, CREATE, INSERT, UPDATE, DELETE, SELECT)

#### ✅ Redis (`examples/test_redis_real.ar`)
- [x] RESP Protocol implementation
- [x] TCP socket-based connection
- [x] String commands (GET, SET, INCR)
- [x] List commands (LPUSH, LLEN)
- [x] Key expiry (EXPIRE, TTL)
- [x] Key deletion (DEL)

### ✅ New Built-in Functions (v3.2)
- [x] TCP Client Functions:
  - `@tcp_connect(host, port)` - Outbound TCP connection
  - `@tcp_write(conn, data)` - Write with CRLF
  - `@tcp_read_line(conn)` - Read until newline
  - `@tcp_write_raw(conn, bytes)` - Write raw bytes
  - `@tcp_read_raw(conn, count)` - Read raw bytes
  - `@tcp_read_available(conn)` - Read available data
- [x] Crypto Functions:
  - `@sha1(string)` - SHA1 hash (hex)
  - `@sha1_bytes(data)` - SHA1 hash (20-byte array)
  - `@xor_bytes(a, b)` - XOR byte arrays
  - `@concat_bytes(a, b)` - Concatenate byte arrays
- [x] Encoding Functions:
  - `@chr(n)` - Int to character
  - `@ord(s)` - Character to int
  - `@bytes_to_string(arr)` - Byte array to string
  - `@string_to_bytes(s)` - String to byte array

---

## 🔮 Phase 6: Future (v3.3+)
Next steps for Argon.

### Package Registry
- [ ] `apm.argon.dev` web portal
- [ ] Package publishing workflow
- [ ] Version management & semver
- [ ] Dependency resolution

### Additional Databases
- [ ] SQLite driver (native)
- [ ] MongoDB client
- [ ] Connection pooling

### Performance
- [ ] JIT compilation
- [ ] Async I/O (non-blocking)
- [ ] SIMD optimizations

### Security
- [ ] TLS/SSL support for connections
- [ ] Password hashing utilities (bcrypt, argon2)
- [ ] JWT token utilities

---

## Docker Database Setup

Quick start for database testing:

```bash
# Start all databases
docker-compose -f docker-compose.db.yml up -d

# Containers:
# - PostgreSQL: localhost:5432 (user: argon, db: argondb)
# - MariaDB:    localhost:3307 (user: argon, db: argondb)
# - Redis:      localhost:6379

# Test database connections
./target/release/argon.exe examples/test_redis_real.ar
./target/release/argon.exe examples/test_postgres_real.ar
./target/release/argon.exe examples/test_mysql_real.ar
```

---

## Release Schedule
| Version | Feature | Status |
|---------|---------|--------|
| v2.25.0 | Performance & Stdlib | ✅ |
| v2.26.0 | Traits & Interfaces | ✅ |
| v2.27.0 | FFI Support | ✅ |
| v2.28.0 | Garbage Collector | ✅ |
| v2.29.0 | LSP & Debugger | ✅ |
| v3.0.0  | Enterprise Stdlib | ✅ |
| v3.0.1  | Concurrency (channel, worker) | ✅ |
| v3.1.0  | True OS Threading | ✅ |
| v3.1.1  | ArgonWeb Framework | ✅ |
| v3.2.0  | Native Database Connectors | ✅ (Current) |
| v3.2.1  | SHA1 Auth & Binary Protocols | ✅ (Current) |
| v3.3.0  | Package Registry | 🔮 Next |
| v3.4.0  | TLS/SSL Support | 🔮 Future |

---

*Last updated: 2026-01-01*
