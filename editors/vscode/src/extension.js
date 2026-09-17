const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');
const fs = require('fs');

/** @type {cp.ChildProcessWithoutNullStreams | null} */
let lspProcess = null;
/** @type {vscode.DiagnosticCollection} */
let diagnosticCollection;
/** @type {vscode.OutputChannel} */
let outputChannel;

let messageId = 1;
const pendingRequests = new Map();
let incomingBuffer = Buffer.alloc(0);

/**
 * Finds the scratch CLI binary path from settings, workspace targets, or PATH.
 */
function resolveScratchPath() {
    const configPath = vscode.workspace.getConfiguration('scratch').get('lsp.path');
    if (configPath && configPath !== 'scratch') {
        if (fs.existsSync(configPath)) {
            return configPath;
        }
    }

    // Check workspace local target builds
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders && workspaceFolders.length > 0) {
        const root = workspaceFolders[0].uri.fsPath;
        const candidates = [
            path.join(root, 'target', 'debug', process.platform === 'win32' ? 'scratch.exe' : 'scratch'),
            path.join(root, 'target', 'release', process.platform === 'win32' ? 'scratch.exe' : 'scratch'),
            path.join(root, '..', 'target', 'debug', process.platform === 'win32' ? 'scratch.exe' : 'scratch'),
            path.join(root, '..', 'target', 'release', process.platform === 'win32' ? 'scratch.exe' : 'scratch'),
        ];
        for (const candidate of candidates) {
            if (fs.existsSync(candidate)) {
                return candidate;
            }
        }
    }

    return process.platform === 'win32' ? 'scratch.exe' : 'scratch';
}

/**
 * Writes an LSP JSON-RPC framed message to the server stdio.
 */
function sendJsonRpc(obj) {
    if (!lspProcess || lspProcess.killed) return;
    const jsonStr = JSON.stringify(obj);
    const byteLen = Buffer.byteLength(jsonStr, 'utf8');
    const header = `Content-Length: ${byteLen}\r\n\r\n`;
    lspProcess.stdin.write(header + jsonStr, 'utf8');
}

/**
 * Sends a request expecting a response.
 */
function sendRequest(method, params) {
    return new Promise((resolve, reject) => {
        if (!lspProcess || lspProcess.killed) {
            return reject(new Error('LSP server is not running'));
        }
        const id = messageId++;
        pendingRequests.set(id, { resolve, reject, method });
        sendJsonRpc({
            jsonrpc: '2.0',
            id,
            method,
            params,
        });

        // 10s timeout
        setTimeout(() => {
            if (pendingRequests.has(id)) {
                pendingRequests.delete(id);
                reject(new Error(`LSP Request ${method} timed out`));
            }
        }, 10000);
    });
}

/**
 * Sends a notification with no response expected.
 */
function sendNotification(method, params) {
    sendJsonRpc({
        jsonrpc: '2.0',
        method,
        params,
    });
}

/**
 * Processes incoming data from LSP stdout.
 */
function handleIncomingData(chunk) {
    incomingBuffer = Buffer.concat([incomingBuffer, chunk]);

    while (true) {
        const headerEnd = incomingBuffer.indexOf('\r\n\r\n');
        if (headerEnd === -1) break;

        const headerStr = incomingBuffer.slice(0, headerEnd).toString('utf8');
        const match = headerStr.match(/Content-Length:\s*(\d+)/i);
        if (!match) {
            outputChannel.appendLine('[LSP Error] Missing Content-Length header');
            incomingBuffer = incomingBuffer.slice(headerEnd + 4);
            continue;
        }

        const contentLength = parseInt(match[1], 10);
        const totalMsgLen = headerEnd + 4 + contentLength;
        if (incomingBuffer.length < totalMsgLen) {
            // Incomplete body, wait for next chunk
            break;
        }

        const bodyBuf = incomingBuffer.slice(headerEnd + 4, totalMsgLen);
        incomingBuffer = incomingBuffer.slice(totalMsgLen);

        try {
            const msg = JSON.parse(bodyBuf.toString('utf8'));
            handleLspMessage(msg);
        } catch (err) {
            outputChannel.appendLine('[LSP Parse Error] ' + err.message);
        }
    }
}

/**
 * Dispatches an incoming LSP message.
 */
function handleLspMessage(msg) {
    if (msg.id !== undefined && pendingRequests.has(msg.id)) {
        const { resolve, reject } = pendingRequests.get(msg.id);
        pendingRequests.delete(msg.id);
        if (msg.error) {
            reject(new Error(msg.error.message || 'LSP Request error'));
        } else {
            resolve(msg.result);
        }
        return;
    }

    // Server notifications
    if (msg.method === 'textDocument/publishDiagnostics') {
        const params = msg.params;
        if (params && params.uri) {
            const uri = vscode.Uri.parse(params.uri);
            const diagnostics = (params.diagnostics || []).map((d) => {
                const range = new vscode.Range(
                    d.range.start.line,
                    d.range.start.character,
                    d.range.end.line,
                    d.range.end.character
                );
                const severity =
                    d.severity === 1
                        ? vscode.DiagnosticSeverity.Error
                        : d.severity === 2
                        ? vscode.DiagnosticSeverity.Warning
                        : vscode.DiagnosticSeverity.Information;

                const diag = new vscode.Diagnostic(range, d.message, severity);
                diag.source = d.source || 'scratch-lang';
                if (d.code) diag.code = d.code;
                return diag;
            });
            diagnosticCollection.set(uri, diagnostics);
        }
    }
}

