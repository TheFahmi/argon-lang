# Argon Programming Language (v2.1)
![Argon Logo](logo.png)

Argon is a high-performance, self-hosted systems programming language that compiles directly to LLVM IR and Native Machine Code.
Version 2.1 introduces native Networking capabilities (HTTP Server) and a production-ready MVC Project Generator.

## Features
- **Self-Hosted**: Compiler written in Argon (`self-host/compiler.argon`).
- **Native Backend**: Uses LLVM for optimized native binary generation.
- **Networking**: Built-in TCP Socket support (`listen`, `accept`, `read`, `write`).
- **Toolchain**: Integrated Project Scaffolding, Bundler, and Builder.
- **Performance**: High-speed integer arithmetic (tagged pointers optimized).

## Quick Start
Argon Toolchain uses Docker to ensure a consistent build environment.

### 1. Create a New Project
Generate a full MVC Backend skeleton:
```bash
# Git Bash / Linux / Mac
./ar new my_api

# Windows CMD
ar new my_api
```

### 2. Run
Compile and run the project immediately (starts HTTP server on port 3000):
```bash
./ar run my_api
```

### 3. Build only
Produces a native executable inside `dist/`:
```bash
./ar build my_api
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

## Networking API (v2.1)
The runtime now supports primitive TCP networking:
- `argon_listen(port)`: Binds to 0.0.0.0:port. Returns server socket ID.
- `argon_accept(server)`: Blocks and waits for connection. Returns client socket ID.
- `argon_socket_read(client)`: Reads data from client. Returns String.
- `argon_socket_write(client, data)`: Writes string data to client.
- `argon_socket_close(client)`: Closes connection.

## Requirements
- **Docker**: The toolchain runs inside the `argon-toolchain` image.
