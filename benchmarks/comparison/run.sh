#!/bin/bash
echo "=================================="
echo "      BENCHMARK: Fib(35)"
echo "=================================="

# Compile Rust
echo "[Compiling Rust...]"
rustc -C opt-level=3 fib.rs -o fib_rs

# Compile C++
echo "[Compiling C++...]"
g++ -O3 fib.cpp -o fib_cpp

# Run Rust
echo ""
./fib_rs

# Run C++
echo ""
./fib_cpp

# Run Argon (Interpreter)
echo ""
# Assumes 'argon' is copied to /usr/bin/argon during build
argon fib.ar
