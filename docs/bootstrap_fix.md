# Argon Bootstrap Fix Documentation

## Quick Reference

| Versi | Binary | Status |
|-------|--------|--------|
| v2.16.0 | `argonc_v216` | ✅ Stable |
| v2.18.0 | `argonc_v218` | ✅ Async/await |
| v2.19.0 | `argonc_v219` | 🔄 WebAssembly (pending) |

---

## Problem

The Argon compiler has a bootstrap challenge: to compile the self-hosting compiler, you need a working compiler first. The Rust interpreter has a bug where it deletes output files after `clang` failures.

### Root Causes

1. **Interpreter File Deletion Bug**: The Rust interpreter overwrites the input `.ar` file with LLVM IR, then calls `clang`. If `clang` fails, the interpreter deletes the file.

2. **Duplicate Function Definition** (v2.15.0): `generate_specialized_funcs()` was defined twice, causing LLVM IR error.

---

## Solution: Bootstrap New Compiler

### Method 1: Using Docker (Recommended)

```bash
# Build dengan Dockerfile yang sudah ada
docker build -t argon-toolchain .

# Test compiler
docker run --rm argon-toolchain argonc --version
# Output: Argon Compiler v2.18.0
```

### Method 2: Manual Bootstrap dengan inotifywait

Jika perlu bootstrap binary baru dari source:

```bash
# 1. Jalankan di dalam Docker container
docker run -it --rm -v $(pwd):/workspace argon-toolchain bash

# 2. Copy compiler source ke /app
cp /workspace/self-host/compiler.ar /app/src.ar

# 3. Setup inotifywait untuk capture LLVM IR
inotifywait -m -e modify /app/src.ar --format "%w%f" 2>/dev/null | while read f; do
    cp /app/src.ar /app/compiler.ll 2>/dev/null
done &

# 4. Jalankan interpreter untuk generate LLVM IR
sleep 1
/app/argon --emit-llvm /app/src.ar || true
sleep 1

# 5. Kill inotifywait
kill %1 2>/dev/null

# 6. Compile LLVM IR ke binary
clang++ -O2 -Wno-override-module \
    /app/compiler.ll \
    /usr/lib/libruntime_argon.a \
    -lpthread -ldl \
    -o /workspace/self-host/argonc_new

# 7. Test binary baru
/workspace/self-host/argonc_new --version
```

### Method 3: Fix di Dockerfile

```dockerfile
# Capture LLVM IR dengan inotifywait trick
RUN bash -c ' \
    inotifywait -m -e modify /app/src.ar --format "%w%f" 2>/dev/null | while read f; do \
        cp /app/src.ar /app/compiler.ll 2>/dev/null; \
    done & \
    sleep 1; \
    /app/argon --emit-llvm /app/src.ar || true; \
    sleep 1; \
    kill %1 2>/dev/null \
'

# Compile ke binary
RUN clang++ -O2 -Wno-override-module \
    /app/compiler.ll \
    /usr/lib/libruntime_argon.a \
    -lpthread -ldl \
    -o /usr/bin/argonc
```

---

## Troubleshooting

### Error: "invalid redefinition of function"
**Cause**: Duplicate function di `compiler.ar`
**Fix**: Hapus salah satu definisi function yang duplikat

```bash
# Cari duplicate
grep -n "fn generate_specialized_funcs" self-host/compiler.ar
```

### Error: File hilang setelah compile
**Cause**: Interpreter menghapus file setelah clang gagal
**Fix**: Gunakan inotifywait trick untuk capture file sebelum dihapus

### Error: "undefined reference to argon_*"
**Cause**: Runtime library tidak di-link
**Fix**: Pastikan link dengan `-lpthread -ldl` dan `libruntime_argon.a`

### Banner menunjukkan versi lama
**Cause**: Rust interpreter punya hardcoded version string
**Note**: Ini kosmetik saja, functionality tetap versi baru

---

## Current Compiler Binaries

### argonc_v218 (Latest)
- **Features**: async/await, generics, debugger, networking
- **File**: `self-host/argonc_v218`
- **Dockerfile**: Uses this by default

### argonc_v216 (Stable)
- **Features**: generics, debugger, networking
- **File**: `self-host/argonc_v216`
- **Use for**: Fallback jika v218 bermasalah

---

## Files Reference

| File | Description |
|------|-------------|
| `self-host/compiler.ar` | Source code compiler v2.19.0 |
| `self-host/argonc_v219` | Compiled binary v2.19.0 (pending) |
| `self-host/argonc_v218` | Compiled binary v2.18.0 |
| `self-host/argonc_v216` | Compiled binary v2.16.0 |
| `Dockerfile` | Build script dengan bootstrap |
| `stdlib/*.ar` | Standard library modules |

---

## Version History

- **v2.19.0**: WebAssembly target, WASM codegen
- **v2.18.0**: Async/await support
- **v2.17.0**: Debugger support  
- **v2.16.0**: Fixed duplicate function, generic types
- **v2.15.0**: Original with bootstrap bug

---

## Quick Commands

```bash
# Build Docker image
docker build -t argon-toolchain .

# Run program
./argon.sh run examples/hello.ar

# Compile program
./argon.sh build examples/hello.ar -o hello

# Run async example
./argon.sh run examples/async_example.ar

# Check compiler version
docker run --rm argon-toolchain argonc --version
```
