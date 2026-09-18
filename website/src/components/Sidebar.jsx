import React, { useState, useMemo } from 'react';
import { Search, ChevronDown, ChevronRight, Hash } from 'lucide-react';
import { CATEGORIES } from '../data/categories';

export default function Sidebar({
  selectedCategory,
  onSelectCategory,
  functions,
  onSelectFunction,
  activeFunctionId,
}) {
  const [filterText, setFilterText] = useState('');
  const [expandedCategories, setExpandedCategories] = useState({
    motion: true,
    looks: false,
    sound: false,
    events: false,
    control: false,
    sensing: false,
    operators: false,
    variables: false,
    lists: false,
  });

  const toggleCategory = (catId) => {
    setExpandedCategories((prev) => ({
      ...prev,
      [catId]: !prev[catId],
    }));
  };

  // Group functions by category
  const functionsByCategory = useMemo(() => {
    const map = {};
    functions.forEach((fn) => {
      if (!map[fn.category]) {
        map[fn.category] = [];
      }
      map[fn.category].push(fn);
    });
    return map;
  }, [functions]);

  // Filtered categories and functions
  const filteredCategories = useMemo(() => {
    const q = filterText.trim().toLowerCase();
    if (!q) return CATEGORIES;

    return CATEGORIES.filter((cat) => {
      const matchCat = cat.name.toLowerCase().includes(q);
      const catFns = functionsByCategory[cat.id] || [];
      const matchFn = catFns.some(
        (f) =>
          f.name.toLowerCase().includes(q) ||
          f.opcode.toLowerCase().includes(q) ||
          f.syntax.toLowerCase().includes(q)
      );
      return matchCat || matchFn;
    });
  }, [filterText, functionsByCategory]);

  return (
    <aside className="docs-sidebar">
      {/* Quick Filter Input */}
      <div className="docs-sidebar-filter">
        <input
          type="text"
          value={filterText}
          onChange={(e) => setFilterText(e.target.value)}
          placeholder="Filter functions..."
          className="docs-sidebar-input"
          aria-label="Filter functions"
        />
      </div>

      <div className="docs-sidebar-section-title">
        <span>Standard Library</span>
        <span>{functions.length} API</span>
      </div>

      {/* Categories & Function Tree */}
      <nav aria-label="API Directory">
        {filteredCategories.map((cat) => {
          const catFns = functionsByCategory[cat.id] || [];
          const isSelected = selectedCategory === cat.id;
          const isExpanded = filterText.trim().length > 0 || expandedCategories[cat.id] || isSelected;

          const visibleFns = filterText.trim()
            ? catFns.filter(
                (f) =>
                  f.name.toLowerCase().includes(filterText.toLowerCase()) ||
                  f.opcode.toLowerCase().includes(filterText.toLowerCase())
              )
            : catFns;

          return (
            <div key={cat.id} className="docs-sidebar-cat-group">
              <button
                type="button"
                onClick={() => {
                  onSelectCategory(cat.id);
                  toggleCategory(cat.id);
                }}
                className={`docs-sidebar-cat-header ${isSelected ? 'active' : ''}`}
              >
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.45rem' }}>
                  {isExpanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
                  <span>{cat.name}</span>
                </div>
                <span className="docs-sidebar-cat-count">{catFns.length}</span>
              </button>

              {isExpanded && visibleFns.length > 0 && (
                <ul className="docs-sidebar-fn-list">
                  {visibleFns.map((fn) => {
                    const isActiveFn = activeFunctionId === fn.id;
                    return (
                      <li key={fn.id} className="docs-sidebar-fn-item">
                        <a
                          href={`#${fn.id}`}
                          onClick={(e) => {
                            e.preventDefault();
                            onSelectFunction(fn);
                          }}
                          className={`docs-sidebar-fn-link ${isActiveFn ? 'active' : ''}`}
                          title={`${fn.syntax} (${fn.opcode})`}
                        >
                          {fn.name}()
                        </a>
                      </li>
                    );
                  })}
                </ul>
              )}
            </div>
          );
        })}
      </nav>
    </aside>
  );
}
