#!/bin/bash
set -e
echo "=========================================="
echo "   COMPREHENSIVE BENCHMARK SUITE"
echo "=========================================="

echo ""
echo "CPU: $(cat /proc/cpuinfo | grep 'model name' | head -1 | cut -d: -f2)"
echo ""

###############################################
# 1. SUM LOOP - Tests simple loop performance
###############################################
echo "[1] Creating Sum Loop benchmark (1 billion iterations)..."

cat > sumloop.cpp << 'EOF'
#include <iostream>
#include <chrono>

int main() {
    std::cout << "C++ Sum Loop: Starting..." << std::endl;
    auto start = std::chrono::high_resolution_clock::now();
    
    long long sum = 0;
    for (long long i = 1; i <= 1000000000LL; i++) {
        sum += i * i;
    }
    
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "C++ Sum Loop: Result = " << sum << std::endl;
    std::cout << "C++ Sum Loop: Time = " << duration.count() << "ms" << std::endl;
    return 0;
}
EOF

cat > sumloop.rs << 'EOF'
use std::time::Instant;

fn main() {
    println!("Rust Sum Loop: Starting...");
    let start = Instant::now();
    
    let mut sum: i64 = 0;
    for i in 1i64..=1000000000 {
        sum = sum.wrapping_add(i.wrapping_mul(i));
    }
    
    let duration = start.elapsed();
    println!("Rust Sum Loop: Result = {}", sum);
    println!("Rust Sum Loop: Time = {}ms", duration.as_millis());
}
EOF

###############################################
# 2. ACKERMANN - Tests deep recursion
###############################################
echo "[2] Creating Ackermann benchmark..."

cat > ack.cpp << 'EOF'
#include <iostream>
#include <chrono>

int ack(int m, int n) {
    if (m == 0) return n + 1;
    if (n == 0) return ack(m - 1, 1);
    return ack(m - 1, ack(m, n - 1));
}

int main() {
    std::cout << "C++ Ackermann(3,11): Starting..." << std::endl;
    auto start = std::chrono::high_resolution_clock::now();
    int result = ack(3, 11);
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "C++ Ackermann: Result = " << result << std::endl;
    std::cout << "C++ Ackermann: Time = " << duration.count() << "ms" << std::endl;
    return 0;
}
EOF

cat > ack.rs << 'EOF'
use std::time::Instant;

fn ack(m: i32, n: i32) -> i32 {
    if m == 0 { return n + 1; }
    if n == 0 { return ack(m - 1, 1); }
    ack(m - 1, ack(m, n - 1))
}

fn main() {
    println!("Rust Ackermann(3,11): Starting...");
    let start = Instant::now();
    let result = ack(3, 11);
    let duration = start.elapsed();
    println!("Rust Ackermann: Result = {}", result);
    println!("Rust Ackermann: Time = {}ms", duration.as_millis());
}
EOF

###############################################
# ARGON LLVM FILES
###############################################
echo "[3] Creating Argon LLVM IR files..."

# SumLoop LLVM
cat > sumloop_argon.ll << 'LLVM_EOF'
target triple = "x86_64-pc-linux-gnu"
@.s0 = private constant [29 x i8] c"Argon Sum Loop: Starting...\0A\00"
@.s1 = private constant [24 x i8] c"Argon Sum Loop: Result=\00"
@.s2 = private constant [22 x i8] c"Argon Sum Loop: Time=\00"
@.s3 = private constant [4 x i8] c"ms\0A\00"
@.s4 = private constant [6 x i8] c" %ld\0A\00"
@.s5 = private constant [4 x i8] c"%ld\00"

declare i32 @printf(i8*, ...) nounwind
declare i64 @clock() nounwind

define i64 @sum_loop(i64 %n) nounwind readnone {
entry:
    br label %loop
loop:
    %i = phi i64 [ 1, %entry ], [ %i_next, %loop ]
    %sum = phi i64 [ 0, %entry ], [ %sum_next, %loop ]
    %sq = mul nsw i64 %i, %i
    %sum_next = add nsw i64 %sum, %sq
    %i_next = add nuw nsw i64 %i, 1
    %cond = icmp sle i64 %i_next, %n
    br i1 %cond, label %loop, label %done
done:
    ret i64 %sum_next
}

