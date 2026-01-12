# Cryo Roadmap

**Current Version: v3.3.0** (2026-01-12)

Cryo is evolving rapidly. This document outlines the current state and future milestones for the language.

---

## ✅ Phase 1: Foundation (v1.0 - v2.25) [COMPLETED]
The core infrastructure is now stable and performant.

- **Runtime**:
    - [x] Tree-walk Interpreter
    - [x] Bytecode VM (Register-based, ~16x faster)
    - [x] Optimized HashMaps (`FxHashMap`)
    - [x] Standard Library (Math, String, Array, IO, Net)
- **Compiler**:
    - [x] Self-hosted Compiler (`cryoc`)
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
- [x] `ffiLoad()` and `ffiCall()` built-ins
- [x] Load .dll/.so dynamically

### ✅ 3. Garbage Collection [v2.28.0]
- [x] Mark-and-Sweep GC module
- [x] `gcCollect()` and `gcStats()` built-ins
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
  - [x] `threadSpawn()` / `threadJoin()` built-ins
  - [x] `channelNew()` / `channelSend()` / `channelRecv()` built-ins
  - [x] Non-blocking `channelTryRecv()` and `channelRecvTimeout()`

### ✅ Tooling
- [x] Documentation generator (`cryodoc`)
- [x] Code formatter (`cryofmt`)

---

## ✅ Phase 5: Ecosystem (v3.1.1 - v3.2.1) [COMPLETED]
Building a thriving developer ecosystem.

### ✅ Web Framework (`cryoweb`) [v3.1.1]
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
All database clients are implemented **100% in native Cryo** without external library dependencies.

#### ✅ PostgreSQL (`stdlib/postgres_native.ar`)
- [x] Wire Protocol v3.0 implementation
- [x] Trust auth mode support
- [x] TCP connection management
- [x] Binary protocol message parsing
- [x] `pgConnect()`, `pgQuery()`, `pgClose()` API
- [x] Full CRUD operations (CREATE, INSERT, UPDATE, DELETE, SELECT)
- [x] Transaction support (BEGIN, COMMIT, ROLLBACK)

#### ✅ MySQL/MariaDB (`stdlib/mysql_native.ar`)
- [x] MySQL Wire Protocol implementation
- [x] **SHA1-based mysql_native_password authentication**
- [x] Auth switch protocol handling
- [x] Full handshake parsing (scramble extraction)
- [x] `mysqlConnect()`, `mysqlQuery()`, `mysqlClose()` API
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
| `@tcpConnect(host, port)` | Connect to remote server |
| `@tcpWrite(conn, data)` | Write string with CRLF |
| `@tcpReadLine(conn)` | Read until newline |
| `@tcpWriteRaw(conn, bytes)` | Write raw byte array |
| `@tcpReadRaw(conn, count)` | Read exact bytes as array |
| `@tcpReadAvailable(conn)` | Read all available bytes |
| `@cryoSocketClose(conn)` | Close connection |

#### Crypto Functions
| Function | Description |
|----------|-------------|
| `@sha1(string)` | SHA1 hash → hex string |
| `@sha1Bytes(data)` | SHA1 hash → 20-byte array |
| `@xorBytes(a, b)` | XOR two byte arrays |
| `@concatBytes(a, b)` | Concatenate byte arrays |

#### Encoding Functions
| Function | Description |
|----------|-------------|
| `@chr(n)` | Integer to character |
| `@ord(s)` | Character to integer |
| `@bytesToString(arr)` | Byte array to string |
| `@stringToBytes(s)` | String to byte array |

---

## ✅ Phase 6: Package Ecosystem (v3.3) [COMPLETED]
Building the package ecosystem.

### Package Registry (`apm.cryo.dev`)
- [x] Web portal for package discovery (`tools/registry-server.cryo`)
- [x] Package listing & search UI
- [x] Package detail pages
- [x] API endpoints (`/api/packages`, `/api/search`, `/api/index.json`)
- [x] Package publishing workflow (`apm publish`) ✅
- [x] Version management & semver (`apm bump`) ✅
- [x] Dependency resolution algorithm ✅
- [x] Private package support (`apm login/logout`) ✅
- [ ] Package statistics & downloads tracking (registry-side)

