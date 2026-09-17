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
    CompletionItemKind: {
        Function: 2,
        Property: 9,
        Keyword: 13,
        Class: 6,
        Variable: 5,
        Value: 11,
        Event: 22,
    },
    SymbolKind: {
        Event: 23,
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
        terminals: [],
        showInformationMessage: () => {},
        showWarningMessage: () => {},
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
        registerDocumentFormattingEditProvider: () => ({ dispose: () => {} }),
        registerDocumentSymbolProvider: () => ({ dispose: () => {} }),
    },
    commands: {
        registerCommand: () => ({ dispose: () => {} }),
    },
    Uri: {
        parse: (u) => ({ toString: () => u }),
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
console.log('   OK! Extension activated and providers registered.');

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
    console.log('   OK! Hover returned instant rich documentation for "move":');
    console.log(hover.contents.value.split('\n').slice(0, 4).join('\n'));

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

    const moveItem = compList.items.find((i) => i.label === 'move');
    assert(moveItem, 'Must contain "move" completion item');
    assert.strictEqual(moveItem.insertText.value, 'move("${1:Player}", ${2:10})', 'Snippet must place cursor at parameters');

    const scoreItem = compList.items.find((i) => i.label === 'score');
    assert(scoreItem, 'Must contain local user variable "score"');
    console.log(`   OK! Completion provider returned ${compList.items.length} items with isIncomplete: false.`);

    console.log('5. Deactivating...');
    ext.deactivate();
    console.log('   OK! All VS Code extension tests passed successfully!');
}

runTests().catch((e) => {
    console.error('Test Failed:', e);
    process.exit(1);
});
