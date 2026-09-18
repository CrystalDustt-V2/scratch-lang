const assert = require('assert');
const path = require('path');

// Mock vscode module
const mockVscode = {
    MarkdownString: class {
        constructor(text = '') {
            this.value = text;
        }
        appendMarkdown(txt) {
            this.value += txt;
        }
    },
    Range: class {
        constructor(sl, sc, el, ec) {
            this.start = { line: sl, character: sc };
            this.end = { line: el, character: ec };
        }
    },
    Hover: class {
        constructor(contents, range) {
            this.contents = contents;
            this.range = range;
        }
    },
    CompletionItem: class {
        constructor(label, kind) {
            this.label = label;
            this.kind = kind;
        }
    },
    CompletionList: class {
        constructor(items, isIncomplete) {
            this.items = items;
            this.isIncomplete = isIncomplete;
        }
    },
    SnippetString: class {
        constructor(value) {
            this.value = value;
        }
    },
    SignatureHelp: class {
        constructor() {
            this.signatures = [];
            this.activeSignature = 0;
            this.activeParameter = 0;
        }
    },
    SignatureInformation: class {
        constructor(label, doc) {
            this.label = label;
            this.documentation = doc;
            this.parameters = [];
        }
    },
    ParameterInformation: class {
        constructor(label, doc) {
            this.label = label;
            this.documentation = doc;
        }
    },
    CompletionItemKind: {
        Function: 2,
        Property: 9,
        Keyword: 13,
        Class: 6,
        Variable: 5,
        Value: 11,
        Event: 22,
        Operator: 24,
        File: 17,
    },
    SymbolKind: {
        Event: 23,
    },
    StatusBarAlignment: {
        Left: 1,
        Right: 2,
    },
    ThemeColor: class {
        constructor(id) {
            this.id = id;
        }
    },
    workspace: {
        getConfiguration: () => ({
            get: (key) => (key.endsWith('enable') ? true : 'scratch'),
        }),
        workspaceFolders: [{ uri: { fsPath: path.resolve(__dirname, '..') } }],
        textDocuments: [],
        onDidOpenTextDocument: () => ({ dispose: () => {} }),
        onDidChangeTextDocument: () => ({ dispose: () => {} }),
        onDidCloseTextDocument: () => ({ dispose: () => {} }),
    },
    window: {
        createOutputChannel: () => ({ appendLine: () => {} }),
        createStatusBarItem: (alignment, priority) => ({
            text: '',
            tooltip: '',
            command: '',
            show: () => {},
            dispose: () => {},
        }),
        terminals: [],
        showInformationMessage: () => {},
        showWarningMessage: () => {},
        showQuickPick: async () => null,
    },
    languages: {
        createDiagnosticCollection: () => ({ set: () => {}, delete: () => {} }),
        registerCompletionItemProvider: (lang, provider, ...triggers) => {
            mockVscode._completionProvider = provider;
            return { dispose: () => {} };
        },
        registerHoverProvider: (lang, provider) => {
            mockVscode._hoverProvider = provider;
            return { dispose: () => {} };
        },
        registerSignatureHelpProvider: (lang, provider, ...triggers) => {
            mockVscode._signatureHelpProvider = provider;
            return { dispose: () => {} };
        },
        registerDocumentFormattingEditProvider: () => ({ dispose: () => {} }),
        registerDocumentSymbolProvider: () => ({ dispose: () => {} }),
    },
    commands: {
        registerCommand: (cmd, handler) => {
            mockVscode._commands = mockVscode._commands || {};
            mockVscode._commands[cmd] = handler;
            return { dispose: () => {} };
        },
    },
    Uri: {
        parse: (u) => ({ toString: () => u }),
    },
    env: {
        clipboard: {
            writeText: async () => {},
        },
    },
};

// Intercept require('vscode')
const Module = require('module');
const originalRequire = Module.prototype.require;
Module.prototype.require = function (modulePath) {
    if (modulePath === 'vscode') {
        return mockVscode;
    }
    return originalRequire.apply(this, arguments);
};

// Now require extension
const ext = require('../editors/vscode/src/extension');
const catalog = require('../editors/vscode/src/catalog');

console.log('1. Verifying Catalog...');
assert(catalog.FUNCTIONS_DATA.length >= 147, 'Should have at least 147 functions');
assert(catalog.FUNCTION_MAP.has('move'), 'Should contain "move"');
assert(catalog.FUNCTION_MAP.has('set_x'), 'Should contain "set_x"');
assert(catalog.FUNCTION_MAP.has('say'), 'Should contain "say"');
assert(catalog.KEYWORD_MAP.has('when'), 'Should contain keyword "when"');
console.log(`   OK! Catalog has ${catalog.FUNCTIONS_DATA.length} functions.`);

