import React, { useState, useMemo, useEffect } from 'react';
import Head from 'next/head';
import Header from '../components/Header';
import Sidebar from '../components/Sidebar';
import FunctionCard from '../components/FunctionCard';
import Toc from '../components/Toc';
import LanguageTour from '../components/LanguageTour';
import CliSection from '../components/CliSection';
import VsCodeSection from '../components/VsCodeSection';
import SearchBar from '../components/SearchBar';
import { FUNCTIONS_DATA } from '../data/functionsData';
import { CATEGORIES } from '../data/categories';

export default function Home() {
  const [activeTab, setActiveTab] = useState('functions');
  const [selectedCategory, setSelectedCategory] = useState('motion');
  const [activeFunctionId, setActiveFunctionId] = useState('');
  const [isSearchModalOpen, setIsSearchModalOpen] = useState(false);
  const [darkMode, setDarkMode] = useState(true);

  // Sync dark class on documentElement
  useEffect(() => {
    if (darkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [darkMode]);

  // Keyboard shortcut Ctrl+K / Cmd+K
  useEffect(() => {
    const handleKeyDown = (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        setIsSearchModalOpen((prev) => !prev);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  // Filter functions for active category
  const displayedFunctions = useMemo(() => {
    if (selectedCategory === 'all') {
      return FUNCTIONS_DATA;
    }
    return FUNCTIONS_DATA.filter((f) => f.category === selectedCategory);
  }, [selectedCategory]);

  // Active Category Meta
  const activeCategoryMeta = useMemo(() => {
    return (
      CATEGORIES.find((c) => c.id === selectedCategory) || {
        name: 'Standard Library',
        description: 'Complete API reference for scratch-lang built-in block primitives.',
      }
    );
  }, [selectedCategory]);

  // Scroll to function handler
  const handleSelectFunction = (fn) => {
    setActiveTab('functions');
    setSelectedCategory(fn.category);
    setActiveFunctionId(fn.id);

    setTimeout(() => {
      const el = document.getElementById(fn.id);
      if (el) {
        el.scrollIntoView({ behavior: 'smooth', block: 'start' });
        // Update URL hash without reload
        if (typeof window !== 'undefined' && window.history) {
          window.history.replaceState(null, '', `#${fn.id}`);
        }
      }
    }, 50);
  };

  // Scroll spy to highlight active TOC item
  useEffect(() => {
    if (activeTab !== 'functions') return;

    const handleScroll = () => {
      const entries = displayedFunctions.map((f) => document.getElementById(f.id)).filter(Boolean);
      const scrollPos = window.scrollY + 100;

      for (let i = entries.length - 1; i >= 0; i--) {
        const el = entries[i];
        if (el && el.offsetTop <= scrollPos) {
          setActiveFunctionId(displayedFunctions[i].id);
          break;
        }
      }
    };

    window.addEventListener('scroll', handleScroll, { passive: true });
    return () => window.removeEventListener('scroll', handleScroll);
  }, [activeTab, displayedFunctions]);

  return (
    <div className="docs-app">
      <Head>
        <title>scratch-lang Documentation &amp; API Reference</title>
        <meta
          name="description"
          content="Official documentation, standard library reference, language specification, and VS Code extension manual for scratch-lang (.sch)."
        />
        <link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🐱</text></svg>" />
      </Head>

      {/* Top Fixed Header */}
      <Header
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        onOpenSearch={() => setIsSearchModalOpen(true)}
        darkMode={darkMode}
        setDarkMode={setDarkMode}
        totalCount={FUNCTIONS_DATA.length}
      />

      {/* Main Documentation Layout */}
      <div className="docs-layout-container">
        {activeTab === 'functions' && (
          <>
            {/* Left API Directory Sidebar */}
            <Sidebar
              selectedCategory={selectedCategory}
              onSelectCategory={(catId) => {
                setSelectedCategory(catId);
                // Scroll to top of content
                window.scrollTo({ top: 0, behavior: 'smooth' });
              }}
              functions={FUNCTIONS_DATA}
              onSelectFunction={handleSelectFunction}
              activeFunctionId={activeFunctionId}
            />

            {/* Center Documentation Stream */}
            <main className="docs-content">
              {/* Breadcrumbs */}
              <nav className="docs-breadcrumbs" aria-label="Breadcrumb">
                <a href="#top" onClick={() => setSelectedCategory('all')}>Docs</a>
                <span>/</span>
                <a href="#top" onClick={() => setSelectedCategory('all')}>Standard Library</a>
                <span>/</span>
                <span style={{ color: 'var(--docs-text)', fontWeight: 600 }}>{activeCategoryMeta.name}</span>
              </nav>

              {/* Category Hero / Header */}
              <header className="docs-category-hero">
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '1rem', flexWrap: 'wrap' }}>
                  <h1>{activeCategoryMeta.name} Module</h1>
                  <span className="docs-version-pill" style={{ fontSize: '0.8rem', padding: '0.2rem 0.6rem' }}>
                    {displayedFunctions.length} Functions
                  </span>
                </div>
                <p>{activeCategoryMeta.description}</p>
              </header>

              {/* Function Documentation Entries */}
              <div className="docs-function-list">
                {displayedFunctions.map((fn) => (
                  <FunctionCard key={fn.id} fn={fn} />
                ))}
              </div>
            </main>

            {/* Right "On This Page" TOC Sidebar */}
            <Toc
              functions={displayedFunctions}
              activeFunctionId={activeFunctionId}
              onSelectFunction={handleSelectFunction}
            />
          </>
        )}

        {activeTab === 'tour' && (
          <main className="docs-content" style={{ margin: '0 auto', maxWidth: '900px' }}>
            <LanguageTour />
          </main>
        )}

        {activeTab === 'cli' && (
          <main className="docs-content" style={{ margin: '0 auto', maxWidth: '900px' }}>
            <CliSection />
          </main>
        )}

        {activeTab === 'vscode' && (
          <main className="docs-content" style={{ margin: '0 auto', maxWidth: '900px' }}>
            <VsCodeSection />
          </main>
        )}
      </div>

      {/* Spotlight Search Modal (Ctrl + K) */}
      <SearchBar
        isOpen={isSearchModalOpen}
        onClose={() => setIsSearchModalOpen(false)}
        functions={FUNCTIONS_DATA}
        onSelectFunction={handleSelectFunction}
      />
    </div>
  );
}
