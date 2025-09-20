import { ArrowUpRight, Download, FileText, Search } from "lucide-react";
import { DownloadSection } from "./components/download-section";
import "./index.css";
import { getLatestRelease } from "./utils/get-release";

export function getStaticPaths() {
  return ["/"];
}

export async function Root(_: { url: URL }) {
  const latestRelease = await getLatestRelease();

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
        <div className="relative min-h-screen">
          <div
            aria-hidden="true"
            className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_top,_rgba(56,189,248,0.18),_transparent_55%)]"
          />
          <div className="relative z-10 flex min-h-screen flex-col">
            <header className="border-b border-white/10 px-6 py-10">
              <div className="mx-auto flex max-w-5xl flex-col gap-6 sm:flex-row sm:items-center sm:justify-between">
                <div className="space-y-2">
                  <p className="text-xs uppercase tracking-widest text-slate-400">
                    Moocs Collect
                  </p>
                  <p className="text-sm text-slate-400">
                    INIAD Moocs スライドダウンローダー
                  </p>
                </div>
                <a
                  href="https://github.com/yu7400ki/moocs-collect"
                  className="group inline-flex items-center gap-2 text-sm font-medium text-slate-200 transition hover:text-sky-300"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  GitHub で見る
                  <ArrowUpRight className="h-4 w-4 transition-transform group-hover:-translate-y-0.5 group-hover:translate-x-0.5" />
                </a>
              </div>
            </header>

            <main className="flex-1 px-6">
              <section className="mx-auto max-w-5xl py-24">
                <div className="space-y-12">
                  <div className="space-y-6">
                    <span className="text-xs uppercase tracking-[0.25em] text-slate-400">
                      つながらなくても安心して学ぶために
                    </span>
                    <h1 className="text-4xl font-semibold leading-tight text-slate-100 sm:text-5xl lg:text-6xl">
                      講義資料はすべて手元に。オフラインでも迷わない準備を。
                    </h1>
                    <p className="max-w-2xl text-lg leading-relaxed text-slate-300">
                      Moocs Collect は INIAD
                      生が講義資料を確実に手元へ持ち運べるように開発されたデスクトップアプリです。ネットワークに制限がある試験環境でもローカルからすぐに閲覧できます。
                    </p>
                  </div>
                  <DownloadSection latestRelease={latestRelease} />
                </div>
              </section>

              <section className="mx-auto max-w-5xl border-t border-white/10 py-16">
                <h2 className="text-xs uppercase tracking-[0.35em] text-slate-400">
                  主な機能
                </h2>
                <div className="mt-10 grid gap-12 sm:grid-cols-3">
                  <div className="space-y-3">
                    <div className="flex items-center gap-3 text-slate-200">
                      <Download className="h-5 w-5 text-sky-300" />
                      <p className="text-base font-semibold">
                        常に最新を自動取得
                      </p>
                    </div>
                    <p className="text-sm leading-relaxed text-slate-400">
                      最新のスライドを自動でダウンロード。毎回サイトを巡回する作業から解放されます。
                    </p>
                  </div>
                  <div className="space-y-3">
                    <div className="flex items-center gap-3 text-slate-200">
                      <FileText className="h-5 w-5 text-sky-300" />
                      <p className="text-base font-semibold">
                        PDFで整理された資料
                      </p>
                    </div>
                    <p className="text-sm leading-relaxed text-slate-400">
                      取得したスライドはすべて PDF
                      として保存。復習や共有、注釈も思い通りに行えます。
                    </p>
                  </div>
                  <div className="space-y-3">
                    <div className="flex items-center gap-3 text-slate-200">
                      <Search className="h-5 w-5 text-sky-300" />
                      <p className="text-base font-semibold">
                        ローカル全文検索
                      </p>
                    </div>
                    <p className="text-sm leading-relaxed text-slate-400">
                      試験やレポート作成に必要なページへ瞬時に辿り着けます。
                    </p>
                  </div>
                </div>
              </section>
            </main>
          </div>
        </div>
      </body>
    </html>
  );
}