console.log('2. Activating Extension with mock context...');
const context = { subscriptions: [] };
ext.activate(context);
assert(mockVscode._completionProvider, 'Completion provider must be registered');
assert(mockVscode._hoverProvider, 'Hover provider must be registered');
assert(mockVscode._signatureHelpProvider, 'Signature help provider must be registered');
assert(mockVscode._commands['scratch.searchFunctions'], 'scratch.searchFunctions must be registered');
console.log('   OK! Extension activated, providers and commands registered.');

console.log('3. Testing Instant Hover Provider (0ms latency, no loading freeze)...');
const mockDoc = {
    uri: { toString: () => 'file:///project/main.sch' },
    getText: (r) => (r ? 'move' : 'when start:\n    move("Player", 10)\n'),
    getWordRangeAtPosition: (pos, regex) => new mockVscode.Range(pos.line, 4, pos.line, 8),
    lineAt: (line) => ({ text: '    move("Player", 10)' }),
};

async function runTests() {
    // Hover over 'move'
    const hover = await mockVscode._hoverProvider.provideHover(mockDoc, { line: 1, character: 6 });
    assert(hover, 'Hover must return a result');
    assert(hover.contents.value.includes('move(target, steps, [dy])'), 'Hover markdown must contain function signature');
    assert(hover.contents.value.includes('motion_movesteps'), 'Hover markdown must contain opcode');
    console.log('   OK! Hover returned instant rich documentation for "move".');

    // Hover over keyword 'when'
    const mockDocWhen = {
        uri: { toString: () => 'file:///project/main.sch' },
        getText: () => 'when',
        getWordRangeAtPosition: () => new mockVscode.Range(0, 0, 0, 4),
    };
    const hoverWhen = await mockVscode._hoverProvider.provideHover(mockDocWhen, { line: 0, character: 2 });
    assert(hoverWhen, 'Hover over "when" must return result');
    assert(hoverWhen.contents.value.includes('when <trigger>:'), 'Hover must describe when trigger');
    console.log('   OK! Hover returned instant documentation for keyword "when".');

    console.log('4. Testing Autocompletion Provider (every keystroke typeahead)...');
    const mockDocTyping = {
        lineAt: (line) => ({ text: '    m' }),
        getText: () => 'when start:\n    score = 100\n    m',
    };
    const compList = await mockVscode._completionProvider.provideCompletionItems(mockDocTyping, { line: 1, character: 5 });
    assert(compList, 'Completion provider must return CompletionList');
    assert.strictEqual(compList.isIncomplete, false, 'isIncomplete must be false so VS Code uses native client-side fuzzy ranking');
    assert(compList.items.length >= 150, `Expected at least 150 completion items, got ${compList.items.length}`);

    const moveItem = compList.items.find(
        (i) => i.label === 'move' || (i.label && i.label.label === 'move')
    );
    assert(moveItem, 'Must contain "move" completion item');
    assert(moveItem.label.detail.includes('(target, steps'), 'Item detail in popup row must show parameters');
    assert(moveItem.label.description.includes('motion • Stack'), 'Item description on right must show category and shape');
    assert.strictEqual(moveItem.insertText.value, 'move("${1:Player}", ${2:10})', 'Snippet must place cursor at parameters');

    const whenItem = compList.items.find(
        (i) => i.label === 'when' || (i.label && i.label.label === 'when')
    );
    assert(whenItem, 'Must contain "when" keyword item');
    assert.strictEqual(whenItem.kind, mockVscode.CompletionItemKind.Keyword);

    const scoreItem = compList.items.find(
        (i) => i.label === 'score' || (i.label && i.label.label === 'score')
    );
    assert(scoreItem, 'Must contain local user variable "score"');
    console.log(`   OK! Completion provider returned ${compList.items.length} items with CompletionItemLabel formatting.`);

    console.log('5. Testing Live Parameter Signature Help...');
    const docSig = {
        lineAt: () => ({ text: '    move("Player", ' }),
    };
    const sigHelp = mockVscode._signatureHelpProvider.provideSignatureHelp(docSig, { line: 0, character: 19 });
    assert(sigHelp, 'Must return signature help for move');
    assert.strictEqual(sigHelp.activeParameter, 1, 'Should highlight second parameter (steps)');
    assert(sigHelp.signatures[0].parameters.length >= 2, 'Should have at least 2 parameters');
    console.log('   OK! Signature help highlighted active parameter (steps).');

    console.log('6. Testing Search Functions & Blocks Command...');
    assert(typeof mockVscode._commands['scratch.searchFunctions'] === 'function');
    await mockVscode._commands['scratch.searchFunctions']();
    console.log('   OK! Search Functions command executed cleanly.');

    console.log('7. Deactivating...');
    ext.deactivate();
    console.log('   OK! All VS Code extension tests passed successfully!');
}

runTests().then(() => {
    process.exit(0);
}).catch((e) => {
    console.error('Test Failed:', e);
    process.exit(1);
});
