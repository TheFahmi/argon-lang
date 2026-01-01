# Cryo VS Code Extension v2.0.0

Full IDE support for the Cryo programming language.

## Features

### ✅ Syntax Highlighting
Beautiful syntax highlighting for all Cryo constructs.

### ✅ Autocomplete (Phase 3)
- **Keywords**: `fn`, `let`, `if`, `while`, `struct`, `enum`, `match`, `import`
- **Built-in functions**: `print`, `len`, `push`, `parseInt`, etc.
- **User-defined symbols**: Functions, structs, enums, variables
- **Smart snippets**: Templates with tab stops

### ✅ Go to Definition (Phase 2)
- `Ctrl+Click` or `F12` on any symbol
- Jump to function/struct/enum definitions
- Works across imported files

### ✅ Find All References (Phase 2)
- `Shift+F12` to find all usages
- Highlights all occurrences

### ✅ Signature Help (Phase 3)
- Parameter hints when typing function calls
- Shows active parameter

### ✅ Document Formatting (Phase 3)
- `Ctrl+Shift+I` to format
- Consistent 4-space indentation
- Proper brace alignment

### ✅ Document Outline (Phase 1)
- `Ctrl+Shift+O` for outline view
- Shows functions, structs, enums, imports

### ✅ Hover Information (Phase 1)
- Hover over any symbol for documentation
- Shows function signatures and definitions

### ✅ Diagnostics (Phase 1)
- Real-time error detection
- Unmatched braces
- Missing semicolons

### ✅ Snippets
Quick code templates:
- `fn` → Function definition
- `main` → Main entry point
- `struct` → Struct definition
- `enum` → Enum definition
- `match` → Match expression
- `if` / `ife` → If / If-else
- `while` / `for` → Loops
- `import` / `stdlib` → Imports
- `test` → Test function template

## Installation

### From Source

1. Install Node.js
2. Navigate to extension directory:
   ```bash
   cd lsp/vscode-extension
   ```
3. Install dependencies:
   ```bash
   npm install
   ```
4. Package the extension:
   ```bash
   npx vsce package
   ```
5. Install in VS Code:
   - Press `Ctrl+Shift+P`
   - Select "Extensions: Install from VSIX..."
   - Choose the `.vsix` file

### Development Mode

1. Open `lsp/vscode-extension` in VS Code
2. Press `F5` for Extension Development Host
3. Open any `.ar` file

## Keyboard Shortcuts

| Action | Windows/Linux | Mac |
|--------|---------------|-----|
| Go to Definition | `F12` or `Ctrl+Click` | `F12` or `Cmd+Click` |
| Find References | `Shift+F12` | `Shift+F12` |
| Format Document | `Ctrl+Shift+I` | `Cmd+Shift+I` |
| Document Outline | `Ctrl+Shift+O` | `Cmd+Shift+O` |
| Trigger Autocomplete | `Ctrl+Space` | `Cmd+Space` |
| Parameter Hints | `Ctrl+Shift+Space` | `Cmd+Shift+Space` |

## Standard Library Modules

The autocomplete includes all 19 stdlib modules:
- `math`, `string`, `array`, `json`, `http`
- `fs`, `time`, `csv`, `testing`, `color`
- `console`, `validation`, `random`, `crypto`
- `env`, `set`, `process`, `regex`, `collections`

## Version History

- **v2.0.0**: Full LSP support (Navigation + Editing)
  - Go to definition
  - Find references
  - Autocomplete with snippets
  - Signature help
  - Formatting
- **v1.0.0**: Initial release
  - Syntax highlighting
  - Document symbols
  - Hover
  - Diagnostics

## Contributing

Contributions welcome!
- Language Server: `lsp/cryo-lsp.js`
- Extension: `lsp/vscode-extension/`
