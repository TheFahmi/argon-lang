#!/usr/bin/env node
// ============================================
// CRYO LANGUAGE SERVER v2.0.0
// Language Server Protocol implementation
// With Navigation and Autocomplete support
// ============================================

const fs = require('fs');
const path = require('path');

// State
let documents = new Map();
let workspaceRoot = '';
let initialized = false;

// Symbol cache for cross-file lookups
let symbolCache = new Map(); // uri -> symbols[]

// Logging
function log(msg) {
    console.error(`[cryo-lsp] ${msg}`);
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

// ============================================
// PARSING & SYMBOLS
// ============================================

// Parse Cryo source for symbols with detailed info
function parseSymbols(text, uri) {
    const symbols = [];
    const lines = text.split('\n');

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmed = line.trim();

        // Function: fn name(params...)
        const fnMatch = trimmed.match(/^fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)/);
        if (fnMatch) {
            symbols.push({
                name: fnMatch[1],
                kind: 12, // Function
                params: fnMatch[2],
                line: i,
                uri: uri,
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
            // Parse fields
            let fields = [];
            let j = i + 1;
            while (j < lines.length && !lines[j].includes('}')) {
                const fieldMatch = lines[j].trim().match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)/);
                if (fieldMatch) {
                    fields.push({ name: fieldMatch[1], type: fieldMatch[2] });
                }
                j++;
            }

            symbols.push({
                name: structMatch[1],
                kind: 23, // Struct
                fields: fields,
                line: i,
                uri: uri,
                range: {
                    start: { line: i, character: 0 },
                    end: { line: j, character: lines[j] ? lines[j].length : 0 }
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
            // Parse variants
            let variants = [];
            let j = i + 1;
            while (j < lines.length && !lines[j].includes('}')) {
                const variantMatch = lines[j].trim().match(/^([a-zA-Z_][a-zA-Z0-9_]*)/);
                if (variantMatch && variantMatch[1]) {
                    variants.push(variantMatch[1]);
                }
                j++;
            }

            symbols.push({
                name: enumMatch[1],
                kind: 10, // Enum
                variants: variants,
                line: i,
                uri: uri,
                range: {
                    start: { line: i, character: 0 },
                    end: { line: j, character: lines[j] ? lines[j].length : 0 }
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
                line: i,
                uri: uri,
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
                line: i,
                uri: uri,
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

// Get word at position
function getWordAt(text, line, character) {
    const lines = text.split('\n');
    if (line >= lines.length) return null;

    const lineText = lines[line];
    let start = character;
    let end = character;

    while (start > 0 && /[a-zA-Z0-9_]/.test(lineText[start - 1])) start--;
    while (end < lineText.length && /[a-zA-Z0-9_]/.test(lineText[end])) end++;

    return {
        word: lineText.substring(start, end),
        start: start,
        end: end
    };
}

// ============================================
// PHASE 2: NAVIGATION
// ============================================

// Go to definition
function getDefinition(text, line, character, uri) {
    const wordInfo = getWordAt(text, line, character);
    if (!wordInfo || !wordInfo.word) return null;

    const word = wordInfo.word;
    const lines = text.split('\n');

    // Search in current document
    const fnRegex = new RegExp(`^\\s*fn\\s+${word}\\s*\\(`);
    const structRegex = new RegExp(`^\\s*struct\\s+${word}\\s*\\{`);
    const enumRegex = new RegExp(`^\\s*enum\\s+${word}\\s*\\{`);
    const letRegex = new RegExp(`^\\s*let\\s+${word}\\s*=`);

    for (let i = 0; i < lines.length; i++) {
        if (fnRegex.test(lines[i]) || structRegex.test(lines[i]) ||
            enumRegex.test(lines[i]) || letRegex.test(lines[i])) {
            return {
                uri: uri,
                range: {
                    start: { line: i, character: 0 },
                    end: { line: i, character: lines[i].length }
                }
            };
        }
    }

    // Search in imported modules
    const imports = [];
    for (let i = 0; i < lines.length; i++) {
        const importMatch = lines[i].match(/^import\s+"([^"]+)"/);
        if (importMatch) {
            imports.push(importMatch[1]);
        }
    }

    // Check symbol cache for imports
    for (const [cachedUri, symbols] of symbolCache) {
        for (const sym of symbols) {
            if (sym.name === word) {
                return {
                    uri: cachedUri,
                    range: sym.range
                };
            }
        }
    }

    return null;
}

// Find all references
function findReferences(text, line, character, uri, includeDeclaration) {
    const wordInfo = getWordAt(text, line, character);
    if (!wordInfo || !wordInfo.word) return [];

    const word = wordInfo.word;
    const lines = text.split('\n');
    const references = [];

    const wordRegex = new RegExp(`\\b${word}\\b`, 'g');

    for (let i = 0; i < lines.length; i++) {
        let match;
        while ((match = wordRegex.exec(lines[i])) !== null) {
            references.push({
                uri: uri,
                range: {
                    start: { line: i, character: match.index },
                    end: { line: i, character: match.index + word.length }
                }
            });
        }
    }

    return references;
}

// Document links (for imports)
function getDocumentLinks(text, uri) {
    const links = [];
    const lines = text.split('\n');

    for (let i = 0; i < lines.length; i++) {
        const importMatch = lines[i].match(/import\s+"([^"]+)"/);
        if (importMatch) {
            const importPath = importMatch[1];
            const start = lines[i].indexOf('"') + 1;
            const end = lines[i].lastIndexOf('"');

            // Resolve relative path
            let targetUri = uri;
            if (uri.startsWith('file://')) {
                const dir = path.dirname(uri.replace('file://', ''));
                const resolved = path.resolve(dir, importPath);
                targetUri = 'file://' + resolved;
            }

            links.push({
                range: {
                    start: { line: i, character: start },
                    end: { line: i, character: end }
                },
                target: targetUri
            });
        }
    }

    return links;
}

// ============================================
// PHASE 3: AUTOCOMPLETE
// ============================================

// Built-in functions and keywords
const builtins = {
    'print': { label: 'print', kind: 3, detail: 'fn print(value: any)', insertText: 'print(${1:value})' },
    'len': { label: 'len', kind: 3, detail: 'fn len(value: any) -> int', insertText: 'len(${1:value})' },
    'push': { label: 'push', kind: 3, detail: 'fn push(arr, item) -> array', insertText: 'push(${1:arr}, ${2:item})' },
    'parseInt': { label: 'parseInt', kind: 3, detail: 'fn parseInt(s: string) -> int', insertText: 'parseInt(${1:s})' },
    'char_code_at': { label: 'char_code_at', kind: 3, detail: 'fn char_code_at(s, i) -> int', insertText: 'char_code_at(${1:s}, ${2:i})' },
    'char_from_code': { label: 'char_from_code', kind: 3, detail: 'fn char_from_code(code: int) -> string', insertText: 'char_from_code(${1:code})' },
    'substring': { label: 'substring', kind: 3, detail: 'fn substring(s, start, end) -> string', insertText: 'substring(${1:s}, ${2:start}, ${3:end})' },
    'readFile': { label: 'readFile', kind: 3, detail: 'fn readFile(path: string) -> string', insertText: 'readFile(${1:path})' },
    'writeFile': { label: 'writeFile', kind: 3, detail: 'fn writeFile(path, content) -> int', insertText: 'writeFile(${1:path}, ${2:content})' },
    'sleep': { label: 'sleep', kind: 3, detail: 'fn sleep(ms: int)', insertText: 'sleep(${1:ms})' }
};

const keywords = [
    { label: 'fn', kind: 14, detail: 'Function definition', insertText: 'fn ${1:name}(${2:params}) {\n\t${3}\n}' },
    { label: 'let', kind: 14, detail: 'Variable declaration', insertText: 'let ${1:name} = ${2:value};' },
    { label: 'if', kind: 14, detail: 'Conditional', insertText: 'if (${1:condition}) {\n\t${2}\n}' },
    { label: 'else', kind: 14, detail: 'Else branch', insertText: 'else {\n\t${1}\n}' },
    { label: 'while', kind: 14, detail: 'While loop', insertText: 'while (${1:condition}) {\n\t${2}\n}' },
    { label: 'return', kind: 14, detail: 'Return statement', insertText: 'return ${1:value};' },
    { label: 'struct', kind: 14, detail: 'Struct definition', insertText: 'struct ${1:Name} {\n\t${2:field}: ${3:type}\n}' },
    { label: 'enum', kind: 14, detail: 'Enum definition', insertText: 'enum ${1:Name} {\n\t${2:Variant}(${3:type})\n}' },
    { label: 'match', kind: 14, detail: 'Pattern matching', insertText: 'match ${1:expr} {\n\t${2:Pattern} => {\n\t\t${3}\n\t}\n}' },
    { label: 'import', kind: 14, detail: 'Import module', insertText: 'import "${1:module}";' },
    { label: 'true', kind: 21, detail: 'Boolean true' },
    { label: 'false', kind: 21, detail: 'Boolean false' },
    { label: 'null', kind: 21, detail: 'Null value' }
];

// Get completions
function getCompletions(text, line, character, uri) {
    const items = [];
    const wordInfo = getWordAt(text, line, character);
    const prefix = wordInfo ? wordInfo.word : '';

    // Add keywords
    for (const kw of keywords) {
        if (!prefix || kw.label.startsWith(prefix)) {
            items.push({
                label: kw.label,
                kind: kw.kind,
                detail: kw.detail,
                insertText: kw.insertText,
                insertTextFormat: kw.insertText ? 2 : 1 // 2 = Snippet
            });
        }
    }

    // Add builtins
    for (const [name, fn] of Object.entries(builtins)) {
        if (!prefix || name.startsWith(prefix)) {
            items.push({
                label: fn.label,
                kind: fn.kind,
                detail: fn.detail,
                insertText: fn.insertText,
                insertTextFormat: 2
            });
        }
    }

    // Add symbols from current document
    const symbols = parseSymbols(text, uri);
    for (const sym of symbols) {
        if (!prefix || sym.name.startsWith(prefix)) {
            let insertText = sym.name;
            if (sym.kind === 12 && sym.params !== undefined) { // Function
                insertText = sym.params ? `${sym.name}(\${1})` : `${sym.name}()`;
            }

            items.push({
                label: sym.name,
                kind: sym.kind,
                detail: sym.params !== undefined ? `fn ${sym.name}(${sym.params})` : sym.name,
                insertText: insertText,
                insertTextFormat: sym.kind === 12 ? 2 : 1
            });
        }
    }

    return { isIncomplete: false, items: items };
}

// Signature help
function getSignatureHelp(text, line, character) {
    const lines = text.split('\n');
    if (line >= lines.length) return null;

    const lineText = lines[line];

    // Find the function being called
    let parenDepth = 0;
    let funcStart = -1;

    for (let i = character - 1; i >= 0; i--) {
        if (lineText[i] === ')') parenDepth++;
        if (lineText[i] === '(') {
            if (parenDepth === 0) {
                funcStart = i;
                break;
            }
            parenDepth--;
        }
    }

    if (funcStart <= 0) return null;

    // Get function name
    let nameEnd = funcStart;
    let nameStart = funcStart - 1;
    while (nameStart >= 0 && /[a-zA-Z0-9_]/.test(lineText[nameStart])) {
        nameStart--;
    }
    nameStart++;

    const funcName = lineText.substring(nameStart, nameEnd);
    if (!funcName) return null;

    // Built-in signatures
    const signatures = {
        'print': { label: 'print(value: any)', params: [{ label: 'value: any' }] },
        'len': { label: 'len(value: any) -> int', params: [{ label: 'value: any' }] },
        'push': { label: 'push(arr: array, item: any) -> array', params: [{ label: 'arr: array' }, { label: 'item: any' }] },
        'parseInt': { label: 'parseInt(s: string) -> int', params: [{ label: 's: string' }] },
        'substring': { label: 'substring(s: string, start: int, end: int) -> string', params: [{ label: 's: string' }, { label: 'start: int' }, { label: 'end: int' }] },
        'char_code_at': { label: 'char_code_at(s: string, i: int) -> int', params: [{ label: 's: string' }, { label: 'i: int' }] },
        'char_from_code': { label: 'char_from_code(code: int) -> string', params: [{ label: 'code: int' }] }
    };

    if (signatures[funcName]) {
        const sig = signatures[funcName];

        // Count commas to determine active parameter
        const argsText = lineText.substring(funcStart + 1, character);
        const activeParam = (argsText.match(/,/g) || []).length;

        return {
            signatures: [{
                label: sig.label,
                parameters: sig.params.map(p => ({ label: p.label }))
            }],
            activeSignature: 0,
            activeParameter: Math.min(activeParam, sig.params.length - 1)
        };
    }

    // Check document for function definition
    for (let i = 0; i < lines.length; i++) {
        const fnMatch = lines[i].match(new RegExp(`^\\s*fn\\s+${funcName}\\s*\\(([^)]*)\\)`));
        if (fnMatch) {
            const params = fnMatch[1].split(',').map(p => p.trim()).filter(p => p);
            const argsText = lineText.substring(funcStart + 1, character);
            const activeParam = (argsText.match(/,/g) || []).length;

            return {
                signatures: [{
                    label: `fn ${funcName}(${fnMatch[1]})`,
                    parameters: params.map(p => ({ label: p }))
                }],
                activeSignature: 0,
                activeParameter: Math.min(activeParam, params.length - 1)
            };
        }
    }

    return null;
}

// ============================================
// FORMATTING
// ============================================

function formatDocument(text) {
    const lines = text.split('\n');
    const result = [];
    let indentLevel = 0;
    const indentStr = '    '; // 4 spaces

    for (let line of lines) {
        let trimmed = line.trim();

        // Decrease indent for closing braces
        if (trimmed.startsWith('}')) {
            indentLevel = Math.max(0, indentLevel - 1);
        }

        // Apply indentation
        if (trimmed) {
            result.push(indentStr.repeat(indentLevel) + trimmed);
        } else {
            result.push('');
        }

        // Increase indent for opening braces
        if (trimmed.endsWith('{')) {
            indentLevel++;
        }
    }

    return result.join('\n');
}

// ============================================
// DIAGNOSTICS
// ============================================

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

        // Missing semicolon heuristic
        if (/^\s*(let|return)\s+.+[^;{]\s*$/.test(line) && !line.includes('{')) {
            if (i + 1 < lines.length) {
                const nextLine = lines[i + 1].trim();
                if (nextLine && !nextLine.startsWith('}') && !nextLine.startsWith('{')) {
                    diagnostics.push({
                        range: {
                            start: { line: i, character: line.length - 1 },
                            end: { line: i, character: line.length }
                        },
                        severity: 2,
                        message: 'Possible missing semicolon',
                        source: 'cryo'
                    });
                }
            }
        }

        // Empty print
        if (/print\(\s*\)/.test(line)) {
            const idx = line.indexOf('print()');
            diagnostics.push({
                range: {
                    start: { line: i, character: idx },
                    end: { line: i, character: idx + 7 }
                },
                severity: 2,
                message: 'print() called without arguments',
                source: 'cryo'
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
            severity: 1,
            message: braceDepth > 0 ? 'Unmatched opening brace' : 'Unmatched closing brace',
            source: 'cryo'
        });
    }

    return diagnostics;
}

// ============================================
// HOVER
// ============================================

function getHover(text, line, character) {
    const wordInfo = getWordAt(text, line, character);
    if (!wordInfo || !wordInfo.word) return null;

    const word = wordInfo.word;
    const lines = text.split('\n');

    // Built-in functions
    const builtinDocs = {
        'print': 'fn print(value: any) -> void\n\nPrint value to stdout',
        'len': 'fn len(value: any) -> int\n\nGet length of string or array',
        'push': 'fn push(array: any, item: any) -> array\n\nAppend item to array, return new array',
        'parseInt': 'fn parseInt(s: string) -> int\n\nParse string to integer',
        'char_code_at': 'fn char_code_at(s: string, i: int) -> int\n\nGet ASCII code of character at index',
        'char_from_code': 'fn char_from_code(code: int) -> string\n\nConvert ASCII code to single-character string',
        'substring': 'fn substring(s: string, start: int, end: int) -> string\n\nExtract substring',
        'sleep': 'fn sleep(ms: int) -> void\n\nSleep for milliseconds',
        'readFile': 'fn readFile(path: string) -> string\n\nRead file contents',
        'writeFile': 'fn writeFile(path: string, content: string) -> int\n\nWrite content to file'
    };

    if (builtinDocs[word]) {
        return {
            contents: {
                kind: 'markdown',
                value: '```cryo\n' + builtinDocs[word] + '\n```'
            }
        };
    }

    // Keywords
    const keywordDocs = {
        'fn': 'Define a function\n\n```cryo\nfn name(params) {\n    // body\n}\n```',
        'let': 'Declare a variable\n\n```cryo\nlet x = 10;\n```',
        'struct': 'Define a struct type\n\n```cryo\nstruct Point {\n    x: int,\n    y: int\n}\n```',
        'enum': 'Define an enum type\n\n```cryo\nenum Option {\n    Some(value),\n    None\n}\n```',
        'match': 'Pattern matching\n\n```cryo\nmatch expr {\n    Pattern => { body }\n}\n```',
        'import': 'Import a module\n\n```cryo\nimport "module.cryo";\n```'
    };

    if (keywordDocs[word]) {
        return {
            contents: {
                kind: 'markdown',
                value: keywordDocs[word]
            }
        };
    }

    // Find function definition
    const fnRegex = new RegExp(`^\\s*fn\\s+${word}\\s*\\(([^)]*)\\)`);
    for (let i = 0; i < lines.length; i++) {
        const match = lines[i].match(fnRegex);
        if (match) {
            return {
                contents: {
                    kind: 'markdown',
                    value: `\`\`\`cryo\nfn ${word}(${match[1]})\n\`\`\`\n*Defined on line ${i + 1}*`
                }
            };
        }
    }

    // Find struct definition
    const structRegex = new RegExp(`^\\s*struct\\s+${word}\\s*\\{`);
    for (let i = 0; i < lines.length; i++) {
        if (structRegex.test(lines[i])) {
            // Get fields
            let fields = [];
            let j = i + 1;
            while (j < lines.length && !lines[j].includes('}')) {
                const fieldMatch = lines[j].trim().match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)/);
                if (fieldMatch) {
                    fields.push(`    ${fieldMatch[1]}: ${fieldMatch[2]}`);
                }
                j++;
            }

            return {
                contents: {
                    kind: 'markdown',
                    value: `\`\`\`cryo\nstruct ${word} {\n${fields.join(',\n')}\n}\n\`\`\`\n*Defined on line ${i + 1}*`
                }
            };
        }
    }

    return null;
}

// ============================================
// REQUEST HANDLER
// ============================================

function handleRequest(message) {
    const { id, method, params } = message;

    switch (method) {
        case 'initialize':
            log('Initializing Cryo Language Server v2.0.0');
            initialized = true;
            if (params.rootUri) {
                workspaceRoot = params.rootUri;
            }
            sendResponse(id, {
                capabilities: {
                    textDocumentSync: 1,
                    hoverProvider: true,
                    documentSymbolProvider: true,
                    definitionProvider: true,
                    referencesProvider: true,
                    documentLinkProvider: { resolveProvider: false },
                    completionProvider: {
                        triggerCharacters: ['.', '"'],
                        resolveProvider: false
                    },
                    signatureHelpProvider: {
                        triggerCharacters: ['(', ',']
                    },
                    documentFormattingProvider: true
                },
                serverInfo: {
                    name: 'cryo-lsp',
                    version: '2.0.0'
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
                symbolCache.set(uri, parseSymbols(text, uri));
                log(`Opened: ${uri}`);

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
                symbolCache.set(uri, parseSymbols(text, uri));

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
                const symbols = parseSymbols(text, uri);
                sendResponse(id, symbols);
            }
            break;

        case 'textDocument/definition':
            {
                const { uri } = params.textDocument;
                const { line, character } = params.position;
                const text = documents.get(uri) || '';
                const definition = getDefinition(text, line, character, uri);
                sendResponse(id, definition);
            }
            break;

        case 'textDocument/references':
            {
                const { uri } = params.textDocument;
                const { line, character } = params.position;
                const text = documents.get(uri) || '';
                const includeDecl = params.context ? params.context.includeDeclaration : true;
                const references = findReferences(text, line, character, uri, includeDecl);
                sendResponse(id, references);
            }
            break;

        case 'textDocument/documentLink':
            {
                const { uri } = params.textDocument;
                const text = documents.get(uri) || '';
                const links = getDocumentLinks(text, uri);
                sendResponse(id, links);
            }
            break;

        case 'textDocument/completion':
            {
                const { uri } = params.textDocument;
                const { line, character } = params.position;
                const text = documents.get(uri) || '';
                const completions = getCompletions(text, line, character, uri);
                sendResponse(id, completions);
            }
            break;

        case 'textDocument/signatureHelp':
            {
                const { uri } = params.textDocument;
                const { line, character } = params.position;
                const text = documents.get(uri) || '';
                const help = getSignatureHelp(text, line, character);
                sendResponse(id, help);
            }
            break;

        case 'textDocument/formatting':
            {
                const { uri } = params.textDocument;
                const text = documents.get(uri) || '';
                const formatted = formatDocument(text);
                const lines = text.split('\n');

                sendResponse(id, [{
                    range: {
                        start: { line: 0, character: 0 },
                        end: { line: lines.length, character: 0 }
                    },
                    newText: formatted
                }]);
            }
            break;

        default:
            if (id !== undefined) {
                sendResponse(id, null);
            }
    }
}

// ============================================
// MESSAGE READER
// ============================================

function readMessages() {
    let buffer = '';
    let contentLength = -1;

    process.stdin.setEncoding('utf8');

    process.stdin.on('data', (chunk) => {
        buffer += chunk;

        while (true) {
            if (contentLength === -1) {
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
log('Cryo Language Server v2.0.0 starting...');
readMessages();
