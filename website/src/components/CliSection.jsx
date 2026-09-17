import React, { useState } from 'react';
import { Terminal, Copy, Check, Sparkles, Play, Package, Search } from 'lucide-react';
import { CLI_COMMANDS } from '../data/cliData';

export default function CliSection() {
  const [copiedId, setCopiedId] = useState(null);

  const copyText = (text, id) => {
    navigator.clipboard.writeText(text);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  return (
    <div className="space-y-8 animate-in fade-in duration-150">
      {/* Hero Banner */}
      <div className="p-6 sm:p-8 rounded-2xl bg-gradient-to-br from-slate-900 via-slate-800 to-slate-950 text-white border border-slate-700/60 shadow-xl">
        <div className="flex items-center gap-3 mb-3">
          <div className="p-2 rounded-lg bg-orange-500/20 text-orange-400 border border-orange-500/30">
            <Terminal className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-xl sm:text-2xl font-bold tracking-tight">scratch Command Line Interface</h2>
            <p className="text-sm text-slate-300">Fast, standalone Rust toolchain for creating, compiling, and running Scratch projects.</p>
          </div>
        </div>

        <div className="mt-4 p-3 bg-black/50 rounded-xl border border-slate-700/80 font-mono text-xs flex items-center justify-between">
          <span className="text-emerald-400">$ cargo install --path apps/scratch-cli</span>
          <button
            onClick={() => copyText('cargo install --path apps/scratch-cli', 'install')}
            className="flex items-center gap-1 text-slate-400 hover:text-white transition"
          >
            {copiedId === 'install' ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
            <span>{copiedId === 'install' ? 'Copied' : 'Copy'}</span>
          </button>
        </div>
      </div>

      {/* Commands Grid */}
      <div className="grid grid-cols-1 gap-6">
        {CLI_COMMANDS.map((cmd, idx) => (
          <div
            key={idx}
            className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 sm:p-6 shadow-xs"
          >
            <div className="flex flex-wrap items-center justify-between gap-2 pb-3 border-b border-slate-100 dark:border-slate-800">
              <div className="flex items-center gap-2.5">
                <span className="px-2.5 py-1 rounded-md bg-orange-500/10 text-orange-600 dark:text-orange-400 font-mono font-bold text-sm">
                  scratch {cmd.name}
                </span>
              </div>
              <span className="text-xs font-mono text-slate-400">{cmd.usage}</span>
            </div>

            <p className="mt-3 text-sm text-slate-600 dark:text-slate-300">
              {cmd.description}
            </p>

            {/* Flags */}
            {cmd.flags && cmd.flags.length > 0 && (
              <div className="mt-4">
                <h4 className="text-xs font-bold uppercase tracking-wider text-slate-400 dark:text-slate-500 mb-2">
                  Flags & Options
                </h4>
                <div className="space-y-1.5 font-mono text-xs">
                  {cmd.flags.map((f, fIdx) => (
                    <div key={fIdx} className="flex flex-col sm:flex-row sm:items-center gap-1 sm:gap-4 p-2 rounded bg-slate-50 dark:bg-slate-800/40">
                      <span className="text-orange-600 dark:text-orange-400 font-semibold shrink-0">{f.flag}</span>
                      <span className="text-slate-600 dark:text-slate-300 font-sans text-xs">{f.description}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Example */}
            {cmd.example && (
              <div className="mt-4">
                <div className="flex items-center justify-between px-3 py-1.5 bg-slate-100 dark:bg-slate-800/90 rounded-t-lg border-t border-x border-slate-200 dark:border-slate-700 text-xs font-semibold text-slate-600 dark:text-slate-400">
                  <span>Usage Example</span>
                  <button
                    onClick={() => copyText(cmd.example, `cmd-${idx}`)}
                    className="flex items-center gap-1 text-[11px] text-slate-500 hover:text-orange-500 transition"
                  >
                    {copiedId === `cmd-${idx}` ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
                    <span>{copiedId === `cmd-${idx}` ? 'Copied' : 'Copy'}</span>
                  </button>
                </div>
                <pre className="p-3 bg-slate-900 dark:bg-slate-950 text-emerald-400 font-mono text-xs rounded-b-lg border-b border-x border-slate-200 dark:border-slate-700 overflow-x-auto whitespace-pre leading-relaxed shadow-inner">
                  <code>{cmd.example}</code>
                </pre>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
