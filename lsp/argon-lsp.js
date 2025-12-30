#!/usr/bin/env node
// ============================================
// ARGON LANGUAGE SERVER v1.0.0
// Language Server Protocol implementation
// ============================================

const readline = require('readline');

// State
let documents = new Map();
let initialized = false;

// Logging
function log(msg) {
    console.error(`[argon-lsp] ${msg}`);
}

// Send JSON-RPC response
function sendResponse(id, result) {
    const response = {
        jsonrpc: '2.0',
        id: id,
        result: result
    };
    const content = JSON.stringify(response);
    const header = `Content-Length: ${Buffer.byteLength(content)}\r\n\r\n`;
    process.stdout.write(header + content);
}

// Send JSON-RPC notification
function sendNotification(method, params) {
    const notification = {
        jsonrpc: '2.0',
        method: method,
        params: params
    };
    const content = JSON.stringify(notification);
    const header = `Content-Length: ${Buffer.byteLength(content)}\r\n\r\n`;
    process.stdout.write(header + content);
}

// Parse Argon source for symbols
function parseSymbols(text) {
    const symbols = [];
    const lines = text.split('\n');

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmed = line.trim();

        // Function: fn name(...)
        const fnMatch = trimmed.match(/^fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
        if (fnMatch) {
            symbols.push({
                name: fnMatch[1],
                kind: 12, // Function
                range: {
                    start: { line: i, character: 0 },
                    end: { line: i, character: line.length }
                },
                selectionRange: {
                    start: { line: i, character: line.indexOf(fnMatch[1]) },
                    end: { line: i, character: line.indexOf(fnMatch[1]) + fnMatch[1].length }
                }
            });
        }

        // Struct: struct Name { ... }
        const structMatch = trimmed.match(/^struct\s+([a-zA-Z_][a-zA-Z0-9_]*)/);
        if (structMatch) {
            symbols.push({
                name: structMatch[1],
                kind: 23, // Struct
                range: {
                    start: { line: i, character: 0 },
                    end: { line: i, character: line.length }
                },
                selectionRange: {
                    start: { line: i, character: line.indexOf(structMatch[1]) },
                    end: { line: i, character: line.indexOf(structMatch[1]) + structMatch[1].length }
                }
            });
        }

        // Enum: enum Name { ... }
        const enumMatch = trimmed.match(/^enum\s+([a-zA-Z_][a-zA-Z0-9_]*)/);
        if (enumMatch) {
            symbols.push({
                name: enumMatch[1],
                kind: 10, // Enum
                range: {
                    start: { line: i, character: 0 },
                    end: { line: i, character: line.length }
                },
                selectionRange: {
                    start: { line: i, character: line.indexOf(enumMatch[1]) },
                    end: { line: i, character: line.indexOf(enumMatch[1]) + enumMatch[1].length }
                }
            });
        }

        // Let: let name = ...
        const letMatch = trimmed.match(/^let\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=/);
        if (letMatch) {
            symbols.push({
                name: letMatch[1],
                kind: 13, // Variable
                range: {
                    start: { line: i, character: 0 },
                    end: { line: i, character: line.length }
                },
                selectionRange: {
                    start: { line: i, character: line.indexOf(letMatch[1]) },
                    end: { line: i, character: line.indexOf(letMatch[1]) + letMatch[1].length }
                }
            });
        }

        // Import: import "..."
        const importMatch = trimmed.match(/^import\s+"([^"]+)"/);
        if (importMatch) {
            symbols.push({
                name: importMatch[1],
                kind: 2, // Module
                range: {
                    start: { line: i, character: 0 },
                    end: { line: i, character: line.length }
                },
                selectionRange: {
                    start: { line: i, character: line.indexOf(importMatch[1]) },
                    end: { line: i, character: line.indexOf(importMatch[1]) + importMatch[1].length }
                }
            });
        }
    }

    return symbols;
}

// Get diagnostics for document
function getDiagnostics(text, uri) {
    const diagnostics = [];
    const lines = text.split('\n');

    let braceDepth = 0;
    let parenDepth = 0;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // Skip comments
        if (line.trim().startsWith('//')) continue;

        // Count braces and parens
        for (let j = 0; j < line.length; j++) {
            const c = line[j];
            if (c === '{') braceDepth++;
            if (c === '}') braceDepth--;
            if (c === '(') parenDepth++;
            if (c === ')') parenDepth--;
        }

        // Check for common errors

        // Missing semicolon after let/return (heuristic)
        if (/^\s*(let|return)\s+.+[^;{]\s*$/.test(line) && !line.includes('{')) {
            // Check if next line is not a continuation
            if (i + 1 < lines.length) {
                const nextLine = lines[i + 1].trim();
                if (nextLine && !nextLine.startsWith('}') && !nextLine.startsWith('{')) {
                    diagnostics.push({
                        range: {
                            start: { line: i, character: line.length - 1 },
                            end: { line: i, character: line.length }
                        },
                        severity: 2, // Warning
                        message: 'Possible missing semicolon',
                        source: 'argon'
                    });
                }
            }
        }

        // Empty print/function call
        if (/print\(\s*\)/.test(line)) {
            const idx = line.indexOf('print()');
            diagnostics.push({
                range: {
                    start: { line: i, character: idx },
                    end: { line: i, character: idx + 7 }
                },
                severity: 2, // Warning
                message: 'print() called without arguments',
                source: 'argon'
            });
        }
    }

    // Unmatched braces
    if (braceDepth !== 0) {
        diagnostics.push({
            range: {
                start: { line: lines.length - 1, character: 0 },
                end: { line: lines.length - 1, character: 1 }
            },
            severity: 1, // Error
            message: braceDepth > 0 ? 'Unmatched opening brace' : 'Unmatched closing brace',
            source: 'argon'
        });
    }

    return diagnostics;
}