/**
 * Starts or restarts the LSP server daemon.
 */
function startLspServer() {
    stopLspServer();

    const enabled = vscode.workspace.getConfiguration('scratch').get('lsp.enable');
    if (!enabled) {
        outputChannel.appendLine('[LSP] Disabled in user configuration.');
        return;
    }

    const binPath = resolveScratchPath();
    outputChannel.appendLine(`[LSP] Launching Language Server: ${binPath} lsp --stdio`);

    try {
        lspProcess = cp.spawn(binPath, ['lsp', '--stdio'], {
            stdio: ['pipe', 'pipe', 'pipe'],
            windowsHide: true,
        });

        lspProcess.stdout.on('data', handleIncomingData);
        lspProcess.stderr.on('data', (d) => {
            outputChannel.appendLine(`[LSP Server Log] ${d.toString('utf8').trim()}`);
        });

        lspProcess.on('error', (err) => {
            outputChannel.appendLine(`[LSP Error] Failed to start scratch process: ${err.message}`);
            vscode.window.showWarningMessage(
                `scratch-lang: Could not start 'scratch lsp' (${err.message}). Make sure 'scratch' is in your PATH or configure 'scratch.lsp.path'.`
            );
        });

        lspProcess.on('exit', (code) => {
            outputChannel.appendLine(`[LSP] Process exited with code ${code}`);
        });

        // Initialize handshake
        sendRequest('initialize', {
            processId: process.pid,
            rootUri: vscode.workspace.workspaceFolders?.[0]?.uri.toString() || null,
            capabilities: {
                textDocument: {
                    hover: { contentFormat: ['markdown', 'plaintext'] },
                    completion: { snippetSupport: true },
                    formatting: { dynamicRegistration: false },
                    documentSymbol: { hierarchicalDocumentSymbolSupport: true },
                },
            },
        }).then(() => {
            sendNotification('initialized', {});
            outputChannel.appendLine('[LSP] Initialized handshake successful.');

            // Sync currently active scratch files
            vscode.workspace.textDocuments.forEach((doc) => {
                if (doc.languageId === 'scratch') {
                    notifyDidOpen(doc);
                }
            });
        }).catch((err) => {
            outputChannel.appendLine(`[LSP Init Error] ${err.message}`);
        });
    } catch (e) {
        outputChannel.appendLine(`[LSP Spawn Error] ${e.message}`);
    }
}

function stopLspServer() {
    if (lspProcess) {
        try {
            lspProcess.kill();
        } catch (_) {}
        lspProcess = null;
    }
    incomingBuffer = Buffer.alloc(0);
    pendingRequests.clear();
}

function notifyDidOpen(document) {
    if (!lspProcess || document.languageId !== 'scratch') return;
    sendNotification('textDocument/didOpen', {
        textDocument: {
            uri: document.uri.toString(),
            languageId: 'scratch',
            version: document.version,
            text: document.getText(),
        },
    });
}

function notifyDidChange(document) {
    if (!lspProcess || document.languageId !== 'scratch') return;
    sendNotification('textDocument/didChange', {
        textDocument: {
            uri: document.uri.toString(),
            version: document.version,
        },
        contentChanges: [
            {
                text: document.getText(),
            },
        ],
    });
}

function notifyDidClose(document) {
    if (!lspProcess || document.languageId !== 'scratch') return;
    sendNotification('textDocument/didClose', {
        textDocument: {
            uri: document.uri.toString(),
        },
    });
    diagnosticCollection.delete(document.uri);
}

/**
 * Extension activation entry point.
 */
