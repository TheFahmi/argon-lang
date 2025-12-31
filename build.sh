#!/bin/bash
# Argon Build Script v2.25.0
# Run, test, compile, and benchmark Argon programs

set -e

ARGON="./argon.exe"
COMPILER="./self-host/compiler.ar"
BUILD_DIR="./build"

# Ensure build directories exist
mkdir -p "$BUILD_DIR/llvm" "$BUILD_DIR/wasm" "$BUILD_DIR/bin"

usage() {
    echo "Argon Build Tool v2.25.0"
    echo ""
    echo "Usage: ./build.sh [command] [options] [file]"
    echo ""
    echo "Commands:"
    echo "  run <file.ar>         Run with interpreter (fast)"
    echo "  compile <file.ar>     Compile to LLVM IR -> build/llvm/"
    echo "  wasm <file.ar>        Compile to WASM -> build/wasm/"
    echo "  test                  Run stdlib tests"
    echo "  bench [n]             Run fibonacci benchmark (default n=35)"
    echo "  docker                Run benchmarks in Docker"
    echo "  clean                 Clean build directory"
    echo ""
    echo "Options:"
    echo "  -o <path>             Custom output path"
    echo ""
    echo "Examples:"
    echo "  ./build.sh run examples/hello.ar"
    echo "  ./build.sh compile examples/fib.ar"
    echo "  ./build.sh test"
    echo "  ./build.sh bench 35"
}

get_basename() {
    basename "$1" .ar
}

compile_llvm() {
    local input="$1"
    local output="$2"
    local base=$(get_basename "$input")
    
    if [ -z "$output" ]; then
        output="$BUILD_DIR/llvm/${base}.ll"
    fi
    
    echo "Compiling: $input -> $output"
    $ARGON $COMPILER "$input" -o "$output"
    echo "Done: $output"
}

compile_wasm() {
    local input="$1"
    local output="$2"
    local base=$(get_basename "$input")
    
    if [ -z "$output" ]; then
        output="$BUILD_DIR/wasm/${base}.wat"
    fi
    
    echo "Compiling: $input -> $output (WASM)"
    $ARGON $COMPILER "$input" --target wasm32-wasi -o "$output"
    echo "Done: $output"
}

case "$1" in
    run)
        if [ -z "$2" ]; then
            echo "Error: No input file"
            exit 1
        fi
        $ARGON "$2"
        ;;
    compile)
        if [ -z "$2" ]; then
            echo "Error: No input file"
            exit 1
        fi
        OUTPUT=""
        if [ "$3" = "-o" ] && [ -n "$4" ]; then
            OUTPUT="$4"
        fi
        compile_llvm "$2" "$OUTPUT"
        ;;
    wasm)
        if [ -z "$2" ]; then
            echo "Error: No input file"
            exit 1
        fi
        OUTPUT=""
        if [ "$3" = "-o" ] && [ -n "$4" ]; then
            OUTPUT="$4"
        fi
        compile_wasm "$2" "$OUTPUT"
        ;;
    test)
        echo "=== Running Standard Library Tests ==="
        $ARGON test_stdlib.ar
        ;;
    bench)
        N="${2:-35}"
        echo "=== Fibonacci Benchmark (n=$N) ==="
        echo ""
        echo "Native (Rust baseline):"
        $ARGON --native-bench $N
        echo ""
        echo "Bytecode VM:"
        $ARGON --vm-bench $N
        ;;
    docker)
        echo "=== Building and running Docker benchmark ==="
        docker build -t argon-bench . && docker run --rm argon-bench
        ;;
    clean)
        rm -rf "$BUILD_DIR"
        mkdir -p "$BUILD_DIR/llvm" "$BUILD_DIR/wasm" "$BUILD_DIR/bin"
        echo "Build directory cleaned"
        ;;
    -h|--help|"")
        usage
        ;;
    *)
        # Default: run the file
        $ARGON "$1"
        ;;
esac
