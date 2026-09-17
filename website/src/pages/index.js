import React, { useState, useMemo, useEffect } from 'react';
import Head from 'next/head';
import Header from '../components/Header';
import Sidebar from '../components/Sidebar';
import FunctionCard from '../components/FunctionCard';
import SearchBar from '../components/SearchBar';
import CliSection from '../components/CliSection';
import VsCodeSection from '../components/VsCodeSection';
import { FUNCTIONS_DATA } from '../data/functionsData';
import { CATEGORIES } from '../data/categories';
import { Sparkles, Terminal, Code2, Layers, Search, ArrowUp, ExternalLink, CheckCircle2 } from 'lucide-react';

export default function Home() {
  const [activeTab, setActiveTab] = useState('functions');
  const [selectedCategory, setSelectedCategory] = useState('all');
  const [searchQuery, setSearchQuery] = useState('');
  const [isSearchModalOpen, setIsSearchModalOpen] = useState(false);
  const [darkMode, setDarkMode] = useState(true);
  const [showScrollTop, setShowScrollTop] = useState(false);

  // Sync dark mode class with documentElement
  useEffect(() => {
    if (darkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [darkMode]);

  // Scroll to top listener
  useEffect(() => {
    const handleScroll = () => {
      setShowScrollTop(window.scrollY > 400);
    };
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  // Compute count by category
  const countsByCategory = useMemo(() => {
    const counts = { all: FUNCTIONS_DATA.length };
    FUNCTIONS_DATA.forEach((fn) => {
      counts[fn.category] = (counts[fn.category] || 0) + 1;
    });
    return counts;
  }, []);

  // Filtered functions
  const filteredFunctions = useMemo(() => {
    return FUNCTIONS_DATA.filter((fn) => {
      const matchCategory = selectedCategory === 'all' || fn.category === selectedCategory;
      const matchQuery =
        !searchQuery.trim() ||
        fn.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        fn.opcode.toLowerCase().includes(searchQuery.toLowerCase()) ||
        fn.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        fn.syntax.toLowerCase().includes(searchQuery.toLowerCase());
      return matchCategory && matchQuery;
    });
  }, [selectedCategory, searchQuery]);

  const scrollToFunction = (fn) => {
    setActiveTab('functions');
    setSelectedCategory(fn.category);
    setTimeout(() => {
      const el = document.getElementById(fn.id);
      if (el) {
        el.scrollIntoView({ behavior: 'smooth', block: 'start' });
      }
    }, 100);
  };

  return (
    <div className="min-h-screen bg-slate-50 dark:bg-[#0b101b] text-slate-900 dark:text-slate-100 flex flex-col font-sans transition-colors">
      <Head>
        <title>scratch-lang Documentation & VS Code Reference</title>
        <meta name="description" content="Definitive documentation of all 147 Scratch 3.0 functions, opcodes, examples, and VS Code extension specifications for scratch-lang." />
      </Head>

      {/* Global Header */}
      <Header
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        onOpenSearch={() => setIsSearchModalOpen(true)}
        darkMode={darkMode}
        setDarkMode={setDarkMode}
        totalCount={FUNCTIONS_DATA.length}
      />

      {/* Main Content Area */}
      <main className="flex-1 max-w-7xl mx-auto w-full px-4 sm:px-6 lg:px-8 py-8">
        {/* Top Hero Banner */}
        <div className="mb-8 p-6 sm:p-8 rounded-2xl bg-gradient-to-r from-orange-500/10 via-amber-500/5 to-yellow-500/10 border border-orange-500/20 shadow-xs relative overflow-hidden">
          <div className="relative z-10 flex flex-col lg:flex-row lg:items-center justify-between gap-6">
            <div className="max-w-2xl">
              <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-orange-500/10 border border-orange-500/20 text-orange-600 dark:text-orange-400 font-semibold text-xs mb-3">
                <CheckCircle2 className="w-3.5 h-3.5" />
                <span>100% Core Scratch 3.0 + Official Extensions Complete</span>
              </div>
              <h1 className="text-2xl sm:text-4xl font-extrabold text-slate-900 dark:text-white tracking-tight">
                scratch-lang Developer Docs
              </h1>
              <p className="mt-2 text-sm sm:text-base text-slate-600 dark:text-slate-300 leading-relaxed">
                Complete API reference for all <strong className="text-orange-500">147 executable functions</strong>, block opcodes, parameters, code snippets, and tooling specifications for building the official VS Code extension.
              </p>
            </div>

            {/* Quick Metrics */}
            <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
              <div className="p-3 bg-white dark:bg-slate-900/80 rounded-xl border border-slate-200 dark:border-slate-800 text-center">
                <div className="text-xl font-bold text-orange-500 font-mono">147</div>
                <div className="text-[11px] text-slate-500">Functions</div>
              </div>
              <div className="p-3 bg-white dark:bg-slate-900/80 rounded-xl border border-slate-200 dark:border-slate-800 text-center">
                <div className="text-xl font-bold text-emerald-500 font-mono">15</div>
                <div className="text-[11px] text-slate-500">Categories</div>
              </div>
              <div className="p-3 bg-white dark:bg-slate-900/80 rounded-xl border border-slate-200 dark:border-slate-800 text-center">
                <div className="text-xl font-bold text-indigo-500 font-mono">LSP</div>
                <div className="text-[11px] text-slate-500">Enabled</div>
              </div>
              <div className="p-3 bg-white dark:bg-slate-900/80 rounded-xl border border-slate-200 dark:border-slate-800 text-center">
                <div className="text-xl font-bold text-blue-500 font-mono">Vercel</div>
                <div className="text-[11px] text-slate-500">Ready</div>
              </div>
            </div>
          </div>
        </div>

        {/* Tab Views */}
        {activeTab === 'functions' && (
          <div className="flex flex-col lg:flex-row gap-8">
            {/* Sidebar */}
            <Sidebar
              selectedCategory={selectedCategory}
              onSelectCategory={(id) => {
                setSelectedCategory(id);
                setSearchQuery('');
              }}
              countsByCategory={countsByCategory}
            />

            {/* Functions List */}
            <section className="flex-1 min-w-0 space-y-6">
              {/* Category Header & Filter */}
              <div className="flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-slate-200 dark:border-slate-800">
                <div>
                  <h2 className="text-xl font-bold text-slate-900 dark:text-white capitalize">
                    {selectedCategory === 'all' ? 'All Functions & Constructs' : `${selectedCategory} Blocks`}
                  </h2>
                  <p className="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                    Displaying {filteredFunctions.length} of {FUNCTIONS_DATA.length} functions
                  </p>
                </div>

                {/* Inline filter input */}
                <div className="relative w-full sm:w-64">
                  <Search className="w-4 h-4 text-slate-400 absolute left-3 top-2.5" />
                  <input
                    type="text"
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    placeholder="Filter in this category..."
                    className="w-full pl-9 pr-3 py-1.5 text-xs bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg focus:outline-hidden focus:border-orange-500"
                  />
                </div>
              </div>

              {/* Cards List */}
              {filteredFunctions.length === 0 ? (
                <div className="p-12 text-center bg-white dark:bg-slate-900 rounded-2xl border border-slate-200 dark:border-slate-800">
                  <p className="text-slate-500 text-sm">No functions match your current filter.</p>
                  <button
                    onClick={() => {
                      setSelectedCategory('all');
                      setSearchQuery('');
                    }}
                    className="mt-3 text-xs font-semibold text-orange-500 hover:underline"
                  >
                    Reset filters
                  </button>
                </div>
              ) : (
                <div className="space-y-6">
                  {filteredFunctions.map((fn) => (
                    <FunctionCard key={fn.id} fn={fn} />
                  ))}
                </div>
              )}
            </section>
          </div>
        )}

        {activeTab === 'cli' && <CliSection />}

        {activeTab === 'vscode' && <VsCodeSection />}
      </main>

      {/* Footer */}
      <footer className="mt-16 border-t border-slate-200 dark:border-slate-800 py-8 bg-white dark:bg-slate-900/50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col sm:flex-row items-center justify-between gap-4 text-xs text-slate-500 dark:text-slate-400">
          <div className="flex items-center gap-2">
            <span className="font-bold text-slate-800 dark:text-white">scratch-lang</span>
            <span>&bull;</span>
            <span>Native Rust Compiler & Runtime Engine</span>
          </div>

          <div className="flex items-center gap-4">
            <a href="/api/reference" target="_blank" className="hover:text-orange-500 transition">
              JSON API
            </a>
            <span>&bull;</span>
            <span className="text-emerald-500 font-semibold">100% Vercel Ready</span>
          </div>
        </div>
      </footer>

      {/* Search Modal */}
      <SearchBar
        isOpen={isSearchModalOpen}
        onClose={() => setIsSearchModalOpen(false)}
        functions={FUNCTIONS_DATA}
        onSelectFunction={scrollToFunction}
      />

      {/* Back to top button */}
      {showScrollTop && (
        <button
          onClick={() => window.scrollTo({ top: 0, behavior: 'smooth' })}
          className="fixed bottom-6 right-6 p-3 rounded-full bg-orange-500 text-white shadow-lg hover:bg-orange-600 transition"
          title="Scroll to top"
        >
          <ArrowUp className="w-5 h-5" />
        </button>
      )}
    </div>
  );
}
