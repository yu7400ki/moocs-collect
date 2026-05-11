import type React from "react";

export default function Root({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta
          name="description"
          content="講義資料はすべて手元に。オフラインでも迷わない準備を。"
        />
        <title>Moocs Collect | INIAD Moocs スライドダウンローダー</title>
      </head>
      <body className="bg-slate-950 text-slate-100 antialiased">
        {children}
      </body>
    </html>
  );
}
