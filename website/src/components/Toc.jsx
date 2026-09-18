import React from 'react';

export default function Toc({ functions, activeFunctionId, onSelectFunction }) {
  if (!functions || functions.length === 0) return null;

  return (
    <aside className="docs-toc">
      <div className="docs-toc-title">On this page</div>
      <ul className="docs-toc-list">
        {functions.map((fn) => {
          const isActive = activeFunctionId === fn.id;
          return (
            <li key={fn.id} className="docs-toc-item">
              <a
                href={`#${fn.id}`}
                onClick={(e) => {
                  e.preventDefault();
                  onSelectFunction(fn);
                }}
                className={`docs-toc-link ${isActive ? 'active' : ''}`}
                style={isActive ? { color: 'var(--docs-accent)', fontWeight: 600 } : {}}
              >
                {fn.name}
              </a>
            </li>
          );
        })}
      </ul>
    </aside>
  );
}
