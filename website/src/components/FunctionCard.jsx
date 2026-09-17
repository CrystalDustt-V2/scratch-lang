import React, { useState } from 'react';
import { Copy, Check, Terminal, ExternalLink, Code } from 'lucide-react';

const SHAPE_COLORS = {
  Stack: 'bg-blue-500/10 text-blue-600 dark:text-blue-400 border-blue-200 dark:border-blue-900',
  Reporter: 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-200 dark:border-emerald-900',
  Boolean: 'bg-purple-500/10 text-purple-600 dark:text-purple-400 border-purple-200 dark:border-purple-900',
  Hat: 'bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-200 dark:border-amber-900',
  'C-Block': 'bg-orange-500/10 text-orange-600 dark:text-orange-400 border-orange-200 dark:border-orange-900',
  Cap: 'bg-rose-500/10 text-rose-600 dark:text-rose-400 border-rose-200 dark:border-rose-900',
};

export default function FunctionCard({ fn }) {
  const [copiedSyntax, setCopiedSyntax] = useState(false);
  const [copiedExample, setCopiedExample] = useState(false);
  const [copiedSnippet, setCopiedSnippet] = useState(false);

  const copyToClipboard = (text, setter) => {
    navigator.clipboard.writeText(text);
    setter(true);
    setTimeout(() => setter(false), 2000);
  };

  const shapeClass = SHAPE_COLORS[fn.shape] || 'bg-slate-100 text-slate-700 border-slate-200';

  return (
    <article
      id={fn.id}
      className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 sm:p-6 shadow-xs hover:border-slate-300 dark:hover:border-slate-700 transition-all scroll-mt-24"
    >
      {/* Card Header */}
      <div className="flex flex-wrap items-center justify-between gap-2.5 pb-4 border-b border-slate-100 dark:border-slate-800">
        <div className="flex items-center gap-3">
          <h3 className="text-lg font-bold text-slate-900 dark:text-white font-mono tracking-tight">
            {fn.name}
          </h3>
          <span className={`text-[11px] font-semibold px-2 py-0.5 rounded-full border ${shapeClass}`}>
            {fn.shape}
          </span>
          <span className="text-[11px] font-mono px-2 py-0.5 rounded-md bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 border border-slate-200 dark:border-slate-700">
            {fn.returnType}
          </span>
        </div>

        <div className="flex items-center gap-2">
          <span className="text-[11px] font-mono text-slate-500 dark:text-slate-400 bg-slate-50 dark:bg-slate-800/50 px-2 py-1 rounded border border-slate-200/60 dark:border-slate-700/60">
            {fn.opcode}
          </span>
        </div>
      </div>

      {/* Description */}
      <p className="mt-3 text-sm text-slate-600 dark:text-slate-300 leading-relaxed">
        {fn.description}
      </p>

      {/* Syntax Box */}
      <div className="mt-4">
        <div className="flex items-center justify-between px-3 py-1.5 bg-slate-100 dark:bg-slate-800/90 rounded-t-lg border-t border-x border-slate-200 dark:border-slate-700 text-xs font-semibold text-slate-600 dark:text-slate-400">
          <span>Syntax</span>
          <button
            onClick={() => copyToClipboard(fn.syntax, setCopiedSyntax)}
            className="flex items-center gap-1 text-[11px] text-slate-500 hover:text-orange-500 transition"
          >
            {copiedSyntax ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
            <span>{copiedSyntax ? 'Copied' : 'Copy'}</span>
          </button>
        </div>
        <div className="p-3 bg-slate-900 dark:bg-slate-950 text-amber-300 font-mono text-xs rounded-b-lg border-b border-x border-slate-200 dark:border-slate-700 overflow-x-auto shadow-inner">
          {fn.syntax}
        </div>
      </div>

      {/* Parameters Table */}
      {fn.parameters && fn.parameters.length > 0 && (
        <div className="mt-4">
          <h4 className="text-xs font-bold uppercase tracking-wider text-slate-400 dark:text-slate-500 mb-2">
            Parameters
          </h4>
          <div className="overflow-x-auto rounded-lg border border-slate-200 dark:border-slate-800">
            <table className="w-full text-left text-xs">
              <thead className="bg-slate-50 dark:bg-slate-800/50 text-slate-500 dark:text-slate-400 border-b border-slate-200 dark:border-slate-800 font-medium">
                <tr>
                  <th className="p-2.5 font-mono">Param</th>
                  <th className="p-2.5">Type</th>
                  <th className="p-2.5">Required</th>
                  <th className="p-2.5">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100 dark:divide-slate-800 font-normal">
                {fn.parameters.map((p, idx) => (
                  <tr key={idx} className="hover:bg-slate-50/50 dark:hover:bg-slate-800/20">
                    <td className="p-2.5 font-mono text-orange-600 dark:text-orange-400 font-semibold">{p.name}</td>
                    <td className="p-2.5 font-mono text-slate-500 dark:text-slate-400">{p.type}</td>
                    <td className="p-2.5">
                      {p.required ? (
                        <span className="text-[10px] font-bold text-rose-500">Required</span>
                      ) : (
                        <span className="text-[10px] text-slate-400">Optional</span>
                      )}
                    </td>
                    <td className="p-2.5 text-slate-600 dark:text-slate-300">{p.description}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* Example Code Block */}
      {fn.example && (
        <div className="mt-4">
          <div className="flex items-center justify-between px-3 py-1.5 bg-slate-100 dark:bg-slate-800/90 rounded-t-lg border-t border-x border-slate-200 dark:border-slate-700 text-xs font-semibold text-slate-600 dark:text-slate-400">
            <span>Example (.sch)</span>
            <button
              onClick={() => copyToClipboard(fn.example, setCopiedExample)}
              className="flex items-center gap-1 text-[11px] text-slate-500 hover:text-orange-500 transition"
            >
              {copiedExample ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
              <span>{copiedExample ? 'Copied' : 'Copy'}</span>
            </button>
          </div>
          <pre className="p-3 bg-slate-900 dark:bg-slate-950 text-slate-200 font-mono text-xs rounded-b-lg border-b border-x border-slate-200 dark:border-slate-700 overflow-x-auto whitespace-pre leading-relaxed shadow-inner">
            <code>{fn.example}</code>
          </pre>
        </div>
      )}

      {/* Card Footer: Notes & VS Code Snippet Copy */}
      <div className="mt-4 pt-3 border-t border-slate-100 dark:border-slate-800/80 flex flex-wrap items-center justify-between gap-3 text-xs">
        <div className="text-slate-500 dark:text-slate-400 italic">
          {fn.notes && <span>Tip: {fn.notes}</span>}
        </div>

        {fn.lspSnippet && (
          <button
            onClick={() => copyToClipboard(fn.lspSnippet, setCopiedSnippet)}
            className="flex items-center gap-1.5 px-2.5 py-1 rounded bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:text-orange-500 dark:hover:text-orange-400 hover:bg-slate-200 dark:hover:bg-slate-700 transition font-mono text-[11px]"
            title="Copy VS Code snippet format"
          >
            <Code className="w-3 h-3" />
            <span>{copiedSnippet ? 'Copied Snippet!' : 'Copy VS Code Snippet'}</span>
          </button>
        )}
      </div>
    </article>
  );
}
