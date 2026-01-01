#!/bin/bash
set -e

echo "============================================="
echo "   CRYO DIRECT BOOTSTRAP (Using Rust Interpreter)"
echo "============================================="
echo ""

# --- Use Rust interpreter to compile compiler.ar ---
echo ""
echo "[Stage 0] Compiling with Rust interpreter (./cryo)..."

# Use Rust interpreter as Stage 0 (most reliable)
./cryo self-host/compiler.ar self-host/compiler.ar

if [ -f "self-host/compiler.ar.ll" ]; then
    cp self-host/compiler.ar.ll compiler_stage1.ll
    echo "Stage 1 output size: $(wc -c < compiler_stage1.ll) bytes"
else
    echo "Error: Stage 0 failed to produce output LLVM IR."
    exit 1
fi

echo "[Stage 1] Linking Stage 1 Compiler (using existing runtime)..."
# Use the SAME runtime as original to ensure compatibility
clang++ -O0 -Wno-override-module compiler_stage1.ll /usr/lib/libruntime_cryo.a -o stage1_compiler -lpthread -ldl
echo ">> Stage 1 Compiler Created: ./stage1_compiler"

# --- Test Stage 1 ---
echo ""
echo "[Test] Testing Stage 1 Compiler with simple file..."
echo 'fn main() { print("Hello from test!"); }' > test_simple.cryo
./stage1_compiler test_simple.cryo 2>&1 | head -20

if [ -f "test_simple.cryo.ll" ]; then
    echo ">> Stage 1 Test PASSED - output generated"
    head -30 test_simple.cryo.ll
else
    echo ">> Stage 1 Test FAILED - no output"
fi

echo ""
echo "============================================="
echo "   BOOTSTRAP COMPLETE"
echo "============================================="