// Get hover info at position
function getHover(text, line, character) {
    const lines = text.split('\n');
    if (line >= lines.length) return null;

    const lineText = lines[line];

    // Find word at position
    let start = character;
    let end = character;

    while (start > 0 && /[a-zA-Z0-9_]/.test(lineText[start - 1])) start--;
    while (end < lineText.length && /[a-zA-Z0-9_]/.test(lineText[end])) end++;

    const word = lineText.substring(start, end);
    if (!word) return null;

    // Built-in functions
    const builtins = {
        'print': 'fn print(value: any) -> void\n\nPrint value to stdout',
        'len': 'fn len(value: any) -> int\n\nGet length of string or array',
        'push': 'fn push(array: any, item: any) -> array\n\nAppend item to array, return new array',
        'parseInt': 'fn parseInt(s: string) -> int\n\nParse string to integer',
        'char_code_at': 'fn char_code_at(s: string, i: int) -> int\n\nGet ASCII code of character at index',
        'substring': 'fn substring(s: string, start: int, end: int) -> string\n\nExtract substring',
        'sleep': 'fn argon_sleep(ms: int) -> void\n\nSleep for milliseconds'
    };

    if (builtins[word]) {
        return {
            contents: {
                kind: 'markdown',
                value: '```argon\n' + builtins[word] + '\n```'
            }
        };
    }

    // Find function definition in document
    const fnRegex = new RegExp(`^\\s*fn\\s+${word}\\s*\\(([^)]*)\\)`);
    for (let i = 0; i < lines.length; i++) {
        const match = lines[i].match(fnRegex);
        if (match) {
            return {
                contents: {
                    kind: 'markdown',
                    value: `\`\`\`argon\nfn ${word}(${match[1]})\n\`\`\`\n*Defined on line ${i + 1}*`
                }
            };
        }
    }

    // Find struct definition
    const structRegex = new RegExp(`^\\s*struct\\s+${word}\\s*\\{`);
    for (let i = 0; i < lines.length; i++) {
        if (structRegex.test(lines[i])) {
            return {
                contents: {
                    kind: 'markdown',
                    value: `\`\`\`argon\nstruct ${word}\n\`\`\`\n*Defined on line ${i + 1}*`
                }
            };
        }
    }

    return null;
}

// Handle JSON-RPC request
function handleRequest(message) {
    const { id, method, params } = message;

    switch (method) {
        case 'initialize':
            log('Initializing Argon Language Server');
            initialized = true;
            sendResponse(id, {
                capabilities: {
                    textDocumentSync: 1, // Full sync
                    hoverProvider: true,
                    documentSymbolProvider: true
                },
                serverInfo: {
                    name: 'argon-lsp',
                    version: '1.0.0'
                }
            });
            break;

        case 'initialized':
            log('Server initialized');
            break;

        case 'shutdown':
            log('Shutting down');
            sendResponse(id, null);
            break;

        case 'exit':
            process.exit(0);
            break;

        case 'textDocument/didOpen':
            {
                const { uri, text } = params.textDocument;
                documents.set(uri, text);
                log(`Opened: ${uri}`);

                // Send diagnostics
                const diagnostics = getDiagnostics(text, uri);
                sendNotification('textDocument/publishDiagnostics', {
                    uri: uri,
                    diagnostics: diagnostics
                });
            }
            break;

        case 'textDocument/didChange':
            {
                const { uri } = params.textDocument;
                const text = params.contentChanges[0].text;
                documents.set(uri, text);

                // Send diagnostics
                const diagnostics = getDiagnostics(text, uri);
                sendNotification('textDocument/publishDiagnostics', {
                    uri: uri,
                    diagnostics: diagnostics
                });
            }
            break;

        case 'textDocument/didSave':
            log(`Saved: ${params.textDocument.uri}`);
            break;

        case 'textDocument/hover':
            {
                const { uri } = params.textDocument;
                const { line, character } = params.position;
                const text = documents.get(uri) || '';
                const hover = getHover(text, line, character);
                sendResponse(id, hover);
            }
            break;

        case 'textDocument/documentSymbol':
            {
                const { uri } = params.textDocument;
                const text = documents.get(uri) || '';
                const symbols = parseSymbols(text);
                sendResponse(id, symbols);
            }
            break;

        default:
            if (id !== undefined) {
                sendResponse(id, null);
            }
    }
}

// Read LSP messages
function readMessages() {
    let buffer = '';
    let contentLength = -1;

    process.stdin.setEncoding('utf8');

    process.stdin.on('data', (chunk) => {
        buffer += chunk;

        while (true) {
            if (contentLength === -1) {
                // Look for Content-Length header
                const headerEnd = buffer.indexOf('\r\n\r\n');
                if (headerEnd === -1) break;

                const header = buffer.substring(0, headerEnd);
                const match = header.match(/Content-Length:\s*(\d+)/i);
                if (!match) {
                    buffer = buffer.substring(headerEnd + 4);
                    continue;
                }

                contentLength = parseInt(match[1]);
                buffer = buffer.substring(headerEnd + 4);
            }

            if (buffer.length < contentLength) break;

            const content = buffer.substring(0, contentLength);
            buffer = buffer.substring(contentLength);
            contentLength = -1;

            try {
                const message = JSON.parse(content);
                handleRequest(message);
            } catch (e) {
                log(`Parse error: ${e.message}`);
            }
        }
    });

    process.stdin.on('end', () => {
        process.exit(0);
    });
}

// Main
log('Argon Language Server starting...');
readMessages();
