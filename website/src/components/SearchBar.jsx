import React, { useState, useEffect, useRef } from 'react';
import { Search, X } from 'lucide-react';

export default function SearchBar({ isOpen, onClose, functions, onSelectFunction }) {
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef(null);

  useEffect(() => {
    if (isOpen) {
      setTimeout(() => inputRef.current?.focus(), 40);
    } else {
      setQuery('');
      setSelectedIndex(0);
    }
  }, [isOpen]);

  useEffect(() => {
    const handleKeyDown = (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        onClose();
      }
      if (e.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onClose]);

  if (!isOpen) return null;

  const filtered = query.trim()
    ? functions.filter(
        (f) =>
          f.name.toLowerCase().includes(query.toLowerCase()) ||
          f.opcode.toLowerCase().includes(query.toLowerCase()) ||
          f.description.toLowerCase().includes(query.toLowerCase()) ||
          f.syntax.toLowerCase().includes(query.toLowerCase())
      ).slice(0, 15)
    : functions.slice(0, 10);

  const handleKeyDownInInput = (e) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev + 1) % Math.max(1, filtered.length));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev - 1 + filtered.length) % Math.max(1, filtered.length));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (filtered[selectedIndex]) {
        onSelectFunction(filtered[selectedIndex]);
        onClose();
      }
    }
  };

  return (
    <div className="docs-modal-overlay" onClick={onClose}>
      <div className="docs-modal-dialog" onClick={(e) => e.stopPropagation()}>
        {/* Input */}
        <div className="docs-modal-input-wrap">
          <Search size={18} style={{ color: 'var(--docs-text-muted)' }} />
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setSelectedIndex(0);
            }}
            onKeyDown={handleKeyDownInInput}
            placeholder="Search all 161 functions, opcodes, parameters..."
            className="docs-modal-input"
            aria-label="Search functions"
          />
          {query && (
            <button
              type="button"
              onClick={() => setQuery('')}
              className="docs-icon-btn"
              style={{ width: '24px', height: '24px' }}
            >
              <X size={14} />
            </button>
          )}
          <kbd className="docs-kbd-badge">ESC</kbd>
        </div>

        {/* Results */}
        <div className="docs-modal-results">
          {filtered.length === 0 ? (
            <div style={{ padding: '2rem 1rem', textAlign: 'center', color: 'var(--docs-text-muted)', fontSize: '0.9rem' }}>
              No functions found matching &ldquo;{query}&rdquo;.
            </div>
          ) : (
            filtered.map((fn, idx) => {
              const isSelected = idx === selectedIndex;
              return (
                <div
                  key={fn.id}
                  onClick={() => {
                    onSelectFunction(fn);
                    onClose();
                  }}
                  onMouseEnter={() => setSelectedIndex(idx)}
                  className={`docs-modal-result-item ${isSelected ? 'selected' : ''}`}
                >
                  <div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                      <span className="docs-modal-result-title">{fn.name}()</span>
                      <span className="doc-badge" style={{ fontSize: '0.68rem', padding: '0.1rem 0.35rem' }}>
                        {fn.category}
                      </span>
                      <span style={{ fontSize: '0.74rem', fontFamily: 'var(--font-mono)', color: 'var(--docs-text-muted)' }}>
                        {fn.opcode}
                      </span>
                    </div>
                    <div className="docs-modal-result-desc">{fn.description}</div>
                  </div>
                  <kbd className="docs-kbd-badge" style={{ opacity: isSelected ? 1 : 0.4 }}>↵</kbd>
                </div>
              );
            })
          )}
        </div>
      </div>
    </div>
  );
}
