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
/** @type {vscode.StatusBarItem} */
let lspStatusBarItem;
/** @type {vscode.StatusBarItem} */
let searchStatusBarItem;

let messageId = 1;
const pendingRequests = new Map();
let incomingBuffer = Buffer.alloc(0);
let restartAttempts = 0;
let restartTimer = null;
let uptimeTimer = null;

/**
 * Returns a VS Code Codicon for a block category.
 */
function getCategoryIcon(category) {
    switch ((category || '').toLowerCase()) {
        case 'motion':
            return '$(run)';
        case 'looks':
            return '$(eye)';
        case 'sound':
            return '$(unmute)';
        case 'events':
            return '$(zap)';
        case 'control':
            return '$(gear)';
        case 'sensing':
            return '$(radar)';
        case 'operators':
            return '$(symbol-operator)';
        case 'variables':
            return '$(variable)';
        case 'lists':
            return '$(list-unordered)';
        case 'pen':
            return '$(edit)';
        case 'music':
            return '$(music)';
        default:
            return '$(symbol-function)';
    }
}

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
 * Writes an LSP JSON-RPC framed message to the server stdio with crash guards.
 */
function sendJsonRpc(obj) {
    if (!lspProcess || lspProcess.killed || lspProcess.exitCode !== null) return;
    if (!lspProcess.stdin || !lspProcess.stdin.writable) return;

    try {
        const jsonStr = JSON.stringify(obj);
        const byteLen = Buffer.byteLength(jsonStr, 'utf8');
        const header = `Content-Length: ${byteLen}\r\n\r\n`;
        lspProcess.stdin.write(header + jsonStr, 'utf8');
    } catch (err) {
        if (outputChannel) {
            outputChannel.appendLine(`[LSP Send Error] ${err.message}`);
        }
    }
}

/**
 * Sends a request expecting a response with a safe 2-second timeout.
 */
