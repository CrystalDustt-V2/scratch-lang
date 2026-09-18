import React, { useState } from 'react';
import { Copy, Check } from 'lucide-react';

export default function FunctionCard({ fn }) {
  const [copiedSig, setCopiedSig] = useState(false);
  const [copiedExample, setCopiedExample] = useState(false);

  const copyText = (text, setter) => {
    navigator.clipboard.writeText(text);
    setter(true);
    setTimeout(() => setter(false), 2000);
  };

  const shapeClass = `shape-${fn.shape.toLowerCase().replace(/[^a-z0-9]/g, '')}`;

  return (
    <article id={fn.id} className="doc-func-entry">
      {/* Title & Metadata Badges */}
      <div className="doc-func-header">
        <div className="doc-func-title-wrap">
          <h3 className="doc-func-title">
            <span>{fn.name}</span>
            <a href={`#${fn.id}`} className="doc-anchor-link" aria-label={`Direct link to ${fn.name}`}>
              #
            </a>
          </h3>
        </div>

        <div className="doc-func-badges">
          <span className={`doc-badge ${shapeClass}`}>
            {fn.shape} Block
          </span>
          <span className="doc-badge opcode">
            {fn.opcode}
          </span>
          <span className="doc-badge">
            returns {fn.returnType}
          </span>
        </div>
      </div>

      {/* Function Signature Box */}
      <div className="doc-sig-box">
        <button
          type="button"
          onClick={() => copyText(fn.syntax, setCopiedSig)}
          className="doc-copy-btn"
          title="Copy signature"
        >
          {copiedSig ? <Check size={13} /> : <Copy size={13} />}
          <span>{copiedSig ? 'Copied' : 'Copy'}</span>
        </button>
        <pre className="doc-sig-code">
          <code>pub fn {fn.syntax} -&gt; {fn.returnType}</code>
        </pre>
      </div>

      {/* Description */}
      <p className="doc-func-desc">{fn.description}</p>

      {/* Parameters Table */}
      {fn.parameters && fn.parameters.length > 0 && (
        <div className="doc-params-section">
          <h4 className="doc-subheading">Parameters</h4>
          <table className="doc-table">
            <thead>
              <tr>
                <th>Parameter</th>
                <th>Type</th>
                <th>Requirement</th>
                <th>Default</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              {fn.parameters.map((p, idx) => (
                <tr key={idx}>
                  <td className="doc-param-name">{p.name}</td>
                  <td className="doc-param-type">{p.type}</td>
                  <td>
                    <span className={`doc-param-req ${p.required ? 'required' : 'optional'}`}>
                      {p.required ? 'Required' : 'Optional'}
                    </span>
                  </td>
                  <td className="doc-param-type">{p.default || '—'}</td>
                  <td>{p.description}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {/* Returns */}
      <div className="doc-returns-section">
        <h4 className="doc-subheading">Return Value</h4>
        <p className="doc-func-desc" style={{ marginBottom: '0.75rem', fontSize: '0.9rem' }}>
          <code>{fn.returnType}</code> — {fn.returnType === 'Void' ? 'This function executes a command and does not return a value.' : `Returns a ${fn.returnType} value.`}
        </p>
      </div>

      {/* Code Example */}
      {fn.example && (
        <div className="doc-example-section">
          <h4 className="doc-subheading">Example (.sch)</h4>
          <div className="doc-example-box">
            <div className="doc-example-header">
              <span>scratch-lang</span>
              <button
                type="button"
                onClick={() => copyText(fn.example, setCopiedExample)}
                className="doc-copy-btn"
                style={{ position: 'static' }}
              >
                {copiedExample ? <Check size={13} /> : <Copy size={13} />}
                <span>{copiedExample ? 'Copied' : 'Copy'}</span>
              </button>
            </div>
            <pre className="doc-example-code">
              <code>{fn.example}</code>
            </pre>
          </div>
        </div>
      )}

      {/* Notes / Callout */}
      {fn.notes && (
        <div className="doc-callout">
          <strong>Behavior Details:</strong> {fn.notes}
        </div>
      )}
    </article>
  );
}
