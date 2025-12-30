const vscode = require('vscode');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');
const path = require('path');

let client;

function activate(context) {
    console.log('Argon Language extension activated');

    // Path to the language server
    // Path to the language server
    const serverModule = context.asAbsolutePath(
        path.join('argon-lsp.js')
    );

    const serverOptions = {
        run: { module: serverModule, transport: TransportKind.stdio },
        debug: { module: serverModule, transport: TransportKind.stdio }
    };

    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'argon' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.ar')
        }
    };

    client = new LanguageClient(
        'argonLanguageServer',
        'Argon Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client
    client.start();
}

function deactivate() {
    if (client) {
        return client.stop();
    }
}

module.exports = { activate, deactivate };