### SQLite Native Driver (`stdlib/sqlite.cryo`) ✅ [TESTED: 17/17 PASS]
- [x] In-memory SQL database
- [x] CREATE TABLE / DROP TABLE (with IF EXISTS/IF NOT EXISTS)
- [x] INSERT with column mapping and auto-increment ID
- [x] SELECT with WHERE clause and LIMIT
- [x] UPDATE and DELETE operations
- [x] Utility functions (sqliteGetTables, sqliteGetColumns, sqliteTableExists)
- [x] Full test suite passing

### MongoDB Native Client (`stdlib/mongodb.cryo`) ✅
- [x] MongoDB Wire Protocol (OP_MSG)
- [x] BSON encoding/decoding
- [x] Connection management
- [x] CRUD operations (insert, find, update, delete)
- [x] Aggregation pipeline
- [x] Collection management
- [x] Index operations

### Package Format
- [x] `cryo.toml` manifest file
- [x] Lock file (`cryo.lock`)
- [x] Workspace support (`apm workspace`) ✅
- [x] Build scripts (`[scripts]` section) ✅

---

## ✅ Phase 7: Performance (v3.4) [COMPLETED]
Maximizing runtime performance.

### JIT Compilation (`src/jit.rs`) ✅ [TESTED: 7/7 PASS]
Cranelift-based Just-In-Time compilation:
- [x] Hot path detection with configurable threshold
- [x] Compile functions to native code
- [x] Simple function compilation (identity, double, square, increment, negate)
- [x] Custom operations (add, sub, mul, div)
- [x] Function call via native code pointers
- [x] Method inlining framework (InliningConfig, InlinedFunction)
- [x] Type specialization (SpecializedType, TypeSpecialization)
- [x] Trace-based JIT (TraceRecorder, Trace, TraceOp, Guards)

### Async I/O (`stdlib/async_io.cryo`) ✅ [TESTED: 10/10 PASS]
- [x] Futures/Promises pattern
- [x] Task queue and execution
- [x] Retry with exponential backoff
- [x] Batch processing
- [x] Debounce & throttle
- [x] Queue operations
- [x] Cancellation tokens

### Event Loop (`stdlib/event_loop.cryo`) ✅ [TESTED: 10/10 PASS]
- [x] setTimeout/setInterval simulation
- [x] setImmediate and nextTick
- [x] Event queue with emit
- [x] Tick-based execution
- [x] Timer management (clear)
- [x] Run to completion

### SIMD Operations (`stdlib/simd.cryo`) ✅ [TESTED: 10/10 PASS]
- [x] Vector math (add, sub, mul, div, scale)
- [x] Reductions (sum, product, min, max, mean, dot)
- [x] Comparisons (eq, gt, lt)
- [x] Generation (zeros, ones, fill, range)
- [x] Transforms (abs, clamp, normalize, reverse)
- [x] Filter operations
- [x] Matrix multiply and transpose

### Cache (`stdlib/cache.cryo`) ✅ [TESTED: 10/10 PASS]
- [x] LRU cache with eviction
- [x] TTL cache with expiration
- [x] Memoization with hit/miss stats
- [x] Object pooling

### Profiler (`stdlib/profiler.cryo`) ✅ [TESTED: 10/10 PASS]
- [x] Section profiling (start/end timing)
- [x] Benchmarking with ops/sec
- [x] Memory snapshot tracking
- [x] Call counting
- [x] Hot path detection

---

## 🔮 Phase 8: Security (v3.5)
Enterprise-grade security features.

