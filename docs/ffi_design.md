# Cryo FFI Design (v2.20.0)

## Overview

FFI (Foreign Function Interface) allows Cryo to call external C/Rust functions directly.

## Syntax

### Declaring External Functions

```cryo
// Declare C function
extern "C" fn printf(format: *i8, ...) -> i32;
extern "C" fn malloc(size: usize) -> *void;
extern "C" fn free(ptr: *void);

// Declare with specific library
extern "C" from "libm" {
    fn sin(x: f64) -> f64;
    fn cos(x: f64) -> f64;
    fn sqrt(x: f64) -> f64;
}
```

### Calling External Functions

```cryo
fn main() {
    // Call libc functions
    let ptr = malloc(1024);
    printf("Allocated %d bytes at %p\n", 1024, ptr);
    free(ptr);
    
    // Call math functions
    let angle = 3.14159 / 4.0;
    print("sin(π/4) = " + sin(angle));
}
```

### Pointers and Memory

```cryo
// Pointer types
let ptr: *i32 = null;
let cstr: *i8 = "hello";  // C string

// Pointer operations
let val = *ptr;           // Dereference
let addr = &variable;     // Address-of

// Array to pointer
let arr = [1, 2, 3];
let ptr = arr.as_ptr();
```

## Implementation

### Token Changes

```cryo
let TOK_EXTERN = 83;      // extern keyword (already exists)
let TOK_FROM = 63;        // from keyword (already exists)
let TOK_PTR = 90;         // * for pointers
let TOK_REF = 91;         // & for references
```

### AST Changes

```cryo
let AST_EXTERN_FN = 85;   // External function declaration
let AST_PTR_TYPE = 86;    // Pointer type *T
let AST_DEREF = 87;       // Dereference *expr
let AST_ADDR_OF = 88;     // Address-of &expr
```

### Code Generation

```llvm
; External function declaration
declare i32 @printf(i8*, ...)
declare i8* @malloc(i64)
declare void @free(i8*)

; Calling external function
%ptr = call i8* @malloc(i64 1024)
call void @free(i8* %ptr)
```

## Type Mappings

| Cryo Type | C Type | LLVM Type |
|------------|--------|-----------|
| i8 | char | i8 |
| i16 | short | i16 |
| i32 | int | i32 |
| i64 | long | i64 |
| f32 | float | float |
| f64 | double | double |
| *T | T* | T* |
| *void | void* | i8* |
| usize | size_t | i64 |

## Standard FFI Bindings

### libc.ar
```cryo
// stdlib/ffi/libc.ar
extern "C" fn malloc(size: usize) -> *void;
extern "C" fn free(ptr: *void);
extern "C" fn memcpy(dst: *void, src: *void, n: usize) -> *void;
extern "C" fn strlen(s: *i8) -> usize;
extern "C" fn strcmp(s1: *i8, s2: *i8) -> i32;
```

### libm.ar
```cryo
// stdlib/ffi/libm.ar
extern "C" fn sin(x: f64) -> f64;
extern "C" fn cos(x: f64) -> f64;
extern "C" fn tan(x: f64) -> f64;
extern "C" fn sqrt(x: f64) -> f64;
extern "C" fn pow(x: f64, y: f64) -> f64;
extern "C" fn log(x: f64) -> f64;
extern "C" fn exp(x: f64) -> f64;
```

## Safety

FFI functions are inherently unsafe:
- No bounds checking
- No null pointer checks
- Memory must be managed manually

```cryo
// Recommended: wrap in safe interface
fn safeMalloc(size: i64) -> Option<*void> {
    let ptr = malloc(size as usize);
    if (ptr == null) {
        return None;
    }
    return Some(ptr);
}
```

## Phase Implementation

1. **Phase 1**: extern "C" fn declarations
2. **Phase 2**: Pointer types (*T) and operations
3. **Phase 3**: from "lib" { } syntax
4. **Phase 4**: Standard FFI bindings
