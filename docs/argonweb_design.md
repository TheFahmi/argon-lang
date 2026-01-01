# ArgonWeb Framework

**Version:** v3.2.1  
**Status:** ✅ Implemented

## Overview

ArgonWeb is a **NestJS-inspired** web framework for Argon. It provides an elegant, modular architecture for building REST APIs and web applications.

## Features

- **Express-like API**: Simple and intuitive routing
- **NestJS Patterns**: Controllers, Services, Middleware
- **Route Parameters**: `/users/:id` style param extraction
- **Query Strings**: `?status=completed` parsing
- **Middleware Pipeline**: Before-request processing
- **Response Helpers**: JSON, HTML, redirect support
- **Built-in Middleware**: Logger, CORS, JSON body parser

## Quick Start

```argon
import "argonweb"

fn main() {
    let app = Server::new(3000);
    
    app.get("/", fn(ctx) {
        ctx.html("<h1>Hello ArgonWeb!</h1>");
    });
    
    app.get("/api/users", fn(ctx) {
        ctx.json(responseOk([
            { id: 1, name: "John" },
            { id: 2, name: "Jane" }
        ]));
    });
    
    app.start();
}
```

## API Reference

### Server

```argon
// Create server
let app = Server::new(3000);

// With custom name
let app = Server::withName(3000, "My API");

// Route shortcuts
app.get("/path", handler);
app.post("/path", handler);
app.put("/path", handler);
app.delete("/path", handler);

// Global middleware
app.use(loggerMiddleware);

// Start server
app.start();
```

### Context

The request context passed to handlers:

```argon
fn myHandler(ctx: Context) {
    // Request info
    let method = ctx.req.method;       // "GET", "POST", etc.
    let path = ctx.req.path;           // "/api/users"
    let body = ctx.req.body;           // Request body string
    
    // Route params (/users/:id)
    let id = ctx.param("id");
    
    // Query params (?page=1)
    let page = ctx.queryParam("page");
    
    // Headers
    let auth = ctx.header("Authorization");
    
    // Response methods
    ctx.json(data);                    // JSON response
    ctx.jsonStatus(201, data);         // JSON with status
    ctx.html("<h1>HTML</h1>");         // HTML response
    ctx.text("Plain text");            // Text response
    ctx.redirect("/new-path");         // 302 redirect
    ctx.sendStatus(204);               // Status only
    
    // Error responses
    ctx.badRequest("Invalid input");
    ctx.unauthorized("Not logged in");
    ctx.forbidden("Access denied");
    ctx.notFound("Resource not found");
    ctx.internalError("Server error");
    
    // Custom headers
    ctx.setHeader("X-Custom", "value");
    
    // Local data (shared between middleware)
    ctx.locals["userId"] = 123;
}
```

### Router

For modular route organization:

```argon
// Create router with prefix
let usersRouter = Router::withPrefix("/api/users");

usersRouter.get("/", handleGetUsers);
usersRouter.get("/:id", handleGetUser);
usersRouter.post("/", handleCreateUser);
usersRouter.delete("/:id", handleDeleteUser);

// Use router middleware
usersRouter.use(authMiddleware);
```

### Route Parameters

```argon
// Define route with params
app.get("/users/:id/posts/:postId", fn(ctx) {
    let userId = ctx.param("id");
    let postId = ctx.param("postId");
    
    ctx.json({
        userId: userId,
        postId: postId
    });
});
```

### Query Strings

```argon
// URL: /api/todos?status=completed&page=1
app.get("/api/todos", fn(ctx) {
    let status = ctx.queryParam("status");  // "completed"
    let page = ctx.queryParam("page");      // "1"
    
    ctx.json(filterTodos(status, page));
});
```

### Middleware

```argon
// Custom middleware
fn authMiddleware(ctx: Context) -> bool {
    let token = ctx.header("Authorization");
    
    if (token == null) {
        ctx.unauthorized("Token required");
        return false;  // Stop request chain
    }
    
    // Validate token...
    ctx.locals["userId"] = parseToken(token);
    return true;  // Continue to next middleware/handler
}

// Use middleware
app.use(authMiddleware);
```

### Built-in Middleware

```argon
// Logger - logs all requests
app.use(loggerMiddleware);
// Output: [2024-01-01] GET /api/users

// CORS - handles cross-origin requests
app.use(corsMiddleware);

// JSON Body Parser
app.use(jsonBodyParserMiddleware);
// Access: ctx.locals["body"]
```

### Response Helpers

```argon
// Standard responses (NestJS-style)
ctx.json(responseOk(data));
// { success: true, data: {...} }

ctx.json(responseCreated(data));
// { success: true, message: "Created successfully", data: {...} }

ctx.json(responseError("Message", 400));
// { success: false, error: { message: "Message", code: 400 } }

ctx.json(responseNotFound("User"));
// { success: false, error: { message: "User not found", code: 404 } }

ctx.json(responseBadRequest("Invalid email"));
ctx.json(responseUnauthorized());
ctx.json(responseForbidden());
```

## Complete Example

```argon
import "argonweb"
import "json"

// Database
let users = [
    { id: 1, name: "John", email: "john@example.com" },
    { id: 2, name: "Jane", email: "jane@example.com" }
];

// Service
fn findUserById(id: int) {
    let i = 0;
    while (i < len(users)) {
        if (users[i].id == id) {
            return users[i];
        }
        i = i + 1;
    }
    return null;
}

// Controller
fn handleGetUser(ctx: Context) {
    let id = parseInt(ctx.param("id"));
    let user = findUserById(id);
    
    if (user == null) {
        ctx.json(responseNotFound("User"));
        return;
    }
    
    ctx.json(responseOk(user));
}

// Main
fn main() {
    let app = Server::new(3000);
    
    // Middleware
    app.use(loggerMiddleware);
    
    // Routes
    app.get("/", fn(ctx) {
        ctx.html("<h1>Welcome!</h1>");
    });
    
    app.get("/api/users", fn(ctx) {
        ctx.json(responseOk(users));
    });
    
    app.get("/api/users/:id", handleGetUser);
    
    // Start
    app.start();
}
```

## Project Structure (NestJS-style)

```
my-api/
├── src/
│   ├── main.ar                 # Entry point
│   ├── app.module.ar           # Route registration
│   ├── config/
│   │   └── app.config.ar       # Configuration
│   ├── common/
│   │   ├── middleware/
│   │   │   └── logger.middleware.ar
│   │   ├── guards/
│   │   │   └── auth.guard.ar
│   │   └── utils/
│   │       └── response.util.ar
│   └── modules/
│       ├── users/
│       │   ├── user.entity.ar
│       │   ├── users.service.ar
│       │   └── users.controller.ar
│       └── todos/
│           ├── todo.entity.ar
│           ├── todos.service.ar
│           └── todos.controller.ar
└── README.md
```

## CLI Tool

Generate new projects with the ArgonWeb CLI:

```bash
./argonweb-cli.sh new my-project
cd my-project
argon src/main.ar
```

## Performance

ArgonWeb runs on Argon's native runtime, leveraging:
- Direct TCP socket handling
- Zero external dependencies
- Optimized request parsing

## See Also

- [HTTP Module](./stdlib/http.ar) - Low-level HTTP utilities
- [JSON Module](./stdlib/json.ar) - JSON parsing/stringifying
- [Example: api_server.ar](./examples/api_server.ar) - Full API demo
- [Example: argonweb_demo.ar](./examples/argonweb_demo.ar) - NestJS-style demo
