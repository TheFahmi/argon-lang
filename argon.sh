#!/bin/bash
# Wrapper for Cryo Compiler via Docker
# Usage: ./cryo [build|run] <file.ar> [--unsafe-math]

COMMAND=$1
FILE=$2
FLAG=$3

if [ -z "$COMMAND" ]; then
    echo "Usage: ./cryo [new|build|run] <file_or_project>"
    exit 1
fi

# Scaffold New Project
if [ "$COMMAND" == "new" ]; then
    PROJECT_NAME=$FILE
    if [ -z "$PROJECT_NAME" ]; then
        echo "Error: Please specify project name."
        echo "Usage: ./cryo new <project_name>"
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
    echo "# $PROJECT_NAME" > "$PROJECT_NAME/README.md"; echo "Cryo Backend." >> "$PROJECT_NAME/README.md"
    
    # 1. Service
    cat <<EOF > "$PROJECT_NAME/src/services/app.service.cryo"
fn AppService_getHello() {
    return "Hello from $PROJECT_NAME Service!";
}
EOF

    # 2. Controller
    cat <<EOF > "$PROJECT_NAME/src/controllers/app.controller.cryo"
fn AppController_handle(req) {
    // We could parse 'req' here to check path
    let msg = AppService_getHello();
    return "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n" + msg;
}
EOF

    # 3. Server Logic
    cat <<EOF > "$PROJECT_NAME/src/server.cryo"
fn Server_start() {
    let PORT = 3000;
    print("[$PROJECT_NAME] Server listening on port " + PORT + "...");
    
    let server = cryo_listen(PORT);
    if (server < 0) { print("Error: Bind failed."); return 0; }
    
    while (1) {
        let client = cryo_accept(server);
        if (client != -1) {
             let req = cryo_socket_read(client);
             // Dispatch to Controller
             let resp = AppController_handle(req);
             
             cryo_socket_write(client, resp);
             cryo_socket_close(client);
        }
    }
    return 0;
}
EOF

    # 4. Entry Point
    echo "fn main() { Server_start(); }" > "$PROJECT_NAME/src/main.cryo"
    
    echo ">> Done! Structure:"
    ls -R "$PROJECT_NAME"
    exit 0
fi


# Handle Windows Path for Docker (Git Bash)
HOST_PWD=$(pwd -W 2>/dev/null)
if [ -z "$HOST_PWD" ]; then
    HOST_PWD=$(pwd)
fi

echo "Cryo Toolchain: Processing $FILE in $HOST_PWD..."

# Project Mode (Directory)
if [ -d "$FILE" ]; then
    echo ">> [Bundler] Project Directory detected: $FILE"
    mkdir -p "$FILE/dist"
    BUNDLE="$FILE/dist/bundle.cryo"
    echo ">> [Bundler] Merging sources from $FILE/src..."
    # Concatenate all .cryo and .ar files
    find "$FILE/src" \( -name "*.cryo" -o -name "*.cryo" \) -exec cat {} + > "$BUNDLE"
    # Target the bundle
    FILE="$BUNDLE"
    echo ">> [Bundler] Created $BUNDLE"
fi

# 1. Compile (Build)
# Mounts current dir to /src, compiles with cryoc, links with clang
DEBUG_FLAG=""
CLANG_DEBUG=""
if [ "$COMMAND" == "debug" ]; then
    DEBUG_FLAG="-g"
    CLANG_DEBUG="-g"
fi
docker run --rm -v "${HOST_PWD}:/src" -w //src cryo-toolchain \
    bash -c "cryoc $DEBUG_FLAG $FLAG $FILE && clang++ $CLANG_DEBUG -O0 -Wno-override-module ${FILE}.ll /usr/lib/libruntime_cryo.a -o ${FILE}.out -lpthread -ldl"

EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "Error: Compilation failed."
    exit $EXIT_CODE
fi

echo ">> Build Complete: ${FILE}.out"

# 2. Run (if requested)
if [ "$COMMAND" == "run" ]; then
    echo ">> Running..."
    docker run --rm -v "${HOST_PWD}:/src" -w //src cryo-toolchain ./${FILE}.out
fi

# 3. Debug (if requested)
if [ "$COMMAND" == "debug" ]; then
    echo ">> Starting GDB debugger..."
    docker run --rm -it -v "${HOST_PWD}:/src" -w //src cryo-toolchain gdb ./${FILE}.out
fi
