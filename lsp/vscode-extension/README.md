# Argon VS Code Extension

## Features

- **Syntax Highlighting**: Full syntax highlighting for `.ar` files
- **Document Outline**: View functions, structs, and enums
- **Hover Information**: See function signatures and types
- **Diagnostics**: Basic error checking

## Installation

### From Source

1. Make sure Node.js is installed
2. Navigate to the extension directory:
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
   - Choose the generated `.vsix` file

### Development Mode

1. Open the `lsp/vscode-extension` folder in VS Code
2. Press `F5` to launch Extension Development Host
3. Open any `.ar` file

## Usage

### Syntax Highlighting
Automatically enabled for all `.ar` files.

### Document Symbols
- Press `Ctrl+Shift+O` to see outline
- Shows functions, structs, enums, and imports

### Hover
- Hover over function names to see signatures
- Works for built-in functions and user-defined functions

## Troubleshooting

If the language server doesn't start:
1. Check Node.js is installed: `node --version`
2. Check the output panel: View > Output > Argon Language Server
3. Verify the server path in extension.js

## Configuration

No configuration required. The extension works out of the box.

## Screenshots

*Coming soon*

## Contributing

Contributions welcome! The language server is in `lsp/argon-lsp.js`.
