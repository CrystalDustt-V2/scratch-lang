import React, { useState } from 'react';
import { Code2, Copy, Check, Download, FileCode, Cpu, Layers, Sparkles } from 'lucide-react';
import { VSCODE_REFERENCE } from '../data/vscodeData';
import { FUNCTIONS_DATA } from '../data/functionsData';

export default function VsCodeSection() {
  const [copiedId, setCopiedId] = useState(null);

  const copyText = (text, id) => {
    navigator.clipboard.writeText(text);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  const generateAllSnippetsJson = () => {
    const snippets = {};
    FUNCTIONS_DATA.forEach((fn) => {
      snippets[fn.name] = {
        prefix: fn.name,
        body: [fn.lspSnippet || fn.syntax],
        description: `${fn.description} (${fn.opcode})`,
      };
    });
    return JSON.stringify(snippets, null, 2);
  };

  const downloadSnippets = () => {
    const dataStr = 'data:text/json;charset=utf-8,' + encodeURIComponent(generateAllSnippetsJson());
    const downloadAnchor = document.createElement('a');
    downloadAnchor.setAttribute('href', dataStr);
    downloadAnchor.setAttribute('download', 'scratch.code-snippets');
    document.body.appendChild(downloadAnchor);
    downloadAnchor.click();
    downloadAnchor.remove();
  };

  return (
    <div className="space-y-8 animate-in fade-in duration-150">
      {/* Hero Banner */}
      <div className="p-6 sm:p-8 rounded-2xl bg-gradient-to-br from-indigo-950 via-slate-900 to-slate-950 text-white border border-indigo-900/60 shadow-xl">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-xl bg-indigo-500/20 text-indigo-400 border border-indigo-500/30">
              <Code2 className="w-7 h-7" />
            </div>
            <div>
              <h2 className="text-xl sm:text-2xl font-bold tracking-tight">VS Code Extension Specification</h2>
              <p className="text-sm text-slate-300">Definitive blueprint, grammar scopes, LSP bindings, and snippets for the official VS Code extension.</p>
            </div>
          </div>

          <button
            onClick={downloadSnippets}
            className="flex items-center gap-2 px-4 py-2.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs transition shadow-md shadow-indigo-600/30 shrink-0"
          >
            <Download className="w-4 h-4" />
            <span>Download All 147 Snippets</span>
          </button>
        </div>
      </div>

      {/* Architecture & LSP Features */}
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 sm:p-6 shadow-xs">
        <h3 className="text-base font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
          <Cpu className="w-4 h-4 text-orange-500" />
          <span>Language Server Protocol (LSP) Engine Features</span>
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {VSCODE_REFERENCE.lspFeatures.map((f, idx) => (
            <div key={idx} className="p-4 rounded-xl bg-slate-50 dark:bg-slate-800/40 border border-slate-100 dark:border-slate-800">
              <div className="font-semibold text-xs text-orange-600 dark:text-orange-400 font-mono mb-1">{f.feature}</div>
              <div className="text-xs text-slate-600 dark:text-slate-300 leading-relaxed">{f.description}</div>
            </div>
          ))}
        </div>
      </div>

      {/* Extension Directory Structure */}
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 sm:p-6 shadow-xs">
        <h3 className="text-base font-bold text-slate-900 dark:text-white mb-3 flex items-center gap-2">
          <FileCode className="w-4 h-4 text-indigo-500" />
          <span>Extension File Structure</span>
        </h3>
        <pre className="p-4 bg-slate-900 dark:bg-slate-950 text-indigo-300 font-mono text-xs rounded-xl overflow-x-auto shadow-inner">
          <code>{VSCODE_REFERENCE.extensionStructure}</code>
        </pre>
      </div>

      {/* package.json Manifest */}
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 sm:p-6 shadow-xs">
        <div className="flex items-center justify-between pb-3 border-b border-slate-100 dark:border-slate-800">
          <h3 className="text-sm font-bold text-slate-900 dark:text-white font-mono">
            editors/vscode/package.json
          </h3>
          <button
            onClick={() => copyText(VSCODE_REFERENCE.packageJsonConfig, 'pkg')}
            className="flex items-center gap-1 text-xs text-slate-500 hover:text-orange-500 transition"
          >
            {copiedId === 'pkg' ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
            <span>{copiedId === 'pkg' ? 'Copied' : 'Copy Manifest'}</span>
          </button>
        </div>
        <pre className="mt-3 p-4 bg-slate-900 dark:bg-slate-950 text-emerald-400 font-mono text-xs rounded-xl overflow-x-auto shadow-inner leading-relaxed">
          <code>{VSCODE_REFERENCE.packageJsonConfig}</code>
        </pre>
      </div>

      {/* language-configuration.json */}
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 sm:p-6 shadow-xs">
        <div className="flex items-center justify-between pb-3 border-b border-slate-100 dark:border-slate-800">
          <h3 className="text-sm font-bold text-slate-900 dark:text-white font-mono">
            editors/vscode/language-configuration.json
          </h3>
          <button
            onClick={() => copyText(VSCODE_REFERENCE.languageConfig, 'lang-cfg')}
            className="flex items-center gap-1 text-xs text-slate-500 hover:text-orange-500 transition"
          >
            {copiedId === 'lang-cfg' ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
            <span>{copiedId === 'lang-cfg' ? 'Copied' : 'Copy'}</span>
          </button>
        </div>
        <pre className="mt-3 p-4 bg-slate-900 dark:bg-slate-950 text-amber-300 font-mono text-xs rounded-xl overflow-x-auto shadow-inner leading-relaxed">
          <code>{VSCODE_REFERENCE.languageConfig}</code>
        </pre>
      </div>

      {/* Building & Packaging Guide */}
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 sm:p-6 shadow-xs">
        <h3 className="text-base font-bold text-slate-900 dark:text-white mb-3">
          Compiling & Testing the Extension
        </h3>
        <div className="space-y-3 text-xs text-slate-600 dark:text-slate-300 font-sans">
          <p>
            1. Install the VS Code Extension compiler tools:
          </p>
          <div className="p-3 bg-slate-900 text-slate-200 font-mono rounded-lg">
            npm install -g @vscode/vsce
          </div>
          <p>
            2. Package the extension into an installable <code className="font-mono text-orange-500 font-semibold">.vsix</code> file:
          </p>
          <div className="p-3 bg-slate-900 text-slate-200 font-mono rounded-lg">
            cd editors/vscode && vsce package
          </div>
          <p>
            3. Install the packaged <code className="font-mono text-orange-500 font-semibold">scratch-lang-0.1.0.vsix</code> directly in VS Code:
          </p>
          <div className="p-3 bg-slate-900 text-slate-200 font-mono rounded-lg">
            code --install-extension scratch-lang-0.1.0.vsix
          </div>
        </div>
      </div>
    </div>
  );
}
