#!/bin/bash
set -e

echo "============================================="
echo "   CRYO BOOTSTRAP (Rust Interpreter)"
echo "   Version 2.3"
echo "============================================="
echo ""

# Compile runtime
echo "[Runtime] Compiling Rust Runtime (with threading support)..."
rustc --crate-type staticlib -O -o libruntime_rust.a self-host/runtime.rs

# Use Rust interpreter to compile the Cryo compiler
echo ""
echo "[Stage 0] Compiling compiler.ar with Rust Interpreter..."

# Clean up old files
rm -f self-host/compiler.ll self-host/compiler.ar.ll compiler_stage0.ll

# Use the interpreter to compile the compiler source code
echo "Running: ./cryo --emit-llvm self-host/compiler.cryo"
./cryo --emit-llvm self-host/compiler.ar
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "Error: Stage 0 compilation failed."
    exit 1
fi

# Find output - interpreter writes to file, not stdout
if [ -f "self-host/compiler.ar.ll" ]; then
    mv self-host/compiler.ar.ll compiler_stage0.ll
    echo "Found output at self-host/compiler.ar.ll"
elif [ -f "self-host/compiler.ll" ]; then
    mv self-host/compiler.ll compiler_stage0.ll
    echo "Found output at self-host/compiler.ll"
elif [ -f "compiler.ar.ll" ]; then
    mv compiler.ar.ll compiler_stage0.ll
    echo "Found output at compiler.ar.ll"
fi

echo "Generated IR:"
if [ -f "compiler_stage0.ll" ]; then
    wc -l compiler_stage0.ll
    head -20 compiler_stage0.ll
else
    echo "ERROR: No IR file generated!"
    ls -la self-host/
    ls -la *.ll 2>/dev/null || echo "No .ll files in current dir"
    exit 1
fi

# Link to create cryoc
echo ""
echo "[Link] Creating cryoc binary..."
clang++ -O0 -Wno-override-module compiler_stage0.ll libruntime_rust.a -o cryoc -lpthread -ldl
echo ">> cryoc created!"

# Test with simple file
echo ""
echo "[Test] Testing cryoc..."
echo 'fn main() { print(42); }' > test_simple.ar
./cryoc test_simple.ar

if [ -f "test_simple.ar.ll" ]; then
    echo ">> Compilation successful!"
    clang++ -O0 -Wno-override-module test_simple.ar.ll libruntime_rust.a -o test_simple -lpthread -ldl
    echo "Running test_simple:"
    ./test_simple
else
    echo ">> ERROR: No output generated"
    exit 1
fi

echo ""
echo "============================================="
echo "   BOOTSTRAP COMPLETE (v2.3)"
echo "============================================="