define i32 @main() {
entry:
    %p0 = getelementptr [29 x i8], [29 x i8]* @.s0, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p0)
    %t0 = call i64 @clock()
    %result = call i64 @sum_loop(i64 1000000000)
    %t1 = call i64 @clock()
    %diff = sub i64 %t1, %t0
    %ms = sdiv i64 %diff, 1000
    %p1 = getelementptr [24 x i8], [24 x i8]* @.s1, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p1)
    %p4 = getelementptr [6 x i8], [6 x i8]* @.s4, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p4, i64 %result)
    %p2 = getelementptr [22 x i8], [22 x i8]* @.s2, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p2)
    %p5 = getelementptr [4 x i8], [4 x i8]* @.s5, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p5, i64 %ms)
    %p3 = getelementptr [4 x i8], [4 x i8]* @.s3, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p3)
    ret i32 0
}
LLVM_EOF

# Ackermann LLVM
cat > ack_argon.ll << 'LLVM_EOF'
target triple = "x86_64-pc-linux-gnu"
@.s0 = private constant [36 x i8] c"Argon Ackermann(3,11): Starting...\0A\00"
@.s1 = private constant [26 x i8] c"Argon Ackermann: Result =\00"
@.s2 = private constant [24 x i8] c"Argon Ackermann: Time =\00"
@.s3 = private constant [4 x i8] c"ms\0A\00"
@.s4 = private constant [6 x i8] c" %ld\0A\00"
@.s5 = private constant [4 x i8] c"%ld\00"

declare i32 @printf(i8*, ...) nounwind
declare i64 @clock() nounwind

define i32 @ack(i32 %m, i32 %n) nounwind readnone {
entry:
    %cmp_m = icmp eq i32 %m, 0
    br i1 %cmp_m, label %case0, label %check_n
case0:
    %r0 = add nsw i32 %n, 1
    ret i32 %r0
check_n:
    %cmp_n = icmp eq i32 %n, 0
    br i1 %cmp_n, label %case1, label %case2
case1:
    %m1 = sub nsw i32 %m, 1
    %r1 = call i32 @ack(i32 %m1, i32 1)
    ret i32 %r1
case2:
    %n1 = sub nsw i32 %n, 1
    %inner = call i32 @ack(i32 %m, i32 %n1)
    %m2 = sub nsw i32 %m, 1
    %r2 = call i32 @ack(i32 %m2, i32 %inner)
    ret i32 %r2
}

define i32 @main() {
entry:
    %p0 = getelementptr [36 x i8], [36 x i8]* @.s0, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p0)
    %t0 = call i64 @clock()
    %result = call i32 @ack(i32 3, i32 11)
    %t1 = call i64 @clock()
    %diff = sub i64 %t1, %t0
    %ms = sdiv i64 %diff, 1000
    %p1 = getelementptr [26 x i8], [26 x i8]* @.s1, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p1)
    %result64 = sext i32 %result to i64
    %p4 = getelementptr [6 x i8], [6 x i8]* @.s4, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p4, i64 %result64)
    %p2 = getelementptr [24 x i8], [24 x i8]* @.s2, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p2)
    %p5 = getelementptr [4 x i8], [4 x i8]* @.s5, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p5, i64 %ms)
    %p3 = getelementptr [4 x i8], [4 x i8]* @.s3, i64 0, i64 0
    call i32 (i8*, ...) @printf(i8* %p3)
    ret i32 0
}
LLVM_EOF

###############################################
# COMPILE ALL
###############################################
echo ""
echo "[4] Compiling all benchmarks..."

# C++
g++ -O3 -march=native sumloop.cpp -o sumloop_cpp
g++ -O3 -march=native ack.cpp -o ack_cpp

# Rust
rustc -C opt-level=3 -C target-cpu=native sumloop.rs -o sumloop_rs
rustc -C opt-level=3 -C target-cpu=native ack.rs -o ack_rs

# Argon LLVM
echo "    Compiling Argon Sum Loop..."
clang -O3 -march=native sumloop_argon.ll -o sumloop_argon

echo "    Compiling Argon Ackermann..."
clang -O3 -march=native ack_argon.ll -o ack_argon

###############################################
# RUN BENCHMARKS
###############################################
echo ""
echo "=========================================="
echo "  BENCHMARK 1: SUM LOOP (1 Billion iter)"
echo "=========================================="
echo ""
./sumloop_cpp
echo ""
./sumloop_rs
echo ""
./sumloop_argon

echo ""
echo "=========================================="
echo "  BENCHMARK 2: ACKERMANN (3, 11)"
echo "=========================================="
echo ""
./ack_cpp
echo ""
./ack_rs
echo ""
./ack_argon

echo ""
echo "=========================================="
echo "       ALL BENCHMARKS COMPLETE"
echo "=========================================="