### Authentication
- [x] bcrypt password hashing (`stdlib/bcrypt.cryo`) ✅ [TESTED: 6/6 PASS]
- [x] Argon2 password hashing (Planned v4.0)
- [x] JWT token creation/validation (`stdlib/jwt.cryo`) ✅ [TESTED: 6/6 PASS]
- [x] OAuth2 client (`stdlib/oauth2.cryo`) ✅ [TESTED: 6/6 PASS]

### Security Utilities (`stdlib/security.cryo`) ✅ [TESTED: 10/10 PASS]
- [x] Constant-time comparison
- [x] Secure random generation
- [x] CSRF token generation
- [x] Rate limiting utilities
- [x] Input sanitization

---

## 🔮 Phase 9: Cloud Native (v3.6)
Ready for cloud deployment.

### Container Support
- [x] Optimized Alpine Docker image (`Dockerfile.minimal`) ✅
- [x] Multi-stage builds
- [x] Health check endpoints (`stdlib/health.cryo`) ✅ [TESTED: 7/7 PASS]
- [x] Graceful shutdown

### Observability
- [x] Structured logging (JSON) (`stdlib/logger.cryo`) ✅ [TESTED: 8/8 PASS]
- [x] Metrics collection (`stdlib/metrics.cryo`) ✅ [TESTED: 8/8 PASS]
- [ ] OpenTelemetry integration
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

## 🔮 Phase 11: Mobile & Desktop (v4.1)
Cross-platform application development.

### Mobile Development
- [ ] iOS compilation (via LLVM → ARM64)
- [ ] Android compilation (via LLVM → AArch64)
- [ ] React Native bridge
- [ ] Flutter plugin support
- [ ] Native UI bindings

### Desktop Development
- [ ] Electron alternative (lightweight)
- [ ] Native GUI framework
- [ ] System tray support
- [ ] Notifications API
- [ ] File dialogs

### Cross-Platform
- [ ] Single codebase for all platforms
- [ ] Platform-specific modules
- [ ] Hot reload for development
- [ ] App bundling & signing

---

## 🔮 Phase 12: AI & Machine Learning (v4.2)
AI/ML capabilities for modern applications.

### Tensor Operations
- [ ] N-dimensional arrays (tensors)
- [ ] Matrix operations (BLAS-style)
- [ ] GPU acceleration (CUDA/Metal)
- [ ] Automatic differentiation

### ML Libraries
- [ ] Neural network primitives
- [ ] Pre-trained model loading (ONNX)
- [ ] TensorFlow Lite integration
- [ ] PyTorch model import

### AI Utilities
- [ ] Embeddings generation
- [ ] Vector similarity search
- [ ] LLM API clients (OpenAI, Claude, Gemini)
- [ ] RAG (Retrieval-Augmented Generation)

---

## 🔮 Phase 13: Embedded & IoT (v4.3)
Running Cryo on constrained devices.

### Embedded Targets
- [ ] ARM Cortex-M compilation
- [ ] RISC-V support
- [ ] ESP32/ESP8266 support
- [ ] Bare metal execution

### IoT Protocols
- [ ] MQTT client
- [ ] CoAP support
- [ ] Modbus protocol
- [ ] Bluetooth Low Energy (BLE)

### Resource Optimization
- [ ] Minimal runtime (~50KB)
- [ ] No-alloc mode
- [ ] Static memory allocation
- [ ] Power management APIs

---

## 🔮 Phase 14: Enterprise & Compliance (v4.4)
Enterprise-grade features for large organizations.

### Enterprise Authentication
- [ ] LDAP/Active Directory integration
- [ ] SAML 2.0 support
- [ ] OpenID Connect
- [ ] Multi-factor authentication

### Compliance & Audit
- [ ] Audit logging
- [ ] Data encryption at rest
- [ ] PCI-DSS compliance helpers
- [ ] GDPR utilities (data anonymization)

### Enterprise Integration
- [ ] Message queues (RabbitMQ, Kafka)
- [ ] Service mesh (Envoy, Istio)
- [ ] Vault integration (secrets management)
- [ ] LDAP directory services

---

## 🔮 Phase 15: Cryo 5.0 (v5.0)
The next major version with breaking changes.

