const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');
const fs = require('fs');
const catalog = require('./catalog');

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
 * Formats a Scratch block catalog entry into a rich Markdown hover tooltip.
 */
function formatFunctionHover(fn) {
    const md = new vscode.MarkdownString();
    md.isTrusted = true;
    md.appendMarkdown(`### \`${fn.syntax} -> ${fn.returnType}\`\n\n`);
    md.appendMarkdown(
        `**Category**: \`${fn.category.toUpperCase()}\` &nbsp;|&nbsp; **Shape**: \`${fn.shape}\` &nbsp;|&nbsp; **Opcode**: \`${fn.opcode}\`\n\n`
    );
    md.appendMarkdown(`${fn.description}\n\n`);

    if (fn.parameters && fn.parameters.length > 0) {
        md.appendMarkdown(`#### Parameters\n`);
        for (const p of fn.parameters) {
            const req = p.required ? '*(required)*' : `*(optional, default: \`${p.default}\`)*`;
            md.appendMarkdown(`- \`${p.name}\` (\`${p.type}\`) ${req} &mdash; ${p.description}\n`);
        }
        md.appendMarkdown('\n');
    }

    if (fn.example) {
        md.appendMarkdown(`#### Example\n\`\`\`sch\n${fn.example}\n\`\`\`\n\n`);
    }

    if (fn.notes) {
        md.appendMarkdown(`> 💡 **Note**: ${fn.notes}\n`);
    }

    return md;
}

/**
 * Formats a language keyword into a Markdown hover tooltip.
 */
function formatKeywordHover(kw) {
    const md = new vscode.MarkdownString();
    md.isTrusted = true;
    md.appendMarkdown(`### \`${kw.syntax}\`\n\n`);
    md.appendMarkdown(`**Category**: \`${kw.category.toUpperCase()}\` &nbsp;|&nbsp; **Kind**: Keyword\n\n`);
    md.appendMarkdown(`${kw.description}\n\n`);
    if (kw.example) {
        md.appendMarkdown(`#### Example\n\`\`\`sch\n${kw.example}\n\`\`\`\n`);
    }
    return md;
}

/**
 * Formats an event handler trigger into a Markdown hover tooltip.
 */
