import React, { useState } from 'react';
import { Copy, Check, Download, Code2 } from 'lucide-react';
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
    <div className="docs-vscode-container" style={{ maxWidth: '880px' }}>
      {/* Category Overview */}
      <div className="docs-category-hero">
        <h1>VS Code Extension &amp; LSP Tooling Specification</h1>
        <p>
          The official VS Code extension for scratch-lang provides instant 0ms client-side hover tooltips, typeahead autocompletion, live parameter signature help, quick block search (<code>Ctrl+Shift+P &gt; scratch.searchFunctions</code>), and direct bindings to the Rust <code>scratch lsp</code> daemon.
        </p>

        <div style={{ marginTop: '1.25rem', display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
          <button
            type="button"
            onClick={downloadSnippets}
            className="doc-copy-btn"
            style={{ position: 'static', padding: '0.45rem 0.85rem', fontSize: '0.82rem' }}
          >
            <Download size={14} />
            <span>Download All {FUNCTIONS_DATA.length} Snippets (.json)</span>
          </button>
        </div>
      </div>

      {/* 1. Language Server Protocol Features */}
      <section className="doc-func-entry">
        <h2 className="doc-func-title" style={{ fontSize: '1.35rem', marginBottom: '0.5rem' }}>
          1. LSP &amp; Editor Capabilities
        </h2>
        <table className="doc-table">
          <thead>
            <tr>
              <th style={{ width: '220px' }}>Capability</th>
              <th>Technical Implementation</th>
            </tr>
          </thead>
          <tbody>
            {VSCODE_REFERENCE.lspFeatures.map((f, idx) => (
              <tr key={idx}>
                <td className="doc-param-name">
                  <code>{f.feature}</code>
                </td>
                <td>{f.description}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>

      {/* 2. Installation & Build */}
      <section className="doc-func-entry">
        <h2 className="doc-func-title" style={{ fontSize: '1.35rem', marginBottom: '0.5rem' }}>
          2. Packaging &amp; Local Installation
        </h2>
        <p className="doc-func-desc">
          To package and install the extension locally into VS Code from source:
        </p>

        <div className="doc-example-box">
          <div className="doc-example-header">
            <span>Terminal</span>
            <button
              type="button"
              onClick={() =>
                copyText(
                  `cd editors/vscode
npx --yes @vscode/vsce package
code --install-extension scratch-lang-1.0.0.vsix`,
                  'install-guide'
                )
              }
              className="doc-copy-btn"
              style={{ position: 'static' }}
            >
              {copiedId === 'install-guide' ? <Check size={13} /> : <Copy size={13} />}
              <span>{copiedId === 'install-guide' ? 'Copied' : 'Copy'}</span>
            </button>
          </div>
          <pre className="doc-example-code">
            <code>{`# 1. Package the extension VSIX bundle
cd editors/vscode
npx --yes @vscode/vsce package

# 2. Install directly into your VS Code editor
code --install-extension scratch-lang-1.0.0.vsix`}</code>
          </pre>
        </div>

        <div className="doc-callout">
          <strong>Instant Client-Side Cache:</strong> The extension ships with <code>src/catalog.js</code> containing pre-indexed definitions for all {FUNCTIONS_DATA.length} functions. Hovering or typing functions yields zero latency without waiting for process IPC roundtrips.
        </div>
      </section>

      {/* 3. Extension Layout */}
      <section className="doc-func-entry">
        <h2 className="doc-func-title" style={{ fontSize: '1.35rem', marginBottom: '0.5rem' }}>
          3. Extension Architecture
        </h2>
        <div className="doc-example-box">
          <div className="doc-example-header">
            <span>directory tree</span>
          </div>
          <pre className="doc-example-code">
            <code>{VSCODE_REFERENCE.extensionStructure}</code>
          </pre>
        </div>
      </section>
    </div>
  );
}
