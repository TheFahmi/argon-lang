# Cryo WebAssembly Design (v2.19.0)

## Overview

Add WebAssembly compilation target to Cryo, enabling:
- **Compile to WASM** - Run Cryo code in browsers
- **WASI Support** - WebAssembly System Interface for I/O
- **JavaScript Interop** - Call JS from Cryo, export to JS
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
cryoc --target wasm32 hello.cryo -o hello.wasm

# Compile with WASI support
cryoc --target wasm32-wasi hello.cryo -o hello.wasm
```

### Export Functions to JavaScript
```cryo
// Export function for JavaScript to call
@wasmExport("add")
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

@wasmExport("greet")
fn greet(name: string) {
    print("Hello, " + name + "!");
}
```

### Import Functions from JavaScript
```cryo
// Import JavaScript console.log
@wasmImport("console", "log")
extern fn jsLog(msg: string);

// Import custom JS function
@wasmImport("env", "get_time")
extern fn getTime() -> i64;

fn main() {
    jsLog("Hello from Cryo WASM!");
    let t = getTime();
    print("Time: " + t);
}
```

### Memory Management
```cryo
// Allocate WASM linear memory
let buffer = wasmAlloc(1024);  // 1KB

// Read/Write memory
wasmStoreI32(buffer, 0, 42);
let val = wasmLoadI32(buffer, 0);

// Free memory
wasmFree(buffer);
```

## Implementation Strategy

### Phase 1: WASM Codegen Backend
1. Add `TARGET_WASM32` constant
2. Add `--target wasm32` CLI option
3. Create WASM module structure generator
4. Emit WAT (WebAssembly Text) format first
5. Use `wat2wasm` to convert to binary

### Phase 2: WASM Instructions
1. Map Cryo types to WASM types
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
    source.cryo ──────►   Lexer/Parser  │
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

| Cryo Type | WASM Type | Note |
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
  (import "console" "log" (func $jsLog(param i32 i32)))
  
  ;; Cryo runtime functions
  (func $cryoAlloc(param $size i32) (result i32)
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
```cryo
// Cryo
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
```cryo
// Cryo
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
```cryo
// Cryo
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
```cryo
// Cryo
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
;; Import WASI fdWrite(import "wasi_snapshot_preview1" "fd_write" 
  (func $fdWrite(param i32 i32 i32 i32) (result i32)))

;; Print string
(func $print (param $ptr i32) (param $len i32)
  ;; Setup iovec
  i32.const 0          ;; iovec base offset
  local.get $ptr       ;; string pointer
  i32.store
  i32.const 4          ;; iovec len offset
  local.get $len       ;; string length
  i32.store
  
  ;; fdWrite(stdout=1, iovec_ptr=0, iovec_len=1, nwritten_ptr=8)
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
// cryo_loader.js
async function loadCryoModule(wasmPath) {
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
  const bytes = await response.cryorayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, importObject);
  
  return instance.exports;
}

// Usage
const cryo = await loadCryoModule('hello.wasm');
cryo.main();
console.log('add result:', cryo.add(5, 3));
```

## Browser Example

### HTML Demo
```html
<!DOCTYPE html>
<html>
<head>
  <title>Cryo WASM Demo</title>
</head>
<body>
  <h1>Cryo WebAssembly Demo</h1>
  <div id="output"></div>
  
  <script>
    async function runCryo() {
      const output = document.getElementById('output');
      
      // Redirect console.log to page
      const originalLog = console.log;
      console.log = (...cryogs) => {
        originalLog(...cryogs);
        output.innerHTML += args.join(' ') + '<br>';
      };
      
      const cryo = await loadCryoModule('demo.wasm');
      cryo.main();
    }
    
    runCryo();
  </script>
  <script src="cryo_loader.js"></script>
</body>
</html>
```

## Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `self-host/compiler.cryo` | Modify | Add WASM backend |
| `self-host/wasm_codegen.cryo` | Create | WASM code generator |
| `stdlib/wasm.cryo` | Create | WASM utilities |
| `examples/wasm_example.cryo` | Create | Basic WASM demo |
| `examples/wasm_demo.html` | Create | Browser demo |
| `examples/cryo_loader.js` | Create | JS loader |

## Compiler CLI Changes

```bash
# New options
cryoc [OPTIONS] <file.cryo>

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
cryoc --target wasm32 --emit wat hello.cryo -o hello.wat

# 2. Convert WAT to WASM (using wabt)
wat2wasm hello.wat -o hello.wasm

# Or one-step:
cryoc --target wasm32 hello.cryo -o hello.wasm
```

## Constants

### Target Constants
```cryo
let TARGET_NATIVE = 0;
let TARGET_WASM32 = 1;
let TARGET_WASM32_WASI = 2;
```

### Token IDs
```cryo
let TOK_WASM_EXPORT = 82;
let TOK_WASM_IMPORT = 83;
let TOK_EXTERN = 84;
let TOK_AT = 85;
```

### AST Node IDs
```cryo
let AST_WASM_EXPORT = 150;
let AST_WASM_IMPORT = 151;
let AST_EXTERN_FUNC = 152;
let AST_ATTRIBUTE = 153;
```

## Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| 1 | ✅ | Design document (`docs/wasm_design.md`) |
| 2 | ✅ | WAT text output (`self-host/wasm_codegen.cryo`) |
| 3 | ✅ | Basic arithmetic & functions (codegen done) |
| 4 | ✅ | Control flow (if/while) (codegen done) |
| 5 | ✅ | WASI print support (template done) |
| 6 | ✅ | JS interop (@wasm_export) - tokens added |
| 7 | ✅ | Browser demo (`examples/wasm_demo.html`) |
| 8 | ✅ | String/array support (`stdlib/wasm.cryo`) |
| 9 | ✅ | Integrate to main compiler CLI |
| 10 | 🔄 | Bootstrap new binary with WASM support |

### Completed Files
- ✅ `docs/wasm_design.md` - Design document
- ✅ `self-host/wasm_codegen.cryo` - Standalone WAT code generator
- ✅ `self-host/compiler.cryo` - WASM codegen integrated
- ✅ `stdlib/wasm.cryo` - WASM standard library  
- ✅ `examples/wasm_example.cryo` - Example program
- ✅ `examples/wasm_demo.html` - Browser demo
- ✅ `examples/cryo_loader.js` - JavaScript loader

### CLI Options Added
```bash
cryoc --target wasm32 hello.cryo        # Compile to WASM
cryoc --target wasm32-wasi hello.cryo   # Compile with WASI
cryoc -o output.wat hello.cryo          # Custom output file
cryoc --version                       # Show version
cryoc --help                          # Show help
```

### Remaining Work
- 🔄 Bootstrap new compiler binary with WASM support

## Example Program

```cryo
// examples/wasm_example.cryo
// Compile: cryoc --target wasm32 wasm_example.cryo -o demo.wasm

@wasmExport("add")
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

@wasmExport("factorial")
fn factorial(n: i32) -> i32 {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

@wasmExport("main")
fn main() {
    print("Hello from Cryo WASM!");
    print("5 + 3 = " + add(5, 3));
    print("5! = " + factorial(5));
}
```