function sendRequest(method, params) {
    return new Promise((resolve, reject) => {
        if (!lspProcess || lspProcess.killed || lspProcess.exitCode !== null) {
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
 * Updates status bar indicators.
 */
function updateStatusBar(status) {
    if (!lspStatusBarItem) return;
    if (status === 'ready') {
        lspStatusBarItem.text = '$(check) Scratch LSP: Ready';
        lspStatusBarItem.tooltip = 'Scratch Language Server is running. Click to restart.';
        lspStatusBarItem.backgroundColor = undefined;
    } else if (status === 'initializing') {
        lspStatusBarItem.text = '$(sync~spin) Scratch LSP: Initializing';
        lspStatusBarItem.tooltip = 'Scratch Language Server is starting up...';
        lspStatusBarItem.backgroundColor = undefined;
    } else if (status === 'stopped') {
        lspStatusBarItem.text = '$(warning) Scratch LSP: Offline';
        lspStatusBarItem.tooltip = 'Scratch Language Server stopped. Click to restart.';
        lspStatusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
    }
}

/**
 * Starts or restarts the LSP server daemon with crash resilience and auto-restart.
 */
function startLspServer() {
    stopLspServer();

    const enabled = vscode.workspace.getConfiguration('scratch').get('lsp.enable');
    if (!enabled) {
        outputChannel.appendLine('[LSP] Disabled in user configuration.');
        updateStatusBar('stopped');
        return;
    }

    const binPath = resolveScratchPath();
    outputChannel.appendLine(`[LSP] Launching Language Server: ${binPath} lsp --stdio`);
    updateStatusBar('initializing');

    try {
        lspProcess = cp.spawn(binPath, ['lsp', '--stdio'], {
            stdio: ['pipe', 'pipe', 'pipe'],
            windowsHide: true,
        });

        // Crash-guard: prevent unhandled EPIPE on stdin
        lspProcess.stdin.on('error', (err) => {
            outputChannel.appendLine(`[LSP Stdin Error] ${err.message}`);
        });

        lspProcess.stdout.on('data', handleIncomingData);
        lspProcess.stderr.on('data', (d) => {
            outputChannel.appendLine(`[LSP Server Log] ${d.toString('utf8').trim()}`);
        });

        lspProcess.on('error', (err) => {
            outputChannel.appendLine(`[LSP Error] Failed to spawn scratch binary: ${err.message}`);
            updateStatusBar('stopped');
        });

        lspProcess.on('exit', (code, signal) => {
            outputChannel.appendLine(`[LSP] Process exited with code ${code}, signal ${signal}`);
            lspProcess = null;
            updateStatusBar('stopped');

            // Reject all pending requests
            for (const [, req] of pendingRequests.entries()) {
                req.reject(new Error('LSP server exited'));
            }
            pendingRequests.clear();

            // Auto-restart with backoff (up to 3 consecutive attempts)
            if (restartAttempts < 3) {
                restartAttempts++;
                const delay = restartAttempts * 1500;
                outputChannel.appendLine(`[LSP] Scheduling auto-restart attempt ${restartAttempts}/3 in ${delay}ms...`);
                restartTimer = setTimeout(() => {
                    startLspServer();
                }, delay);
            }
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
        })
            .then(() => {
                sendNotification('initialized', {});
                outputChannel.appendLine('[LSP] Initialized handshake successful.');
                updateStatusBar('ready');

                // If server stays alive for 30 seconds, reset restart attempt counter
                if (uptimeTimer) clearTimeout(uptimeTimer);
                uptimeTimer = setTimeout(() => {
                    restartAttempts = 0;
                }, 30000);

                // Sync currently active scratch files
                vscode.workspace.textDocuments.forEach((doc) => {
                    if (doc.languageId === 'scratch') {
                        notifyDidOpen(doc);
                    }
                });
            })
            .catch((err) => {
                outputChannel.appendLine(`[LSP Init Error] ${err.message}`);
                updateStatusBar('stopped');
            });
    } catch (e) {
        outputChannel.appendLine(`[LSP Spawn Error] ${e.message}`);
        updateStatusBar('stopped');
    }
}

function stopLspServer() {
    if (restartTimer) {
        clearTimeout(restartTimer);
        restartTimer = null;
    }
    if (uptimeTimer) {
        clearTimeout(uptimeTimer);
        uptimeTimer = null;
    }
    if (lspProcess) {
        try {
            lspProcess.kill();
        } catch (_) {}
        lspProcess = null;
    }
    incomingBuffer = Buffer.alloc(0);
    pendingRequests.clear();
    updateStatusBar('stopped');
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

    // Status bar item for Quick Search
    searchStatusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 101);
    searchStatusBarItem.text = '$(search) Scratch Blocks';
    searchStatusBarItem.tooltip = 'Search all 151 Scratch blocks and functions (Ctrl+Alt+S)';
    searchStatusBarItem.command = 'scratch.searchFunctions';
    searchStatusBarItem.show();
    context.subscriptions.push(searchStatusBarItem);

    // Status bar item for LSP status
    lspStatusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    lspStatusBarItem.command = 'scratch.restartServer';
    lspStatusBarItem.show();
    context.subscriptions.push(lspStatusBarItem);

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

                // Accurate check if cursor is currently inside a string literal quote
                const quoteCount = (prefix.match(/"/g) || []).length;
                const isInsideQuotes = quoteCount % 2 === 1;

                const items = [];

                // 1. String argument context completions
                if (isInsideQuotes) {
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
                            ci.detail = `Input Action: "${act}"`;
                            ci.documentation = new vscode.MarkdownString(
                                `Physical keyboard key mapping for \`${act}\``
                            );
                            ci.insertText = act;
                            items.push(ci);
                        }
                    }

                    if (prefix.includes('scene.') || prefix.includes('scene_switch')) {
                        for (const sc of ['main', 'level1', 'level2', 'gameover', 'victory']) {
                            const ci = new vscode.CompletionItem(
                                {
                                    label: sc,
                                    detail: ' (scene name)',
                                    description: 'scene',
                                },
                                vscode.CompletionItemKind.File
                            );
                            ci.detail = `Scene Target: "${sc}"`;
                            ci.insertText = sc;
                            items.push(ci);
                        }
                    }

                    if (prefix.includes('sound') || prefix.includes('play')) {
                        for (const snd of ['jump', 'coin', 'hit', 'laser', 'win', 'lose', 'step', 'powerup']) {
                            const ci = new vscode.CompletionItem(
                                {
                                    label: snd,
                                    detail: ' (sound effect)',
                                    description: 'sound asset',
                                },
                                vscode.CompletionItemKind.File
                            );
                            ci.detail = `Audio Asset: "${snd}"`;
                            ci.insertText = snd;
                            items.push(ci);
                        }
                    }

                    // Also allow general game objects inside quotes (e.g. move("Player", 10))
                    for (const obj of catalog.KNOWN_OBJECTS) {
                        const ci = new vscode.CompletionItem(
                            {
                                label: obj,
                                detail: ' (Game Entity)',
                                description: 'sprite target',
                            },
                            vscode.CompletionItemKind.Class
                        );
                        ci.detail = `Target Sprite: "${obj}"`;
                        ci.insertText = obj;
                        items.push(ci);
                    }

                    return new vscode.CompletionList(items, false);
                }

                // 2. Event header suggestions (always available for easy searching)
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
                    ci.filterText = `${ev.label} ${ev.name}`;
                    ci.sortText = `0_${ev.name}`;
                    items.push(ci);
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
                    // Match both exact name, name without underscores, and opcode suffix
                    const cleanOpcode = (fn.opcode || '').replace(/^[^_]+_/, '');
                    ci.filterText = `${fn.name} ${fn.name.replace(/_/g, '')} ${cleanOpcode}`;
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
                    ci.filterText = kw.name;
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
                    ci.filterText = obj;
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
                            ci.filterText = varName;
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
    context.subscriptions.push(
        vscode.languages.registerHoverProvider('scratch', {
            async provideHover(document, position) {
                const idRange = document.getWordRangeAtPosition(position, /[a-zA-Z0-9_\.:]+/);
                const wordRange = document.getWordRangeAtPosition(position);
                const range = idRange || wordRange;
                if (!range) return null;

                const fullToken = document.getText(range);
                const baseWord = wordRange ? document.getText(wordRange) : fullToken;

                // 1. Instant check in Function Catalog (0ms response)
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
                if (lspProcess && !lspProcess.killed && lspProcess.exitCode === null) {
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
    // 3. Live Parameter Signature Help Provider (Trigger on '(' and ',')
    // =========================================================================
    context.subscriptions.push(
        vscode.languages.registerSignatureHelpProvider(
            'scratch',
            {
                provideSignatureHelp(document, position) {
                    const lineText = document.lineAt(position.line).text;
                    const textBeforeCursor = lineText.slice(0, position.character);

                    // Find opening paren of the innermost function call
                    let openParenIndex = -1;
                    let parenDepth = 0;
                    let commaCount = 0;

                    for (let i = textBeforeCursor.length - 1; i >= 0; i--) {
                        const ch = textBeforeCursor[i];
                        if (ch === ')') {
                            parenDepth++;
                        } else if (ch === '(') {
                            if (parenDepth === 0) {
                                openParenIndex = i;
                                break;
                            }
                            parenDepth--;
                        } else if (ch === ',' && parenDepth === 0) {
                            commaCount++;
                        }
                    }

                    if (openParenIndex === -1) return null;

                    // Extract function identifier immediately preceding '('
                    const beforeParen = textBeforeCursor.slice(0, openParenIndex).trim();
                    const match = beforeParen.match(/([a-zA-Z_][a-zA-Z0-9_]*)$/);
                    if (!match) return null;

                    const funcName = match[1];
                    const fn = catalog.FUNCTION_MAP.get(funcName);
                    if (!fn) return null;

                    const sigHelp = new vscode.SignatureHelp();
                    const sigInfo = new vscode.SignatureInformation(
                        `${fn.syntax} -> ${fn.returnType}`,
                        new vscode.MarkdownString(fn.description)
                    );

                    sigInfo.parameters = (fn.parameters || []).map((p) => {
                        const req = p.required ? 'required' : `optional, default: ${p.default}`;
                        return new vscode.ParameterInformation(
                            `${p.name}: ${p.type}`,
                            new vscode.MarkdownString(`**${p.name}** (\`${p.type}\`) &mdash; ${p.description} *(${req})*`)
                        );
                    });

                    sigHelp.signatures = [sigInfo];
                    sigHelp.activeSignature = 0;
                    sigHelp.activeParameter = Math.min(commaCount, (fn.parameters || []).length - 1);

                    return sigHelp;
                },
            },
            '(',
            ','
        )
    );

    // =========================================================================
    // 4. Document Formatting Provider (Shift + Alt + F)
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
    // 5. Document Symbol Provider (Outline View)
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
    // 6. Commands Registration
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
            const items = catalog.FUNCTIONS_DATA.map((fn) => {
                const icon = getCategoryIcon(fn.category);
                const paramStr = (fn.parameters || []).map((p) => (p.required ? p.name : `[${p.name}]`)).join(', ');
                return {
                    label: `${icon} ${fn.name}`,
                    description: `(${paramStr})`,
                    detail: `[${fn.category.toUpperCase()}] ${fn.shape} • ${fn.description}`,
                    fn: fn,
                };
            });

            const selected = await vscode.window.showQuickPick(items, {
                placeHolder: '🔍 Search all 151 Scratch blocks and functions by name, category, or description...',
                matchOnDescription: true,
                matchOnDetail: true,
            });

            if (selected) {
                if (editor) {
                    editor.insertSnippet(new vscode.SnippetString(selected.fn.lspSnippet));
                } else {
                    // If no editor open, copy snippet to clipboard and notify
                    await vscode.env.clipboard.writeText(selected.fn.lspSnippet);
                    vscode.window.showInformationMessage(
                        `Copied snippet for '${selected.fn.name}' to clipboard! (Open a .sch file to insert directly)`
                    );
                }
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
