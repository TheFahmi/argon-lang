# Argon WebAssembly Design (v2.19.0)

## Overview

Add WebAssembly compilation target to Argon, enabling:
- **Compile to WASM** - Run Argon code in browsers
- **WASI Support** - WebAssembly System Interface for I/O
- **JavaScript Interop** - Call JS from Argon, export to JS
- **Browser Examples** - Demo running in browser

## Goals

| Goal | Description |
|------|-------------|
| `--target wasm32` | Compile to WebAssembly binary (.wasm) |
| `@wasm_export` | Export functions to JavaScript |
| `@wasm_import` | Import functions from JavaScript |
| WASI I/O | Print, file read/write via WASI |
| Browser demo | Working example in HTML page |

## Syntax

### Basic Compilation
```bash
# Compile to WASM
argonc --target wasm32 hello.ar -o hello.wasm

# Compile with WASI support
argonc --target wasm32-wasi hello.ar -o hello.wasm
```

### Export Functions to JavaScript
```argon
// Export function for JavaScript to call
@wasm_export("add")
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

@wasm_export("greet")
fn greet(name: string) {
    print("Hello, " + name + "!");
}
```

### Import Functions from JavaScript
```argon
// Import JavaScript console.log
@wasm_import("console", "log")
extern fn js_log(msg: string);

// Import custom JS function
@wasm_import("env", "get_time")
extern fn get_time() -> i64;

fn main() {
    js_log("Hello from Argon WASM!");
    let t = get_time();
    print("Time: " + t);
}
```

### Memory Management
```argon
// Allocate WASM linear memory
let buffer = wasm_alloc(1024);  // 1KB

// Read/Write memory
wasm_store_i32(buffer, 0, 42);
let val = wasm_load_i32(buffer, 0);

// Free memory
wasm_free(buffer);
```

## Implementation Strategy

### Phase 1: WASM Codegen Backend
1. Add `TARGET_WASM32` constant
2. Add `--target wasm32` CLI option
3. Create WASM module structure generator
4. Emit WAT (WebAssembly Text) format first
5. Use `wat2wasm` to convert to binary

### Phase 2: WASM Instructions
1. Map Argon types to WASM types
   - `i32` → `i32`
   - `i64` → `i64`
   - `f32` → `f32`
   - `f64` → `f64`
   - `string` → `i32` (pointer + length)
   - `array` → `i32` (pointer)
2. Generate WASM instructions for arithmetic
3. Generate WASM instructions for control flow
4. Generate WASM instructions for function calls

### Phase 3: WASI Support
1. Import WASI functions for I/O
2. Map `print()` to `fd_write`
3. Map file operations to WASI file I/O
4. Support command-line arguments

### Phase 4: JS Interop
1. Parse `@wasm_export` attribute
2. Parse `@wasm_import` attribute  
3. Generate export section
4. Generate import section
5. Create JS wrapper/glue code

## Architecture

```
                    ┌─────────────────┐
    source.ar ──────►   Lexer/Parser  │
                    └────────┬────────┘
                             │ AST
                             ▼
                    ┌─────────────────┐
                    │   Type Checker  │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
       ┌───────────┐  ┌───────────┐  ┌───────────┐
       │ LLVM IR   │  │   WAT     │  │   C       │
       │ Backend   │  │ Backend   │  │ Backend   │
       └─────┬─────┘  └─────┬─────┘  └───────────┘
             │              │
             ▼              ▼
        native binary   .wasm file
```

## Token & AST Changes

### New Tokens
```
TOK_WASM_EXPORT = 82   // @wasm_export
TOK_WASM_IMPORT = 83   // @wasm_import
TOK_EXTERN = 84        // extern fn
TOK_AT = 85            // @ (attribute prefix)
```

### New AST Nodes
```
AST_WASM_EXPORT = 150  // [150, name, export_name]
AST_WASM_IMPORT = 151  // [151, module, name, fn_signature]
AST_EXTERN_FUNC = 152  // [152, name, params, return_type]
AST_ATTRIBUTE = 153    // [153, attr_name, attr_args]
```

