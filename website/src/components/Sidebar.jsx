import React from 'react';
import { 
  Layers, Move, Eye, Volume2, Zap, GitBranch, Compass, 
  Binary, Database, ListFilter, Code2, PenTool, Music, Mic, Languages, Gamepad2 
} from 'lucide-react';
import { CATEGORIES } from '../data/categories';

const ICON_MAP = {
  Layers, Move, Eye, Volume2, Zap, GitBranch, Compass,
  Binary, Database, ListFilter, Code2, PenTool, Music, Mic, Languages, Gamepad2
};

export default function Sidebar({ selectedCategory, onSelectCategory, countsByCategory }) {
  return (
    <aside className="w-full lg:w-64 shrink-0">
      <div className="sticky top-20 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-3 shadow-xs">
        <div className="px-3 py-2 mb-1 flex items-center justify-between">
          <span className="text-xs font-bold uppercase tracking-wider text-slate-400 dark:text-slate-500">Categories</span>
          <span className="text-[11px] font-semibold px-2 py-0.5 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-500">
            147 Total
          </span>
        </div>

        <nav className="space-y-0.5">
          {CATEGORIES.map((cat) => {
            const IconComponent = ICON_MAP[cat.icon] || Layers;
            const isSelected = selectedCategory === cat.id;
            const count = countsByCategory[cat.id] ?? cat.count;

            return (
              <button
                key={cat.id}
                onClick={() => onSelectCategory(cat.id)}
                className={`w-full flex items-center justify-between px-3 py-2 text-xs font-medium rounded-lg transition-all text-left ${
                  isSelected
                    ? 'bg-orange-500/10 text-orange-600 dark:text-orange-400 font-semibold'
                    : 'text-slate-600 dark:text-slate-400 hover:bg-slate-50 dark:hover:bg-slate-800/60 hover:text-slate-900 dark:hover:text-slate-200'
                }`}
              >
                <div className="flex items-center gap-2.5">
                  <span
                    className="w-2 h-2 rounded-full shrink-0"
                    style={{ backgroundColor: cat.color }}
                  />
                  <IconComponent className="w-3.5 h-3.5 shrink-0 opacity-70" />
                  <span className="truncate">{cat.name}</span>
                </div>
                <span
                  className={`text-[10px] px-1.5 py-0.5 rounded-full font-mono ${
                    isSelected
                      ? 'bg-orange-500 text-white font-bold'
                      : 'bg-slate-100 dark:bg-slate-800 text-slate-500'
                  }`}
                >
                  {count}
                </span>
              </button>
            );
          })}
        </nav>
      </div>
    </aside>
  );
}
