#!/bin/bash
# ============================================
# ArgonWeb Project Generator
# Usage: ./argonweb-cli.sh new <project-name>
# ============================================

VERSION="1.0.0"
ARGON_BIN="${ARGON_BIN:-argon}"  # Use 'argon' or set ARGON_BIN env var

create_project() {
    PROJECT_NAME=$1
    
    if [ -z "$PROJECT_NAME" ]; then
        echo "Error: Project name is required"
        echo "Usage: argonweb new <project-name>"
        exit 1
    fi
    
    echo ""
    echo "🚀 Creating new ArgonWeb project: $PROJECT_NAME"
    echo ""
    
    # Create directory structure
    mkdir -p "$PROJECT_NAME/src/modules/users"
    mkdir -p "$PROJECT_NAME/src/modules/todos"
    mkdir -p "$PROJECT_NAME/src/modules/auth"
    mkdir -p "$PROJECT_NAME/src/common/middleware"
    mkdir -p "$PROJECT_NAME/src/common/guards"
    mkdir -p "$PROJECT_NAME/src/common/utils"
    mkdir -p "$PROJECT_NAME/src/config"
    
    echo "📁 Created project structure"
    
    # Create config
    cat > "$PROJECT_NAME/src/config/app.config.ar" << 'EOF'
// Application Configuration
let APP_NAME = "ArgonWeb API";
let APP_VERSION = "1.0.0";
let APP_PORT = env("PORT", 3000);
let JWT_SECRET = env("JWT_SECRET", "your-secret-key-change-in-production");
EOF

    # Create utils
    cat > "$PROJECT_NAME/src/common/utils/response.util.ar" << 'EOF'
// Response Utilities
fn response_ok(data) {
    return json_object([
        ["success", json_bool(true)],
        ["data", data]
    ]);
}

fn response_created(data) {
    return json_object([
        ["success", json_bool(true)],
        ["message", json_string("Created successfully")],
        ["data", data]
    ]);
}

fn response_error(message, code) {
    return json_object([
        ["success", json_bool(false)],
        ["error", json_object([
            ["message", json_string(message)],
            ["code", json_number(code)]
        ])]
    ]);
}

fn response_not_found(resource) {
    return response_error(resource + " not found", 404);
}

fn response_unauthorized() {
    return response_error("Unauthorized", 401);
}
EOF

    # Create middleware
    cat > "$PROJECT_NAME/src/common/middleware/logger.middleware.ar" << 'EOF'
// Logger Middleware
fn log_request(method, path) {
    print("[" + date_now() + "] " + method + " " + path);
}

fn log_info(message) {
    print("[INFO] " + message);
}

fn log_error(message) {
    print("[ERROR] " + message);
}
EOF

    # Create auth guard
    cat > "$PROJECT_NAME/src/common/guards/auth.guard.ar" << 'EOF'
// Auth Guard
fn validate_token(token) {
    if (token == null) { return false; }
    let payload = jwt_verify(token, JWT_SECRET);
    return payload != null;
}

fn get_user_from_token(token) {
    return jwt_verify(token, JWT_SECRET);
}
EOF

    # Create user entity
    cat > "$PROJECT_NAME/src/modules/users/user.entity.ar" << 'EOF'
// User Entity
fn user_to_json(id, email, name, role) {
    return json_object([
        ["id", json_number(id)],
        ["email", json_string(email)],
        ["name", json_string(name)],
        ["role", json_string(role)]
    ]);
}
EOF

    # Create users service
    cat > "$PROJECT_NAME/src/modules/users/users.service.ar" << 'EOF'
// Users Service
let users_db = [];
let next_user_id = 1;

fn users_seed() {
    users_db = push(users_db, [1, "admin@example.com", "Admin", bcrypt_hash("admin123"), "admin"]);
    users_db = push(users_db, [2, "user@example.com", "User", bcrypt_hash("user123"), "user"]);
    next_user_id = 3;
}

fn users_find_all() {
    let result = [];
    let i = 0;
    while (i < len(users_db)) {
        let u = users_db[i];
        result = push(result, user_to_json(u[0], u[1], u[2], u[4]));
        i = i + 1;
    }
    return json_array(result);
}

fn users_find_by_id(id) {
    let i = 0;
    while (i < len(users_db)) {
        if (users_db[i][0] == id) {
            let u = users_db[i];
            return user_to_json(u[0], u[1], u[2], u[4]);
        }
        i = i + 1;
    }
    return null;
}

fn users_find_by_email(email) {
    let i = 0;
    while (i < len(users_db)) {
        if (users_db[i][1] == email) {
            return users_db[i];
        }
        i = i + 1;
    }
    return null;
}

fn users_create(email, name, password, role) {
    let id = next_user_id;
    next_user_id = next_user_id + 1;
    let hashed = bcrypt_hash(password);
    users_db = push(users_db, [id, email, name, hashed, role]);
    return user_to_json(id, email, name, role);
}
EOF

    # Create users controller
    cat > "$PROJECT_NAME/src/modules/users/users.controller.ar" << 'EOF'
// Users Controller
fn handle_get_users(ctx) {
    log_request("GET", "/api/users");
    ctx.json(response_ok(users_find_all()));
}

fn handle_get_user_by_id(ctx, id) {
    log_request("GET", "/api/users/" + id);
    let user = users_find_by_id(id);
    if (user == null) {
        ctx.json(response_not_found("User"));
        return;
    }
    ctx.json(response_ok(user));
}

fn handle_create_user(ctx) {
    log_request("POST", "/api/users");
    let user = users_create("new@example.com", "New User", "password", "user");
    ctx.json(response_created(user));
}
EOF

    # Create auth service
    cat > "$PROJECT_NAME/src/modules/auth/auth.service.ar" << 'EOF'
// Auth Service
fn auth_login(email, password) {
    let user = users_find_by_email(email);
    if (user == null) { return null; }
    
    if (!bcrypt_verify(password, user[3])) { return null; }
    
    let payload = "{\"userId\":" + user[0] + ",\"email\":\"" + user[1] + "\"}";
    let token = jwt_sign(payload, JWT_SECRET);
    
    return json_object([
        ["token", json_string(token)],
        ["user", user_to_json(user[0], user[1], user[2], user[4])],
        ["expiresIn", json_number(3600)]
    ]);
}

fn auth_register(email, name, password) {
    if (users_find_by_email(email) != null) { return null; }
    
    let user = users_create(email, name, password, "user");
    let payload = "{\"userId\":" + next_user_id + ",\"email\":\"" + email + "\"}";
    let token = jwt_sign(payload, JWT_SECRET);
    
    return json_object([
        ["token", json_string(token)],
        ["user", user]
    ]);
}
EOF

    # Create auth controller
    cat > "$PROJECT_NAME/src/modules/auth/auth.controller.ar" << 'EOF'
// Auth Controller
fn handle_login(ctx) {
    log_request("POST", "/api/auth/login");
    let result = auth_login("admin@example.com", "admin123");
    if (result == null) {
        ctx.json(response_error("Invalid credentials", 401));
        return;
    }
    ctx.json(response_ok(result));
}

fn handle_register(ctx) {
    log_request("POST", "/api/auth/register");
    let email = "user" + random() + "@example.com";
    let result = auth_register(email, "New User", "password123");
    if (result == null) {
        ctx.json(response_error("Email already exists", 400));
        return;
    }
    ctx.json(response_created(result));
}
EOF

    # Create app.module.ar
    cat > "$PROJECT_NAME/src/app.module.ar" << 'EOF'
// App Module - Route Registration
macro route(app, method, path, handler) {
    $app.router.add($method, $path, $handler);
}

fn register_routes(app) {
    log_info("Registering routes...");
    
    // Root
    route(app, "GET", "/", handle_root);
    route(app, "GET", "/health", handle_health);
    route(app, "GET", "/api", handle_api_info);
    
    // Auth
    route(app, "GET", "/api/auth/login", handle_login);
    route(app, "GET", "/api/auth/register", handle_register);
    
    // Users
    route(app, "GET", "/api/users", handle_get_users);
    
    log_info("Routes registered!");
}

fn handle_root(ctx) {
    ctx.html("<h1>Welcome to ArgonWeb</h1><p>Visit <a href='/api'>/api</a></p>");
}

fn handle_health(ctx) {
    ctx.json(json_object([
        ["status", json_string("healthy")],
        ["timestamp", json_number(now())]
    ]));
}

fn handle_api_info(ctx) {
    ctx.json(json_object([
        ["name", json_string(APP_NAME)],
        ["version", json_string(APP_VERSION)]
    ]));
}
EOF

    # Create main.ar
    cat > "$PROJECT_NAME/src/main.ar" << 'EOF'
// ============================================
// ArgonWeb Application
// Generated by ArgonWeb CLI
// ============================================

import "argon_web"
import "json"

// Config
import "config/app.config"

// Utils
import "common/utils/response.util"
import "common/middleware/logger.middleware"
import "common/guards/auth.guard"

// Modules
import "modules/users/user.entity"
import "modules/users/users.service"
import "modules/users/users.controller"
import "modules/auth/auth.service"
import "modules/auth/auth.controller"

// App Module
import "app.module"

fn main() {
    print("================================================");
    print("   " + APP_NAME);
    print("   Version " + APP_VERSION);
    print("================================================");
    print("");
    
    // Seed database
    users_seed();
    log_info("Database seeded");
    
    // Create server
    let app = Server::new(APP_PORT);
    
    // Register routes
    register_routes(app);
    
    print("");
    print("Server running on http://localhost:" + APP_PORT);
    print("");
    
    app.start();
}
EOF

    # Create README
    cat > "$PROJECT_NAME/README.md" << EOF
# $PROJECT_NAME

ArgonWeb REST API project.

## Project Structure

\`\`\`
$PROJECT_NAME/
├── src/
│   ├── main.ar              # Entry point
│   ├── app.module.ar        # Route registration
│   ├── config/
│   │   └── app.config.ar    # Configuration
│   ├── common/
│   │   ├── middleware/
│   │   ├── guards/
│   │   └── utils/
│   └── modules/
│       ├── users/
│       │   ├── user.entity.ar
│       │   ├── users.service.ar
│       │   └── users.controller.ar
│       └── auth/
│           ├── auth.service.ar
│           └── auth.controller.ar
└── README.md
\`\`\`

## Run

\`\`\`bash
argon src/main.ar
\`\`\`

Or if argon is not in PATH:
\`\`\`bash
./argon src/main.ar
\`\`\`

## API Endpoints

- GET /           - Home
- GET /health     - Health check
- GET /api        - API info
- GET /api/auth/login    - Login
- GET /api/auth/register - Register
- GET /api/users         - List users
EOF

    echo "📄 Created source files"
    echo ""
    echo "✅ Project '$PROJECT_NAME' created successfully!"
    echo ""
    echo "Next steps:"
    echo "  cd $PROJECT_NAME"
    echo "  argon src/main.ar"
    echo ""
    echo "Or if argon is not in PATH:"
    echo "  ../argon.exe src/main.ar"
    echo ""
}

# Main
case "$1" in
    "new")
        create_project "$2"
        ;;
    "version"|"-v"|"--version")
        echo "ArgonWeb CLI v$VERSION"
        ;;
    "help"|"-h"|"--help"|"")
        echo "ArgonWeb CLI v$VERSION"
        echo ""
        echo "Usage:"
        echo "  argonweb new <project-name>   Create new project"
        echo "  argonweb version              Show version"
        echo "  argonweb help                 Show help"
        echo ""
        ;;
    *)
        echo "Unknown command: $1"
        echo "Run 'argonweb help' for usage"
        exit 1
        ;;
esac
