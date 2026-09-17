import { Html, Head, Main, NextScript } from 'next/document';

export default function Document() {
  return (
    <Html lang="en" className="dark">
      <Head>
        <meta charSet="utf-8" />
        <meta name="description" content="Complete documentation and VS Code reference for scratch-lang (.sch)" />
        <link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🐱</text></svg>" />
      </Head>
      <body className="bg-slate-50 dark:bg-[#0b101b] text-slate-900 dark:text-slate-100 min-h-screen antialiased selection:bg-orange-500 selection:text-white">
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
