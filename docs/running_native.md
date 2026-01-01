# Menjalankan Cryo Native

Dokumentasi ini menjelaskan cara menjalankan Cryo dalam mode native untuk mendapatkan performa maksimal.

> **Sejak v3.1.0**: Native mode adalah **default**. Gunakan `--interpret` untuk fallback ke interpreter.

---

## 📋 Daftar Isi

1. [Prerequisite](#prerequisite)
2. [Build Cryo Native Compiler](#build-cryo-native-compiler)
3. [Mode Menjalankan Cryo](#mode-menjalankan-cryo)
4. [Kompilasi ke Native Binary](#kompilasi-ke-native-binary)
5. [Benchmark Native](#benchmark-native)
6. [Menggunakan Docker](#menggunakan-docker)
7. [Troubleshooting](#troubleshooting)

---

## Prerequisite

Sebelum menjalankan Cryo native, pastikan Anda memiliki:

### Tools yang Diperlukan

| Tool | Deskripsi | Instalasi |
|------|-----------|-----------|
| **Rust** | Untuk build Cryo Native Compiler | [rustup.rs](https://rustup.rs) |
| **Clang/LLVM** | Untuk kompilasi LLVM IR ke native binary | `apt install clang llvm` (Linux) atau [LLVM Downloads](https://releases.llvm.org/) |
| **Git Bash** (Windows) | Untuk menjalankan script `.sh` | Termasuk dalam Git for Windows |

---

## Build Cryo Native Compiler

### Quick Build

```bash
# Clone repository (jika belum)
git clone <repo-url>
cd cryo

# Build dengan optimasi release
cargo build --release

# Binary akan tersedia di:
# - Windows: target/release/cryo.exe
# - Linux/Mac: target/release/cryo
```

### Struktur Source Code

```
src/
├── main.rs              # Entry point & CLI
├── lexer.rs             # Tokenizer
├── parser.rs            # AST Parser
├── interpreter.rs       # Tree-walking interpreter
├── native_compiler.rs   # LLVM IR code generator
├── bytecode_vm.rs       # Bytecode Virtual Machine
├── fast_vm.rs           # Optimized native benchmarks
├── optimizer.rs         # AST optimizer
├── expander.rs          # Macro expander
├── ffi.rs               # Foreign Function Interface
└── gc.rs                # Garbage Collector
```

### Build Options

```bash
# Debug build (cepat compile, lambat runtime)
cargo build

# Release build (lambat compile, cepat runtime) - RECOMMENDED
cargo build --release

# Build dengan fitur spesifik
cargo build --release --features "llvm"

# Build untuk target spesifik
cargo build --release --target x86_64-pc-windows-msvc
```

### Verifikasi Build

```bash
# Cek versi
./target/release/cryo.exe --version
# Output: Cryo Native v3.1.0

# Cek help
./target/release/cryo.exe --help

# Test benchmark (target: ~40ms)
./target/release/cryo.exe --native-bench 35

# Test file
./target/release/cryo.exe examples/hello.cryo
```

### Install ke System Path (Opsional)

```bash
# Windows (PowerShell sebagai Admin)
copy target\release\cryo.exe C:\Windows\System32\cryo.exe

# Linux/Mac
sudo cp target/release/cryo /usr/local/bin/cryo

# Atau copy ke root project
cp target/release/cryo.exe cryo.exe  # Windows
cp target/release/cryo ./cryo        # Linux/Mac
```

---

## Mode Menjalankan Cryo

Cryo memiliki beberapa mode eksekusi:

### 1. **Native Mode** (Default - v3.1.0+)

Mode default dengan performa optimal. Menjalankan file Cryo dengan optimized interpreter:

```bash
# Windows
./cryo.exe examples/hello.cryo

# Linux/Mac
./cryo examples/hello.cryo

# Eksplisit native mode
./cryo.exe --native examples/hello.cryo
```

### 2. **Interpreter Mode** (Fallback)

Gunakan jika memerlukan kompatibilitas penuh dengan tree-walking interpreter:

```bash
./cryo.exe --interpret examples/hello.cryo
```

### 3. **Bytecode VM Mode** (Benchmark)

Menjalankan dengan bytecode VM untuk performa benchmark:

```bash
./cryo.exe --vm-bench 35
```

### 4. **Native Rust Benchmark**

Baseline performa dengan implementasi native Rust (target: 40ms untuk Fib(35)):

```bash
./cryo.exe --native-bench 35
```

### 5. **Kompilasi ke LLVM IR**

Menghasilkan file LLVM IR (`.ll`) untuk kompilasi native:

```bash
./cryo.exe --emit-llvm output.ll source.cryo
```

---

## Kompilasi ke Native Binary

Untuk mendapatkan performa maksimal (near C++ performance), kompilasi kode Cryo ke native binary melalui LLVM:

### Menggunakan `build.sh`

Script `build.sh` menyediakan workflow lengkap:

```bash
# Lihat bantuan
./build.sh --help

# Kompilasi ke LLVM IR saja
./build.sh compile examples/fib.cryo
# Output: build/llvm/fib.ll

# Kompilasi ke Native Binary (memerlukan clang)
./build.sh native examples/fib.cryo
# Output: build/bin/fib.exe
```

### Manual Compilation

Jika Anda ingin mengontrol proses kompilasi secara manual:

```bash
# Step 1: Generate LLVM IR
./cryo.exe self-host/compiler.cryo examples/fib.cryo -o fib.ll

# Step 2: Compile ke native binary dengan clang
clang -O3 -march=native fib.ll -o fib.exe

# Step 3: Jalankan
./fib.exe
```

### Opsi Optimasi Clang

| Flag | Deskripsi |
|------|-----------|
| `-O0` | Tanpa optimasi (debug) |
| `-O2` | Optimasi standar |
| `-O3` | Optimasi agresif (recommended) |
| `-march=native` | Optimasi untuk CPU saat ini |
| `-flto` | Link-time optimization |

Contoh kompilasi dengan optimasi penuh:

```bash
clang -O3 -march=native -flto fib.ll -o fib_optimized.exe
```

---

## Benchmark Native

### Quick Benchmark

Jalankan benchmark Fibonacci cepat:

```bash
# Bytecode VM benchmark
./cryo.exe --vm-bench 35

# Native Rust baseline
./cryo.exe --native-bench 35

# Atau menggunakan build.sh
./build.sh bench 35
```

### Benchmark Komprehensif

Untuk benchmark lengkap yang membandingkan Cryo dengan C++, Rust, dll:

```bash
# Masuk ke direktori benchmark
cd benchmarks/comparison

# Jalankan semua benchmark
./run.sh
```

Script ini akan menjalankan:
- **Sum Loop**: 1 miliar iterasi
- **Ackermann(3,11)**: Test rekursi dalam
- **Fibonacci(45)**: Test rekursi klasik

### Hasil Benchmark Fibonacci(35)

Benchmark dijalankan pada 1 Januari 2026:

| Mode | Waktu | Hasil |
|------|-------|-------|
| **Native Rust** | **40ms** ✅ | 9227465 |
| Bytecode VM | 3678ms | 9227465 |

**Speedup**: Native ~92x lebih cepat dari bytecode VM

### Hasil Benchmark Komprehensif (Referensi)

| Benchmark | C++ | Cryo Native | Rust | Cryo vs Rust |
|-----------|-----|--------------|------|---------------|
| **Fibonacci(35)** | ~35ms | **40ms** | ~50ms | **20% faster** |
| **Fibonacci(45)** | 4.1s | 5.1s | 6.3s | **19% faster** |
| **Ackermann(3,11)** | 136ms | 232ms | 261ms | **11% faster** |
| **Sum Loop (1B)** | 798ms | 0ms* | 1526ms | **∞ faster** |

*LLVM mengoptimasi loop secara penuh saat compile time

---

## Menggunakan Docker

Docker menyediakan environment yang konsisten untuk build dan benchmark:

### Build Image

```bash
docker build -t cryo-bench .
```

### Jalankan Benchmark

```bash
# Jalankan benchmark default
docker run --rm cryo-bench

# Jalankan perintah custom
docker run --rm cryo-bench ./build.sh native examples/fib.cryo
```

### Dockerfile Overview

```dockerfile
FROM rust:slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    g++ clang llvm time

# Build Cryo
RUN cargo build --release
RUN cp target/release/cryo /usr/bin/cryo

# Default: Run tests then benchmarks
CMD ["bash", "-c", "./build.sh test && ./build.sh bench"]
```

---

## Troubleshooting

### Error: "clang not found"

**Solusi**: Install LLVM/Clang

```bash
# Ubuntu/Debian
sudo apt install clang llvm

# macOS
brew install llvm

# Windows
# Download dari: https://releases.llvm.org/
# Atau gunakan chocolatey: choco install llvm
```

### Error: "undefined reference" saat linking

**Penyebab**: Beberapa fungsi runtime belum diimplementasikan dalam LLVM IR.

**Solusi**: Gunakan interpreter untuk fungsi yang kompleks, atau pastikan semua fungsi yang dipanggil sudah ada dalam file `.ll`.

### Performa tidak optimal

**Solusi**:
1. Pastikan menggunakan flag `-O3` dan `-march=native`
2. Gunakan release build: `cargo build --release`
3. Pastikan tidak ada proses lain yang berjalan saat benchmark

### Windows: Script `.sh` tidak berjalan

**Solusi**: Gunakan Git Bash atau WSL untuk menjalankan script shell.

```bash
# Dengan Git Bash
bash build.sh native examples/fib.cryo

# Dengan WSL
wsl ./build.sh native examples/fib.cryo
```

---

## Ringkasan Perintah

| Perintah | Deskripsi |
|----------|-----------|
| `./cryo.exe file.cryo` | Jalankan dengan interpreter |
| `./cryo.exe --vm-bench N` | Benchmark bytecode VM |
| `./cryo.exe --native-bench N` | Benchmark native Rust |
| `./build.sh run file.cryo` | Jalankan file |
| `./build.sh compile file.cryo` | Compile ke LLVM IR |
| `./build.sh native file.cryo` | Compile ke native binary |
| `./build.sh bench N` | Jalankan benchmark |
| `./build.sh test` | Jalankan test stdlib |

---

## Lihat Juga

- [README.md](../README.md) - Overview proyek Cryo
- [ROADMAP.md](../ROADMAP.md) - Roadmap pengembangan
- [performance_optimization.md](./performance_optimization.md) - Tips optimasi performa
- [wasm_design.md](./wasm_design.md) - Kompilasi ke WebAssembly
