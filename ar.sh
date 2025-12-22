#!/bin/bash
# Wrapper for Argon Compiler via Docker
# Usage: ./ar [build|run] <file.ar> [--unsafe-math]

COMMAND=$1
FILE=$2
FLAG=$3

if [ -z "$COMMAND" ]; then
    echo "Usage: ./ar [new|build|run] <file_or_project>"
    exit 1
fi

# Scaffold New Project
if [ "$COMMAND" == "new" ]; then
    PROJECT_NAME=$FILE
    if [ -z "$PROJECT_NAME" ]; then
        echo "Error: Please specify project name."
        echo "Usage: ./ar new <project_name>"
        exit 1
    fi
    
    TARGET_DIR="$HOST_PWD/$PROJECT_NAME"
    # Check if exists (using relative check first to be safe)
    if [ -d "$PROJECT_NAME" ]; then
        echo "Error: Directory '$PROJECT_NAME' already exists."
        exit 1
    fi
    
    # Create Directories
    mkdir -p "$PROJECT_NAME/src/controllers"
    mkdir -p "$PROJECT_NAME/src/services"
    mkdir -p "$PROJECT_NAME/src/models"
    mkdir -p "$PROJECT_NAME/tests"
    mkdir -p "$PROJECT_NAME/dist"
    
    # Create .gitignore & README (Simplified for brevity)
    echo "dist/" > "$PROJECT_NAME/.gitignore"; echo "*.out" >> "$PROJECT_NAME/.gitignore"; echo "*.ll" >> "$PROJECT_NAME/.gitignore"
    echo "# $PROJECT_NAME" > "$PROJECT_NAME/README.md"; echo "Argon Backend." >> "$PROJECT_NAME/README.md"
    
    # 1. Service
    cat <<EOF > "$PROJECT_NAME/src/services/app.service.ar"
fn AppService_getHello() {
    return "Hello from $PROJECT_NAME Service!";
}
EOF

    # 2. Controller
    cat <<EOF > "$PROJECT_NAME/src/controllers/app.controller.ar"
fn AppController_handle(req) {
    // We could parse 'req' here to check path
    let msg = AppService_getHello();
    return "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n" + msg;
}
EOF

    # 3. Server Logic
    cat <<EOF > "$PROJECT_NAME/src/server.ar"
fn Server_start() {
    let PORT = 3000;
    print("[$PROJECT_NAME] Server listening on port " + PORT + "...");
    
    let server = argon_listen(PORT);
    if (server < 0) { print("Error: Bind failed."); return 0; }
    
    while (1) {
        let client = argon_accept(server);
        if (client != -1) {
             let req = argon_socket_read(client);
             // Dispatch to Controller
             let resp = AppController_handle(req);
             
             argon_socket_write(client, resp);
             argon_socket_close(client);
        }
    }
    return 0;
}
EOF

    # 4. Entry Point
    echo "fn main() { Server_start(); }" > "$PROJECT_NAME/src/main.ar"
    
    echo ">> Done! Structure:"
    ls -R "$PROJECT_NAME"
    exit 0
fi


# Handle Windows Path for Docker (Git Bash)
HOST_PWD=$(pwd -W 2>/dev/null)
if [ -z "$HOST_PWD" ]; then
    HOST_PWD=$(pwd)
fi

echo "Argon Toolchain: Processing $FILE in $HOST_PWD..."

# Project Mode (Directory)
if [ -d "$FILE" ]; then
    echo ">> [Bundler] Project Directory detected: $FILE"
    mkdir -p "$FILE/dist"
    BUNDLE="$FILE/dist/bundle.argon"
    echo ">> [Bundler] Merging sources from $FILE/src..."
    # Concatenate all .argon and .ar files
    find "$FILE/src" \( -name "*.argon" -o -name "*.ar" \) -exec cat {} + > "$BUNDLE"
    # Target the bundle
    FILE="$BUNDLE"
    echo ">> [Bundler] Created $BUNDLE"
fi

# 1. Compile (Build)
# Mounts current dir to /src, compiles with argonc, links with clang
docker run --rm -v "${HOST_PWD}:/src" -w //src argon-toolchain \
    bash -c "argonc $FLAG $FILE && clang++ -O3 -flto -Wno-override-module ${FILE}.ll /usr/lib/libruntime_argon.a -o ${FILE}.out -lpthread -ldl"

EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "Error: Compilation failed."
    exit $EXIT_CODE
fi

echo ">> Build Complete: ${FILE}.out"

# 2. Run (if requested)
if [ "$COMMAND" == "run" ]; then
    echo ">> Running..."
    docker run --rm -v "${HOST_PWD}:/src" -w //src argon-toolchain ./${FILE}.out
fi
