#!/bin/bash
# Cryo Build Script v2.25.0

set -e

if [ -f "./cryo.exe" ]; then
    CRYO="./cryo.exe"
elif command -v cryo &> /dev/null; then
    CRYO="cryo"
else
    CRYO="./target/release/cryo"
fi
COMPILER="./self-host/compiler.cryo"
BUILD_DIR="./build"

mkdir -p "$BUILD_DIR/llvm" "$BUILD_DIR/wasm" "$BUILD_DIR/bin"

usage() {
    echo "Cryo Build Tool v2.25.0"
    echo ""
    echo "Usage: ./build.sh [command] [file]"
    echo ""
    echo "Commands:"
    echo "  run <file.ar>         Run with interpreter"
    echo "  compile <file.ar>     Compile to LLVM IR -> build/llvm/"
    echo "  native <file.ar>      Compile to native binary -> build/bin/"
    echo "  wasm <file.ar>        Compile to WASM -> build/wasm/"
    echo "  test                  Run stdlib tests"
    echo "  bench [n]             Run benchmark"
    echo "  clean                 Clean build directory"
    echo ""
    echo "Examples:"
    echo "  ./build.sh run examples/hello.cryo"
    echo "  ./build.sh compile examples/fib.cryo"
    echo "  ./build.sh native examples/fib.cryo"
}

get_basename() {
    basename "$1" .ar
}

# Compile to LLVM IR
compile_llvm() {
    local input="$1"
    local base=$(get_basename "$input")
    local output="$BUILD_DIR/llvm/${base}.ll"
    
    echo "Compiling: $input -> $output"
    $CRYO $COMPILER "$input" -o "$output"
}

# Compile to Native Binary
compile_native() {
    local input="$1"
    local base=$(get_basename "$input")
    local ll_file="$BUILD_DIR/llvm/${base}.ll"
    local bin_file="$BUILD_DIR/bin/${base}.exe"
    
    # First compile to LLVM IR
    compile_llvm "$input"
    
    # Then link with clang
    if command -v clang &> /dev/null; then
        echo "Linking: $ll_file -> $bin_file"
        clang -O2 "$ll_file" -o "$bin_file" 2>/dev/null || echo "Note: Requires runtime library for full execution"
        echo "Done: $bin_file"
    else
        echo "Warning: clang not found. Install LLVM to compile native binaries."
        echo "LLVM IR saved to: $ll_file"
    fi
}

# Compile to WASM
compile_wasm() {
    local input="$1"
    local base=$(get_basename "$input")
    local output="$BUILD_DIR/wasm/${base}.wat"
    
    echo "Compiling: $input -> $output"
    $CRYO $COMPILER "$input" --target wasm32-wasi -o "$output"
    
    # Convert to binary if wat2wasm available
    if command -v wat2wasm &> /dev/null; then
        local wasm_file="$BUILD_DIR/wasm/${base}.wasm"
        wat2wasm "$output" -o "$wasm_file" 2>/dev/null && echo "Binary: $wasm_file"
    fi
}

case "$1" in
    run)
        [ -z "$2" ] && { echo "Error: No input file"; exit 1; }
        $CRYO "$2"
        ;;
    compile)
        [ -z "$2" ] && { echo "Error: No input file"; exit 1; }
        compile_llvm "$2"
        ;;
    native)
        [ -z "$2" ] && { echo "Error: No input file"; exit 1; }
        compile_native "$2"
        ;;
    wasm)
        [ -z "$2" ] && { echo "Error: No input file"; exit 1; }
        compile_wasm "$2"
        ;;
    test)
        echo "=== Cryo Standard Library Tests ==="
        $CRYO test_stdlib.ar
        ;;
    bench)
        N="${2:-35}"
        echo "=== Fibonacci Benchmark (n=$N) ==="
        echo ""
        echo "Native:"
        $CRYO --native-bench $N
        echo ""
        echo "Bytecode VM:"
        $CRYO --vm-bench $N
        ;;
    docker)
        docker build -t cryo-bench . && docker run --rm cryo-bench
        ;;
    clean)
        rm -rf "$BUILD_DIR"
        mkdir -p "$BUILD_DIR/llvm" "$BUILD_DIR/wasm" "$BUILD_DIR/bin"
        echo "Cleaned"
        ;;
    -h|--help|"")
        usage
        ;;
    *)
        $CRYO "$1"
        ;;
esac