function activate(context) {
    outputChannel = vscode.window.createOutputChannel('scratch-lang');
    diagnosticCollection = vscode.languages.createDiagnosticCollection('scratch');
    context.subscriptions.push(outputChannel, diagnosticCollection);

    outputChannel.appendLine('scratch-lang extension activated.');

    startLspServer();

    // Debounced didChange for responsive real-time linting without server saturation
    let changeTimer = null;
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(notifyDidOpen),
        vscode.workspace.onDidChangeTextDocument((e) => {
            if (e.document.languageId === 'scratch') {
                if (changeTimer) clearTimeout(changeTimer);
                changeTimer = setTimeout(() => notifyDidChange(e.document), 100);
            }
        }),
        vscode.workspace.onDidCloseTextDocument(notifyDidClose)
    );

    // 1. Completion Provider
    context.subscriptions.push(
        vscode.languages.registerCompletionItemProvider(
            'scratch',
            {
                async provideCompletionItems(document, position) {
                    try {
                        const res = await sendRequest('textDocument/completion', {
                            textDocument: { uri: document.uri.toString() },
                            position: { line: position.line, character: position.character },
                        });
                        if (!res) return [];

                        const items = Array.isArray(res) ? res : res.items || [];
                        return items.map((item) => {
                            const ci = new vscode.CompletionItem(item.label);
                            if (item.kind) {
                                // Map LSP CompletionItemKind to VS Code CompletionItemKind
                                ci.kind = item.kind - 1;
                            }
                            ci.detail = item.detail;
                            if (item.documentation) {
                                const docText =
                                    typeof item.documentation === 'string'
                                        ? item.documentation
                                        : item.documentation.value;
                                ci.documentation = new vscode.MarkdownString(docText);
                            }
                            if (item.insertText) {
                                if (item.insertTextFormat === 2) {
                                    // Snippet format
                                    ci.insertText = new vscode.SnippetString(item.insertText);
                                } else {
                                    ci.insertText = item.insertText;
                                }
                            }
                            return ci;
                        });
                    } catch (e) {
                        return [];
                    }
                },
            },
            '.', '(', '"', ' '
        )
    );

    // 2. Hover Provider
    context.subscriptions.push(
        vscode.languages.registerHoverProvider('scratch', {
            async provideHover(document, position) {
                try {
                    const res = await sendRequest('textDocument/hover', {
                        textDocument: { uri: document.uri.toString() },
                        position: { line: position.line, character: position.character },
                    });
                    if (!res || !res.contents) return null;

                    const contents = res.contents;
                    const markdownText = typeof contents === 'string' ? contents : contents.value || '';
                    return new vscode.Hover(new vscode.MarkdownString(markdownText));
                } catch (e) {
                    return null;
                }
            },
        })
    );

    // 3. Document Formatting Provider
    context.subscriptions.push(
        vscode.languages.registerDocumentFormattingEditProvider('scratch', {
            async provideDocumentFormattingEdits(document) {
                const formatEnabled = vscode.workspace.getConfiguration('scratch').get('format.enable');
                if (!formatEnabled) return [];

                try {
                    const res = await sendRequest('textDocument/formatting', {
                        textDocument: { uri: document.uri.toString() },
                        options: {
                            tabSize: 4,
                            insertSpaces: true,
                        },
                    });
                    if (!res || !Array.isArray(res)) return [];

                    return res.map((edit) => {
                        const range = new vscode.Range(
                            edit.range.start.line,
                            edit.range.start.character,
                            edit.range.end.line,
                            edit.range.end.character
                        );
                        return new vscode.TextEdit(range, edit.newText);
                    });
                } catch (e) {
                    return [];
                }
            },
        })
    );

    // 4. Document Symbol Provider (Outline View)
    context.subscriptions.push(
        vscode.languages.registerDocumentSymbolProvider('scratch', {
            async provideDocumentSymbols(document) {
                try {
                    const res = await sendRequest('textDocument/documentSymbol', {
                        textDocument: { uri: document.uri.toString() },
                    });
                    if (!res || !Array.isArray(res)) return [];

                    return res.map((sym) => {
                        const range = new vscode.Range(
                            sym.range.start.line,
                            sym.range.start.character,
                            sym.range.end.line,
                            sym.range.end.character
                        );
                        const selectionRange = new vscode.Range(
                            sym.selectionRange.start.line,
                            sym.selectionRange.start.character,
                            sym.selectionRange.end.line,
                            sym.selectionRange.end.character
                        );
                        const kind = sym.kind ? sym.kind - 1 : vscode.SymbolKind.Event;
                        const docSym = new vscode.DocumentSymbol(
                            sym.name,
                            sym.detail || '',
                            kind,
                            range,
                            selectionRange
                        );
                        return docSym;
                    });
                } catch (e) {
                    return [];
                }
            },
        })
    );

    // 5. Commands Registration
    const runTerminalCommand = (cmdTitle, cmdStr) => {
        let terminal = vscode.window.terminals.find((t) => t.name === 'scratch');
        if (!terminal) {
            terminal = vscode.window.createTerminal('scratch');
        }
        terminal.show();
        terminal.sendText(cmdStr);
    };

    context.subscriptions.push(
        vscode.commands.registerCommand('scratch.run', () => {
            const bin = resolveScratchPath();
            runTerminalCommand('Scratch Run', `"${bin}" run`);
        }),
        vscode.commands.registerCommand('scratch.check', () => {
            const bin = resolveScratchPath();
            runTerminalCommand('Scratch Check', `"${bin}" check`);
        }),
        vscode.commands.registerCommand('scratch.studio', () => {
            const bin = resolveScratchPath();
            runTerminalCommand('Scratch Studio', `"${bin}" project studio`);
        }),
        vscode.commands.registerCommand('scratch.restartServer', () => {
            outputChannel.appendLine('[LSP] Restart command triggered by user.');
            startLspServer();
            vscode.window.showInformationMessage('scratch-lang: Language Server restarted.');
        })
    );
}

function deactivate() {
    stopLspServer();
}

module.exports = {
    activate,
    deactivate,
};
