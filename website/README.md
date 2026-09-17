# scratch-lang Documentation & VS Code Extension Reference

A modern, responsive, developer-oriented documentation portal and API reference for the **scratch-lang** ecosystem.

Designed to serve as both an interactive manual for creators and the reference manual for building the official **scratch-lang VS Code Extension**.

---

## 🌟 Highlights

- **147 Functions Documented**: Every single Scratch 3.0 visual block and software extension mapped to its exact `.sch` syntax, opcode, parameters, return types, and runnable code examples.
- **VS Code Extension Ready**:
  - Full Language Server Protocol (LSP) specifications (`textDocument/hover`, `textDocument/completion`, `textDocument/documentSymbol`, `textDocument/formatting`).
  - Pre-generated VS Code snippets for all 147 functions downloadable with one click (`scratch.code-snippets`).
  - Ready-to-use TextMate syntax grammar scopes and `language-configuration.json`.
- **Machine-Readable API**: Endpoint at `/api/reference` serving complete JSON definitions with filtering support.
- **Searchable**: Real-time modal search with keyboard shortcuts (`Ctrl+K` or `/`).
- **Dark/Light Themes**: Modern UI styled with Tailwind CSS, Lucide icons, and responsive desktop/mobile layouts.

---

## 🚀 Local Development

```bash
# 1. Enter website directory
cd website

# 2. Install dependencies (if not already installed)
npm install

# 3. Start development server
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

---

## ☁️ Deploying to Vercel

This website is **100% pre-configured for Vercel deployment** with root and local `vercel.json` configurations.

### Option 1: Vercel CLI (Fastest)

```bash
# In the website directory:
cd website
npx vercel

# For production deployment:
npx vercel --prod
```

### Option 2: Connect GitHub Repository on Vercel Dashboard

1. Push your repository to GitHub.
2. Log into [Vercel](https://vercel.com) and click **"Add New Project"**.
3. Import your `scratch-lang` repository.
4. Set the **Root Directory** to `website` (or leave as root; the top-level `vercel.json` will automatically direct the build to `website/`).
5. Click **"Deploy"**!

---

## 🔌 Using as Reference for the VS Code Extension

1. Visit the **VS Code Spec** tab on the website or download the JSON definitions via `GET /api/reference`.
2. Drop the downloaded `scratch.code-snippets` directly into `editors/vscode/snippets/scratch.code-snippets`.
3. Use the TextMate scopes and LSP request/response definitions in `website/src/data/vscodeData.js` to implement syntax tokens and auto-completions.