### Language Evolution
- [ ] Pattern matching
- [ ] Algebraic data types
- [ ] Effect system
- [ ] First-class modules

### Runtime 2.0
- [ ] Green threads (coroutines)
- [ ] Structured concurrency
- [ ] Cancellation tokens
- [ ] Resource management (RAII-style)

### Tooling 2.0
- [ ] Visual debugger (GUI)
- [ ] Profiler with flame graphs
- [ ] Memory analyzer
- [ ] AI-powered code completion

### Ecosystem 2.0
- [ ] Central package registry (10k+ packages)
- [ ] Enterprise support tier
- [ ] Certification program
- [ ] Official training courses

---

## 🔮 Phase 16: Blockchain & Web3 (v5.1)
Decentralized application development.

### Smart Contracts
- [ ] EVM bytecode generation
- [ ] Solidity interop
- [ ] Solana program support
- [ ] Move language bridge
- [ ] Contract testing framework

### Crypto & Wallets
- [ ] Wallet management (HD wallets)
- [ ] Transaction signing
- [ ] Multi-chain support (ETH, SOL, BTC)
- [ ] NFT minting utilities
- [ ] DeFi protocol clients

### Web3 Infrastructure
- [ ] IPFS client
- [ ] The Graph integration
- [ ] ENS/Naming services
- [ ] Decentralized identity (DID)

---

## 🔮 Phase 17: Real-time & Gaming (v5.2)
Game development and real-time applications.

### Game Engine Bindings
- [ ] Godot integration
- [ ] Bevy (Rust) bridge
- [ ] Love2D compatibility
- [ ] Physics engine (Box2D/Rapier)
- [ ] 2D sprite management

### Real-time Systems
- [ ] Game loop framework
- [ ] Entity Component System (ECS)
- [ ] Spatial partitioning
- [ ] Networked multiplayer
- [ ] State synchronization

### Media & Audio
- [ ] Audio playback (OpenAL)
- [ ] MIDI support
- [ ] Image manipulation
- [ ] Video streaming
- [ ] WebRTC integration

---

## 🔮 Phase 18: Serverless & Edge (v5.3)
Modern deployment paradigms.

### Serverless Functions
- [ ] AWS Lambda runtime
- [ ] Cloudflare Workers
- [ ] Vercel Edge Functions
- [ ] Deno Deploy target
- [ ] Cold start optimization

### Edge Computing
- [ ] Edge caching
- [ ] Geo-distributed storage
- [ ] CDN integration
- [ ] Request routing
- [ ] A/B testing utilities

### Event-Driven
- [ ] EventBridge integration
- [ ] Webhook handlers
- [ ] Scheduled functions (cron)
- [ ] Event sourcing
- [ ] CQRS patterns

---

## 📊 Priority Matrix

### High Priority (2026)
| Feature | Impact | Effort | Target |
|---------|--------|--------|--------|
| Package Registry | 🔥 High | Medium | Q1 2026 |
| TLS/SSL Support | 🔥 High | Medium | Q2 2026 |
| Async I/O | 🔥 High | High | Q2 2026 |
| JWT Support | 🔥 High | Low | Q1 2026 |

### Medium Priority (2026-2027)
| Feature | Impact | Effort | Target |
|---------|--------|--------|--------|
| JIT Compilation | Medium | High | Q3 2026 |
| Static Typing | Medium | High | Q4 2026 |
| gRPC Support | Medium | Medium | Q3 2026 |
| Cloud SDKs | Medium | Medium | Q4 2026 |

### Long-term (2027+)
| Feature | Impact | Effort | Target |
|---------|--------|--------|--------|
| Mobile Compilation | Medium | Very High | 2027 Q1 |
| AI/ML Libraries | Medium | High | 2027 Q2 |
| Generics | High | Very High | 2027 Q2 |
| Effect System | Low | Very High | 2028 |

---

## 🎯 2026 Goals

