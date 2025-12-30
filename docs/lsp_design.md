# Argon Language Server Protocol (LSP)

## Overview

The Argon LSP provides IDE support for Argon programming language.

## Features

### Phase 1: Basic Support
- [x] Diagnostics (syntax errors, undefined variables)
- [x] Hover (show function signatures)
- [x] Document symbols (outline)

### Phase 2: Navigation
- [ ] Go to definition
- [ ] Find references
- [ ] Document links (imports)

### Phase 3: Editing
- [ ] Completion (autocomplete)
- [ ] Signature help
- [ ] Formatting

## Architecture

```
┌─────────────────┐     ┌─────────────────┐
│   VS Code /     │────▶│   argon-lsp     │
│   Any Editor    │◀────│   (Node.js)     │
└─────────────────┘     └────────┬────────┘
                                 │
                        ┌────────▼────────┐
                        │  Argon Parser   │
                        │  (simplified)   │
                        └─────────────────┘
```

## Protocol Implementation

The LSP uses JSON-RPC over stdin/stdout.

### Methods Implemented

| Method | Description |
|--------|-------------|
| `initialize` | Initialize server |
| `textDocument/didOpen` | Document opened |
| `textDocument/didChange` | Document changed |
| `textDocument/didSave` | Document saved |
| `textDocument/hover` | Show hover info |
| `textDocument/documentSymbol` | Get outline |
| `textDocument/publishDiagnostics` | Send errors |

## Usage

### VS Code Extension
Install the Argon extension from marketplace (coming soon).

### Manual Setup
1. Install Node.js
2. Run: `node lsp/argon-lsp.js --stdio`
3. Configure your editor to use this command

## Symbol Types

The LSP recognizes:
- Functions (`fn name(...)`)
- Structs (`struct Name { ... }`)
- Enums (`enum Name { ... }`)
- Variables (`let name = ...`)
- Imports (`import "..."`)
