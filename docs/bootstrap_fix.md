# Argon Bootstrap Fix Documentation

## Problem

The Argon compiler v2.15.0 had a bootstrap issue where the Rust interpreter (`argon`) would delete the output file after its internal `clang` call failed.

### Root Causes Found

1. **Duplicate Function Definition**: `generate_specialized_funcs()` was defined twice in `compiler.ar` (at lines 1207 and 2284). This caused LLVM IR error: "invalid redefinition of function".

2. **Interpreter File Deletion Bug**: The Rust interpreter overwrites the input `.ar` file with LLVM IR, then calls `clang`. If `clang` fails (e.g., input=output conflict), the interpreter deletes the file.

## Solution

### Step 1: Remove Duplicate Function
```argon
// Removed first instance at line 1207
// Kept the more complete version at line 2284 (now 2238 after removal)
```

### Step 2: Capture LLVM IR Before Deletion
Use `inotifywait` to copy the file immediately when it's modified, before the interpreter can delete it:

```dockerfile
RUN bash -c ' \
    inotifywait -m -e modify /app/src.ar --format "%w%f" 2>/dev/null | while read f; do \
        cp /app/src.ar /app/compiler.ll 2>/dev/null; \
    done & \
    sleep 1; \
    /app/argon --emit-llvm /app/src.ar || true; \
    sleep 1; \
    kill %1 2>/dev/null \
'
```

### Step 3: Compile LLVM IR to Binary
```bash
clang++ -O2 -Wno-override-module /app/compiler.ll /usr/lib/libruntime_argon.a -lpthread -ldl -o /usr/bin/argonc
```

## Result

Successfully bootstrapped `argonc_v216` from Rust interpreter with the duplicate function fix applied.

## Files Changed

- `self-host/compiler.ar`: Removed duplicate `generate_specialized_funcs()` function
- `self-host/argonc_v216`: New bootstrapped compiler binary
- `Dockerfile`: Updated to use new compiler
- `stdlib/collections.ar`: Updated version header
- `README.md`: Updated version history

## Version

- **v2.15.0**: Original compiler with duplicate function bug
- **v2.16.0**: Fixed compiler with generic types and Rust interpreter bootstrap