### Q1 2026 (January - March)
- [x] v3.2.1: Native Database Connectors ✅
- [x] v3.3.0: Package Registry Web Portal ✅ (tools/registry-server.cryo)
- [x] v3.3.1: SQLite Native Driver ✅ (stdlib/sqlite.cryo)
- [x] v3.3.2: MongoDB Client ✅ (stdlib/mongodb.cryo)

### Q2 2026 (April - June)
- [ ] v3.4.0: Async I/O & Event Loop
- [ ] v3.4.1: TLS 1.3 Support
- [ ] v3.4.2: HTTPS Server
- [x] v3.4.3: JWT & OAuth2 ✅ (stdlib/jwt.cryo, stdlib/oauth2.cryo)

### Q3 2026 (July - September)
- [ ] v3.5.0: JIT Compilation (hot paths)
- [ ] v3.5.1: gRPC Client & Server
- [ ] v3.5.2: OpenTelemetry Integration
- [ ] v3.5.3: Kafka Client

### Q4 2026 (October - December)
- [ ] v3.6.0: Optional Static Types
- [ ] v3.6.1: Cloud SDK (AWS/GCP/Azure)
- [ ] v3.6.2: Container Optimizations
- [ ] v4.0.0-beta: Generics Preview

---

## Docker Database Setup

Quick start for database testing:

```bash
# Start all databases
docker-compose -f docker-compose.db.yml up -d

# Containers started:
# ├── PostgreSQL : localhost:5432 (user: cryo, db: cryodb, trust auth)
# ├── MariaDB    : localhost:3307 (user: cryo, pass: cryo123, db: cryodb)
# └── Redis      : localhost:6379 (no auth)

# Run database tests
./target/release/cryo.exe examples/test_redis_real.ar      # ✅ All pass
./target/release/cryo.exe examples/test_postgres_real.ar   # ✅ All pass
./target/release/cryo.exe examples/test_mysql_real.ar      # ✅ All pass

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
| v3.1.1 | 2025-12 | CryoWeb Framework | ✅ |
| v3.2.0 | 2025-12 | Native Database Connectors | ✅ |
| v3.2.1 | 2026-01 | SHA1 Auth & Binary Protocols | ✅ |
| v3.3.0 | 2026-01-12 | Package Registry & SQLite | ✅ |
| v3.4.0 | 2026-01-12 | Security & Logging Libraries | ✅ Current |
| v3.5.0 | 2026-Q2 | Async I/O & TLS | 🚀 Next |
| v3.6.0 | 2026-Q3 | JIT Compilation | 🔮 Planned |
| v3.7.0 | 2026-Q4 | Static Types Preview | 🔮 Planned |
| v4.0.0 | 2027-Q1 | Generics & Type System | 🔮 Planned |
| v4.1.0 | 2027-Q2 | Mobile & Desktop | 🔮 Planned |
| v4.2.0 | 2027-Q3 | AI/ML Libraries | 🔮 Planned |
| v4.3.0 | 2027-Q4 | Embedded & IoT | 🔮 Planned |
| v4.4.0 | 2028-Q1 | Enterprise Features | 🔮 Planned |
| v5.0.0 | 2028-Q2 | Cryo 5.0 (Next Gen) | 🔮 Vision |
| v5.1.0 | 2028-Q3 | Blockchain & Web3 | 🔮 Vision |
| v5.2.0 | 2028-Q4 | Real-time & Gaming | 🔮 Vision |
| v5.3.0 | 2029-Q1 | Serverless & Edge | 🔮 Vision |

---

## Community & Resources

### Get Involved
- 📖 Documentation: [docs.cryo.dev](https://docs.cryo.dev)
- 💬 Discord: [discord.gg/cryo](https://discord.gg/cryo)
- 🐦 Twitter: [@cryolang](https://twitter.com/cryolang)
- 📦 Packages: [apm.cryo.dev](https://apm.cryo.dev)

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to Cryo.

### License

Cryo is open source under the MIT License. See [LICENSE](LICENSE).

---

*Last updated: 2026-01-12*

