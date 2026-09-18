import React, { useState } from 'react';
import { Copy, Check, Terminal } from 'lucide-react';
import { CLI_COMMANDS } from '../data/cliData';

export default function CliSection() {
  const [copiedId, setCopiedId] = useState(null);

  const copyText = (text, id) => {
    navigator.clipboard.writeText(text);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  return (
    <div className="docs-cli-container" style={{ maxWidth: '880px' }}>
      {/* Category Overview */}
      <div className="docs-category-hero">
        <h1>Command Line Interface (CLI) Reference</h1>
        <p>
          The <code>scratch</code> CLI is a standalone native toolchain written in Rust for compiling, running, checking, and providing Language Server Protocol (LSP) services for scratch-lang projects.
        </p>

        <div className="doc-sig-box" style={{ marginTop: '1.25rem' }}>
          <button
            type="button"
            onClick={() => copyText('cargo install --path apps/scratch-cli', 'install')}
            className="doc-copy-btn"
          >
            {copiedId === 'install' ? <Check size={13} /> : <Copy size={13} />}
            <span>{copiedId === 'install' ? 'Copied' : 'Copy'}</span>
          </button>
          <pre className="doc-sig-code">
            <code>$ cargo install --path apps/scratch-cli</code>
          </pre>
        </div>
      </div>

      {/* Commands List */}
      <div>
        {CLI_COMMANDS.map((cmd, idx) => (
          <article key={idx} id={`cli-${cmd.name}`} className="doc-func-entry">
            <div className="doc-func-header">
              <div className="doc-func-title-wrap">
                <h3 className="doc-func-title">
                  <span>scratch {cmd.name}</span>
                  <a href={`#cli-${cmd.name}`} className="doc-anchor-link">#</a>
                </h3>
              </div>
              <span className="doc-badge opcode">{cmd.usage}</span>
            </div>

            <p className="doc-func-desc">{cmd.description}</p>

            {/* Flags Table */}
            {cmd.flags && cmd.flags.length > 0 && (
              <div>
                <h4 className="doc-subheading">Options &amp; Flags</h4>
                <table className="doc-table">
                  <thead>
                    <tr>
                      <th style={{ width: '220px' }}>Flag</th>
                      <th>Description</th>
                    </tr>
                  </thead>
                  <tbody>
                    {cmd.flags.map((f, fIdx) => (
                      <tr key={fIdx}>
                        <td className="doc-param-name" style={{ whiteSpace: 'nowrap' }}>
                          <code>{f.flag}</code>
                        </td>
                        <td>{f.description}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}

            {/* Example */}
            {cmd.example && (
              <div>
                <h4 className="doc-subheading">Command Example</h4>
                <div className="doc-example-box">
                  <div className="doc-example-header">
                    <span>Terminal</span>
                    <button
                      type="button"
                      onClick={() => copyText(cmd.example, `cmd-${idx}`)}
                      className="doc-copy-btn"
                      style={{ position: 'static' }}
                    >
                      {copiedId === `cmd-${idx}` ? <Check size={13} /> : <Copy size={13} />}
                      <span>{copiedId === `cmd-${idx}` ? 'Copied' : 'Copy'}</span>
                    </button>
                  </div>
                  <pre className="doc-example-code">
                    <code>{cmd.example}</code>
                  </pre>
                </div>
              </div>
            )}
          </article>
        ))}
      </div>
    </div>
  );
}