function formatEventHover(ev) {
    const md = new vscode.MarkdownString();
    md.isTrusted = true;
    md.appendMarkdown(`### \`${ev.label}\`\n\n`);
    md.appendMarkdown(`**Type**: \`${ev.detail}\`\n\n`);
    md.appendMarkdown(`${ev.description}\n\n`);
    if (ev.example) {
        md.appendMarkdown(`#### Example\n\`\`\`sch\n${ev.example}\n\`\`\`\n`);
    }
    return md;
}

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

    // Check default cargo bin directory
    const homeDir = process.env.USERPROFILE || process.env.HOME || '';
    if (homeDir) {
        const cargoBin = path.join(homeDir, '.cargo', 'bin', process.platform === 'win32' ? 'scratch.exe' : 'scratch');
        if (fs.existsSync(cargoBin)) {
            return cargoBin;
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
 * Sends a request expecting a response with a safe 2-second timeout.
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

        // Safe 2s timeout prevents hover and UI from hanging on "Loading..."
        setTimeout(() => {
            if (pendingRequests.has(id)) {
                pendingRequests.delete(id);
                reject(new Error(`LSP Request ${method} timed out`));
            }
        }, 2000);
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

    // =========================================================================
    // 1. Instant Keystroke Completion Provider (Zero-Latency Fuzzy Typeahead)
    // =========================================================================
    // Registered WITHOUT restrictive trigger characters so it auto-triggers
    // on every single letter/character the user types (a-z, _, etc.)
    context.subscriptions.push(
        vscode.languages.registerCompletionItemProvider('scratch', {
            provideCompletionItems(document, position) {
                const lineText = document.lineAt(position.line).text;
                const prefix = lineText.slice(0, position.character);
                const trimmed = prefix.trim();

                const items = [];

                // 1. Inside string quotes context
                if (prefix.includes('("') || prefix.endsWith('"')) {
                    if (
                        prefix.includes('action.') ||
                        prefix.includes('press') ||
                        prefix.includes('down') ||
                        prefix.includes('up')
                    ) {
                        for (const act of catalog.INPUT_ACTIONS) {
                            const ci = new vscode.CompletionItem(
                                {
                                    label: act,
                                    detail: ' (key mapping)',
                                    description: 'input action',
                                },
                                vscode.CompletionItemKind.Value
                            );
                            ci.detail = `Input Action: ${act}`;
                            ci.documentation = new vscode.MarkdownString(`Physical keyboard key mapping for \`${act}\``);
                            ci.insertText = act;
                            items.push(ci);
                        }
                        return new vscode.CompletionList(items, false);
                    }
                }

                // 2. Event header suggestions (when line starts with 'when' or is empty)
                if (trimmed === '' || trimmed === 'when' || trimmed === 'when ') {
                    for (const ev of catalog.EVENTS_DATA) {
                        const ci = new vscode.CompletionItem(
                            {
                                label: ev.label,
                                detail: ` (${ev.name.replace('when ', '')})`,
                                description: ev.detail || 'event',
                            },
                            vscode.CompletionItemKind.Event
                        );
                        ci.detail = `${ev.label} - ${ev.detail}`;
                        ci.documentation = formatEventHover(ev);
                        ci.insertText = new vscode.SnippetString(ev.snippet);
                        ci.sortText = `0_${ev.name}`;
                        items.push(ci);
                    }
                }

                // 3. All 151 Scratch 3.0 Standard Blocks & Functions
                for (const fn of catalog.FUNCTIONS_DATA) {
                    let kind = vscode.CompletionItemKind.Function;
                    if (fn.shape === 'Reporter') {
                        kind = vscode.CompletionItemKind.Property;
                    } else if (fn.shape === 'Boolean') {
                        kind = vscode.CompletionItemKind.Operator;
                    } else if (fn.shape === 'Hat') {
                        kind = vscode.CompletionItemKind.Event;
                    }

                    const paramSummary =
                        fn.parameters && fn.parameters.length > 0
                            ? `(${fn.parameters.map((p) => (p.required ? p.name : `[${p.name}]`)).join(', ')})`
                            : '()';

                    const ci = new vscode.CompletionItem(
                        {
                            label: fn.name,
                            detail: ` ${paramSummary}`,
                            description: `${fn.category} • ${fn.shape}`,
                        },
                        kind
                    );

                    ci.detail = `${fn.syntax} : ${fn.returnType}`;
                    ci.documentation = formatFunctionHover(fn);
                    ci.insertText = new vscode.SnippetString(fn.lspSnippet);
                    ci.filterText = fn.name;
                    ci.sortText = `1_${fn.category}_${fn.name}`;
                    items.push(ci);
                }

                // 4. Control Flow and Syntax Keywords
                for (const kw of catalog.KEYWORDS_DATA) {
                    const argTail = kw.syntax.replace(kw.name, '').trim();
                    const ci = new vscode.CompletionItem(
                        {
                            label: kw.name,
                            detail: argTail ? ` ${argTail}` : '',
                            description: 'keyword',
                        },
                        vscode.CompletionItemKind.Keyword
                    );
                    ci.detail = `${kw.syntax} (Keyword)`;
                    ci.documentation = formatKeywordHover(kw);
                    ci.insertText = new vscode.SnippetString(kw.snippet);
                    ci.sortText = `2_${kw.name}`;
                    items.push(ci);
                }

                // 5. Built-in Game Entities
                for (const obj of catalog.KNOWN_OBJECTS) {
                    const ci = new vscode.CompletionItem(
                        {
                            label: obj,
                            detail: ' (Entity)',
                            description: 'game object',
                        },
                        vscode.CompletionItemKind.Class
                    );
                    ci.detail = `Game Object: ${obj}`;
                    ci.documentation = new vscode.MarkdownString(`Active scene entity \`${obj}\``);
                    ci.insertText = obj;
                    ci.sortText = `3_${obj}`;
                    items.push(ci);
                }

                // 6. User-defined local variables in the active document
                try {
                    const docText = document.getText();
                    const varMatches = docText.matchAll(/\b([a-zA-Z_][a-zA-Z0-9_]*)\s*=/g);
                    const seen = new Set(catalog.KNOWN_OBJECTS);
                    for (const m of varMatches) {
                        const varName = m[1];
                        if (
                            !seen.has(varName) &&
                            !catalog.FUNCTION_MAP.has(varName) &&
                            !catalog.KEYWORD_MAP.has(varName)
                        ) {
                            seen.add(varName);
                            const ci = new vscode.CompletionItem(
                                {
                                    label: varName,
                                    detail: ' (Variable)',
                                    description: 'local variable',
                                },
                                vscode.CompletionItemKind.Variable
                            );
                            ci.detail = 'User Variable';
                            ci.sortText = `4_${varName}`;
                            items.push(ci);
                        }
                    }
                } catch (_) {}

                // isIncomplete: false instructs VS Code to cache and run its native
                // fuzzy ranking engine at 60 FPS client-side on subsequent keystrokes
                return new vscode.CompletionList(items, false);
            },
        })
    );

    // =========================================================================
    // 2. Instant Hover Documentation Provider (Resolves "Loading..." Freeze)
    // =========================================================================
    // Immediately responds in 0ms from the in-memory catalog dictionary,
    // with a strict 500ms race timeout guard for custom LSP symbol queries.
    context.subscriptions.push(
        vscode.languages.registerHoverProvider('scratch', {
            async provideHover(document, position) {
                const idRange = document.getWordRangeAtPosition(position, /[a-zA-Z0-9_\.:]+/);
                const wordRange = document.getWordRangeAtPosition(position);
                const range = idRange || wordRange;
                if (!range) return null;

                const fullToken = document.getText(range);
                const baseWord = wordRange ? document.getText(wordRange) : fullToken;

                // 1. Instant check in Function Catalog
                if (catalog.FUNCTION_MAP.has(fullToken)) {
                    const fn = catalog.FUNCTION_MAP.get(fullToken);
                    return new vscode.Hover(formatFunctionHover(fn), range);
                }
                if (catalog.FUNCTION_MAP.has(baseWord)) {
                    const fn = catalog.FUNCTION_MAP.get(baseWord);
                    return new vscode.Hover(formatFunctionHover(fn), wordRange);
                }

                // 2. Instant check in Keywords Catalog
                if (catalog.KEYWORD_MAP.has(baseWord)) {
                    const kw = catalog.KEYWORD_MAP.get(baseWord);
                    return new vscode.Hover(formatKeywordHover(kw), wordRange);
                }

                // 3. Instant check in Event Triggers
                const eventMatch = catalog.EVENTS_DATA.find(
                    (e) => e.name === fullToken || e.name === baseWord || e.label.startsWith(fullToken)
                );
                if (eventMatch) {
                    return new vscode.Hover(formatEventHover(eventMatch), range);
                }

                // 4. Fall back to background LSP server for custom symbols with a strict 500ms timeout
                if (lspProcess && !lspProcess.killed) {
                    try {
                        const res = await Promise.race([
                            sendRequest('textDocument/hover', {
                                textDocument: { uri: document.uri.toString() },
                                position: { line: position.line, character: position.character },
                            }),
                            new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), 500)),
                        ]);
                        if (res && res.contents) {
                            const val = typeof res.contents === 'string' ? res.contents : res.contents.value || '';
                            if (val) {
                                const md = new vscode.MarkdownString(val);
                                md.isTrusted = true;
                                return new vscode.Hover(md, range);
                            }
                        }
                    } catch (_) {
                        return null;
                    }
                }

                return null;
            },
        })
    );

    // =========================================================================
    // 3. Document Formatting Provider (Shift + Alt + F)
    // =========================================================================
    context.subscriptions.push(
        vscode.languages.registerDocumentFormattingEditProvider('scratch', {
            async provideDocumentFormattingEdits(document) {
                const formatEnabled = vscode.workspace.getConfiguration('scratch').get('format.enable');
                if (!formatEnabled) return [];

                try {
                    const res = await Promise.race([
                        sendRequest('textDocument/formatting', {
                            textDocument: { uri: document.uri.toString() },
                            options: {
                                tabSize: 4,
                                insertSpaces: true,
                            },
                        }),
                        new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), 1500)),
                    ]);
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

    // =========================================================================
    // 4. Document Symbol Provider (Outline View)
    // =========================================================================
    context.subscriptions.push(
        vscode.languages.registerDocumentSymbolProvider('scratch', {
            async provideDocumentSymbols(document) {
                try {
                    const res = await Promise.race([
                        sendRequest('textDocument/documentSymbol', {
                            textDocument: { uri: document.uri.toString() },
                        }),
                        new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), 1500)),
                    ]);
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

    // =========================================================================
    // 5. Commands Registration
    // =========================================================================
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
        }),
        vscode.commands.registerCommand('scratch.searchFunctions', async () => {
            const editor = vscode.window.activeTextEditor;
            const items = catalog.FUNCTIONS_DATA.map((fn) => ({
                label: `$(symbol-function) ${fn.name}`,
                description: `(${fn.parameters.map((p) => p.name).join(', ')})`,
                detail: `[${fn.category.toUpperCase()}] ${fn.shape} • ${fn.description}`,
                fn: fn,
            }));

            const selected = await vscode.window.showQuickPick(items, {
                placeHolder: 'Search all 151 Scratch blocks and functions by name, category, or description...',
                matchOnDescription: true,
                matchOnDetail: true,
            });

            if (selected && editor) {
                editor.insertSnippet(new vscode.SnippetString(selected.fn.lspSnippet));
            }
        })
    );
}

function deactivate() {
    stopLspServer();
}

module.exports = {
    activate,
    deactivate,
    formatFunctionHover,
    formatKeywordHover,
    formatEventHover,
};
