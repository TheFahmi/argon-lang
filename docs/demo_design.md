# Argon Ecosystem Demo: Web Framework (v2.25.0)

## Goal
Showcase Argon's capabilities by building a lightweight, async web framework and a Todo API.

## Architecture

### 1. `ArgonWeb` Framework
A library written in Argon that provides:
- **HttpServer**: Async TCP listener.
- **Router**: Trie-based or Regex-based route matching.
- **Context**: Wraps Request/Response.

### 2. Features Usage
- **Macros**: `router.get("/", handler)` or even `#[route("/")]` (if attributes supported, otherwise macro `route!(GET, "/", handler)`).
- **Traits**: `Handler` trait for controllers.
- **Async**: `async fn handle_connection(conn)` for concurrency.
- **Defer**: Closing sockets/files.
- **Structs**: `Request`, `Response` models.

## API Example (`examples/web_demo.ar`)

```javascript
import "http"
import "json"

macro route(method, path, handler) {
    server.add_route($method, $path, $handler);
}

async fn main() {
    let server = http.Server::new("127.0.0.1", 8080);
    print("Starting server on :8080");

    // Define Routes
    route("GET", "/", home_handler);
    route("GET", "/api/todos", list_todos);
    route("POST", "/api/todos", create_todo);

    await server.listen();
}

fn home_handler(req) {
    return http.Response::html("<h1>Welcome to ArgonWeb</h1>");
}

fn list_todos(req) {
    let todos = [
        { "id": 1, "task": "Build Compiler" },
        { "id": 2, "task": "Make Demo" }
    ];
    return http.Response::json(todos);
}
```

## Implementation Plan
1. **`lib/http.ar`**: HTTP Request parsing and Response formatting.
2. **`lib/server.ar`**: Async TCP Loop.
3. **`examples/web_demo.ar`**: The application.

## Prerequisites
- `std::net` must support `TcpListener` (already there).
- `std::net` must support `async` accept? (Needs verification if `v2.18` async covers net). 
  - *Note*: If `async` net is not fully ready, we might need to polish it or use blocking for v1.
- `std::json` (we can write a simple serializer in Argon).

## Versioning
This ecosystem demo will mark **v2.25.0**.
