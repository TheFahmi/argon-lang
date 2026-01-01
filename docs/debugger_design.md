# Cryo Debugger Support Design (v2.17.0)

## Overview

Add full debugging support to Cryo, enabling developers to:
- Set breakpoints in source code
- Step through execution (step over, step into, step out)
- Inspect variables at runtime
- View call stack
- Use standard debuggers (GDB, LLDB)

## Implementation Status ✅

| Feature | Status |
|---------|--------|
| Source line tracking in lexer | ✅ Implemented |
| DEBUG_MODE flag (-g/--debug) | ✅ Implemented |
| DICompileUnit metadata | ✅ Implemented |
| DIFile metadata | ✅ Implemented |
| DIBasicType metadata | ✅ Implemented |
| DISubroutineType metadata | ✅ Implemented |
| DISubprogram per function | ✅ Implemented |
| Function !dbg reference | ✅ Implemented |
| Instruction !dbg location | ✅ Implemented |
| GDB in Docker | ✅ Implemented |
| `cryo debug` command | ✅ Implemented |

## Architecture

### Phase 1: DWARF Debug Info in LLVM IR

Generate LLVM debug metadata that maps to Cryo source:

```llvm
!llvm.dbg.cu = !{!0}
!0 = distinct !DICompileUnit(language: DW_LANG_C, file: !1, producer: "cryoc v2.17.0")
!1 = !DIFile(filename: "example.cryo", directory: "/app")

define i64 @myFunction() !dbg !2 {
  ; ...
  call void @llvm.dbg.value(metadata i64 %x, metadata !3, metadata !DIExpression()), !dbg !4
}

!2 = distinct !DISubprogram(name: "my_function", file: !1, line: 10)
!3 = !DILocalVariable(name: "x", scope: !2, file: !1, line: 11, type: !5)
!4 = !DILocation(line: 11, column: 5, scope: !2)
!5 = !DIBasicType(name: "int", size: 64, encoding: DW_ATE_signed)
```

### Phase 2: Compiler Changes

1. **Track source locations** during lexing (line, column for each token)
2. **Propagate locations** through parser to AST nodes
3. **Emit debug metadata** during code generation
4. **Add -g flag** to enable debug mode

### Phase 3: Debug Commands

Add runtime debugging support:
- `cryo debug <file>` - Start debugging session
- Integration with GDB/LLDB

## Implementation Plan

### Step 1: Source Location Tracking (Lexer)
- Add `line` and `col` fields to each token
- Track current line/column during tokenization

### Step 2: AST Location Propagation (Parser)
- Add `loc` field to AST nodes
- Store source location for each statement/expression

### Step 3: Debug Metadata Emission (Code Generator)
- Generate DICompileUnit, DIFile, DISubprogram
- Generate DILocalVariable for let bindings
- Generate DILocation for each instruction
- Add llvm.dbg.value calls for variable tracking

### Step 4: Compiler Flag
- Add `-g` or `--debug` flag to cryoc
- Only emit debug info when flag is present

### Step 5: Debug Script
- Create `cryo.sh debug` command
- Invoke GDB/LLDB with proper settings

## Debug Metadata Format

### Compile Unit (per file)
```
!DICompileUnit(
  language: DW_LANG_C,
  file: !<file>,
  producer: "cryoc v2.17.0",
  isOptimized: false,
  emissionKind: FullDebug
)
```

### Function (per fn)
```
!DISubprogram(
  name: "function_name",
  linkageName: "function_name", 
  scope: !<file>,
  file: !<file>,
  line: <line_number>,
  type: !<function_type>,
  isLocal: false,
  isDefinition: true,
  scopeLine: <line_number>,
  unit: !<compile_unit>
)
```

### Variable (per let)
```
!DILocalVariable(
  name: "var_name",
  scope: !<function>,
  file: !<file>,
  line: <line_number>,
  type: !<type>
)
```

### Location (per statement)
```
!DILocation(
  line: <line>,
  column: <col>,
  scope: !<scope>
)
```

## Usage Example

```bash
# Compile with debug info
cryo compile -g myprogram.ar

# Debug with GDB
gdb ./myprogram.ar.out

# In GDB:
(gdb) break main
(gdb) run
(gdb) next
(gdb) print x
(gdb) backtrace
```

## Files to Modify

1. `self-host/compiler.ar` - Add debug metadata emission
2. `cryo.sh` - Add `debug` command
3. `Dockerfile` - Include GDB

## Testing

Create test file with:
- Multiple functions
- Local variables
- Control flow (if, while)
- Verify breakpoints work
- Verify variable inspection works
