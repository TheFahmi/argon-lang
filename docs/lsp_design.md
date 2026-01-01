# Cryo Language Server Protocol (LSP) v2.0.0

## Overview

The Cryo LSP provides full IDE support for Cryo programming language.

## Features

### Phase 1: Basic Support ✅
- [x] Diagnostics (syntax errors, undefined variables)
- [x] Hover (show function signatures)
- [x] Document symbols (outline)

### Phase 2: Navigation ✅
- [x] Go to definition (Ctrl+Click / F12)
- [x] Find references (Shift+F12)
- [x] Document links (clickable imports)

### Phase 3: Editing ✅
- [x] Completion (autocomplete with snippets)
- [x] Signature help (parameter hints)
- [x] Formatting (auto-indent)

## Architecture

```
┌─────────────────┐     ┌─────────────────┐
│   VS Code /     │────▶│   cryo-lsp     │
│   Any Editor    │◀────│   (Node.js)     │
└─────────────────┘     └────────┬────────┘
                                 │
                        ┌────────▼────────┐
                        │  Symbol Cache   │
                        │  & Parser       │
                        └─────────────────┘
```

## Protocol Implementation

The LSP uses JSON-RPC over stdin/stdout.

### Methods Implemented

| Method | Phase | Description |
|--------|-------|-------------|
| `initialize` | 1 | Initialize server |
| `textDocument/didOpen` | 1 | Document opened |
| `textDocument/didChange` | 1 | Document changed |
| `textDocument/didSave` | 1 | Document saved |
| `textDocument/hover` | 1 | Show hover info |
| `textDocument/documentSymbol` | 1 | Get outline |
| `textDocument/publishDiagnostics` | 1 | Send errors |
| `textDocument/definition` | 2 | Go to definition |
| `textDocument/references` | 2 | Find all references |
| `textDocument/documentLink` | 2 | Clickable imports |
| `textDocument/completion` | 3 | Autocomplete |
| `textDocument/signatureHelp` | 3 | Parameter hints |
| `textDocument/formatting` | 3 | Format document |

## Usage

### VS Code Extension
Install the Cryo extension from marketplace (coming soon).

### Manual Setup
1. Install Node.js
2. Run: `node lsp/cryo-lsp.js --stdio`
3. Configure your editor to use this command

## Features Detail

### Autocomplete

Provides completions for:
- **Keywords**: `fn`, `let`, `if`, `while`, `struct`, `enum`, `match`, `import`
- **Built-ins**: `print`, `len`, `push`, `parseInt`, `substring`, etc.
- **User symbols**: Functions, structs, enums, variables from current file
- **Snippets**: Smart completions with placeholders

Example:
```
fn|  →  fn ${1:name}(${2:params}) {
            ${3}
        }
```

### Signature Help

Shows parameter hints when typing function calls:
```
print(|)
       ↓
fn print(value: any)
         ^^^^^^^^^
         active parameter
```

### Go to Definition

- Ctrl+Click or F12 on any symbol
- Works for functions, structs, enums, variables
- Cross-file support for imported modules

### Find References

- Shift+F12 to find all usages
- Shows all occurrences in current file
- Highlights matches

### Document Formatting

- Ctrl+Shift+I to format
- Auto-indentation (4 spaces)
- Consistent brace alignment

## Symbol Types

The LSP recognizes:
- `fn name(...)` - Functions with parameters
- `struct Name { ... }` - Structs with fields
- `enum Name { ... }` - Enums with variants
- `let name = ...` - Variables
- `import "..."` - Module imports

## Diagnostics

Real-time error checking:
- Unmatched braces `{` `}`
- Missing semicolons (heuristic)
- Empty `print()` calls
- More to come...

## Configuration

No configuration required. The extension works out of the box.

## Version History

- **v2.0.0**: Added Phase 2 (Navigation) and Phase 3 (Editing) support
- **v1.0.0**: Initial release with Basic support