## WASM Type Mapping

| Argon Type | WASM Type | Note |
|------------|-----------|------|
| `int` | `i64` | 64-bit integer |
| `i32` | `i32` | 32-bit integer |
| `i64` | `i64` | 64-bit integer |
| `f32` | `f32` | 32-bit float |
| `f64` | `f64` | 64-bit float |
| `bool` | `i32` | 0 or 1 |
| `string` | `i32, i32` | ptr, len |
| `array` | `i32` | ptr to memory |
| `struct` | `i32` | ptr to memory |

## WASM Module Structure (WAT)

```wat
(module
  ;; Memory
  (memory (export "memory") 1)
  
  ;; Imports from JavaScript
  (import "console" "log" (func $js_log (param i32 i32)))
  
  ;; Argon runtime functions
  (func $argon_alloc (param $size i32) (result i32)
    ;; Simple bump allocator
    ...
  )
  
  ;; User functions
  (func $add (export "add") (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
  
  ;; Main function
  (func $main (export "_start")
    ;; User code
  )
)
```

## WASM Code Generation

### Arithmetic Operations
```argon
// Argon
let x = a + b * c;
```
```wat
;; WAT
local.get $a
local.get $b
local.get $c
i64.mul
i64.add
local.set $x
```

### If Statement
```argon
// Argon
if (x > 0) {
    print("positive");
}
```
```wat
;; WAT
local.get $x
i64.const 0
i64.gt_s
if
  ;; call print
end
```

### While Loop
```argon
// Argon
while (i < 10) {
    i = i + 1;
}
```
```wat
;; WAT
block $break
  loop $continue
    local.get $i
    i64.const 10
    i64.lt_s
    i32.eqz
    br_if $break
    
    local.get $i
    i64.const 1
    i64.add
    local.set $i
    
    br $continue
  end
end
```

### Function Call
```argon
// Argon
let result = add(1, 2);
```
```wat
;; WAT
i64.const 1
i64.const 2
call $add
local.set $result
```

## WASI Support

### Print Implementation
```wat
;; Import WASI fd_write
(import "wasi_snapshot_preview1" "fd_write" 
  (func $fd_write (param i32 i32 i32 i32) (result i32)))

;; Print string
(func $print (param $ptr i32) (param $len i32)
  ;; Setup iovec
  i32.const 0          ;; iovec base offset
  local.get $ptr       ;; string pointer
  i32.store
  i32.const 4          ;; iovec len offset
  local.get $len       ;; string length
  i32.store
  
  ;; fd_write(stdout=1, iovec_ptr=0, iovec_len=1, nwritten_ptr=8)
  i32.const 1
  i32.const 0
  i32.const 1
  i32.const 8
  call $fd_write
  drop
)
```

## JavaScript Glue Code

### Auto-generated JS loader
```javascript
// argon_loader.js
async function loadArgonModule(wasmPath) {
  const memory = new WebAssembly.Memory({ initial: 256 });
  
  const importObject = {
    env: {
      memory: memory,
    },
    console: {
      log: (ptr, len) => {
        const bytes = new Uint8Array(memory.buffer, ptr, len);
        console.log(new TextDecoder().decode(bytes));
      },
    },
  };
  
  const response = await fetch(wasmPath);
  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, importObject);
  
  return instance.exports;
}

// Usage
const argon = await loadArgonModule('hello.wasm');
argon.main();
console.log('add result:', argon.add(5, 3));
```

## Browser Example

### HTML Demo
```html
<!DOCTYPE html>
<html>
<head>
  <title>Argon WASM Demo</title>
</head>
<body>
  <h1>Argon WebAssembly Demo</h1>
  <div id="output"></div>
  
  <script>
    async function runArgon() {
      const output = document.getElementById('output');
      
      // Redirect console.log to page
      const originalLog = console.log;
      console.log = (...args) => {
        originalLog(...args);
        output.innerHTML += args.join(' ') + '<br>';
      };
      
      const argon = await loadArgonModule('demo.wasm');
      argon.main();
    }
    
    runArgon();
  </script>
  <script src="argon_loader.js"></script>
</body>
</html>
```

## Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `self-host/compiler.ar` | Modify | Add WASM backend |
| `self-host/wasm_codegen.ar` | Create | WASM code generator |
| `stdlib/wasm.ar` | Create | WASM utilities |
| `examples/wasm_example.ar` | Create | Basic WASM demo |
| `examples/wasm_demo.html` | Create | Browser demo |
| `examples/argon_loader.js` | Create | JS loader |

## Compiler CLI Changes

```bash
# New options
argonc [OPTIONS] <file.ar>

Options:
  --target <target>     Compilation target
                        - native (default)
                        - wasm32
                        - wasm32-wasi
  --emit <format>       Output format
                        - binary (default)
                        - wat (WebAssembly Text)
                        - llvm (LLVM IR)
  -o <output>           Output file name
```

## Build Pipeline

```bash
# 1. Compile to WAT
argonc --target wasm32 --emit wat hello.ar -o hello.wat

# 2. Convert WAT to WASM (using wabt)
wat2wasm hello.wat -o hello.wasm

# Or one-step:
argonc --target wasm32 hello.ar -o hello.wasm
```

## Constants

### Target Constants
```argon
let TARGET_NATIVE = 0;
let TARGET_WASM32 = 1;
let TARGET_WASM32_WASI = 2;
```

### Token IDs
```argon
let TOK_WASM_EXPORT = 82;
let TOK_WASM_IMPORT = 83;
let TOK_EXTERN = 84;
let TOK_AT = 85;
```

### AST Node IDs
```argon
let AST_WASM_EXPORT = 150;
let AST_WASM_IMPORT = 151;
let AST_EXTERN_FUNC = 152;
let AST_ATTRIBUTE = 153;
```

## Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| 1 | ✅ | Design document (`docs/wasm_design.md`) |
| 2 | ✅ | WAT text output (`self-host/wasm_codegen.ar`) |
| 3 | ✅ | Basic arithmetic & functions (codegen done) |
| 4 | ✅ | Control flow (if/while) (codegen done) |
| 5 | ✅ | WASI print support (template done) |
| 6 | ✅ | JS interop (@wasm_export) - tokens added |
| 7 | ✅ | Browser demo (`examples/wasm_demo.html`) |
| 8 | ✅ | String/array support (`stdlib/wasm.ar`) |
| 9 | ✅ | Integrate to main compiler CLI |
| 10 | 🔄 | Bootstrap new binary with WASM support |

### Completed Files
- ✅ `docs/wasm_design.md` - Design document
- ✅ `self-host/wasm_codegen.ar` - Standalone WAT code generator
- ✅ `self-host/compiler.ar` - WASM codegen integrated
- ✅ `stdlib/wasm.ar` - WASM standard library  
- ✅ `examples/wasm_example.ar` - Example program
- ✅ `examples/wasm_demo.html` - Browser demo
- ✅ `examples/argon_loader.js` - JavaScript loader

### CLI Options Added
```bash
argonc --target wasm32 hello.ar        # Compile to WASM
argonc --target wasm32-wasi hello.ar   # Compile with WASI
argonc -o output.wat hello.ar          # Custom output file
argonc --version                       # Show version
argonc --help                          # Show help
```

### Remaining Work
- 🔄 Bootstrap new compiler binary with WASM support

## Example Program

```argon
// examples/wasm_example.ar
// Compile: argonc --target wasm32 wasm_example.ar -o demo.wasm

@wasm_export("add")
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

@wasm_export("factorial")
fn factorial(n: i32) -> i32 {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

@wasm_export("main")
fn main() {
    print("Hello from Argon WASM!");
    print("5 + 3 = " + add(5, 3));
    print("5! = " + factorial(5));
}
```
