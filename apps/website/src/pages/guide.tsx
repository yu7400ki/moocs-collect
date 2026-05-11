import { ArrowLeft } from "lucide-react";
import collectNotOpened from "../assets/collect-not-opened.png";
import openCollect from "../assets/open-collect.png";
import privacyAndSecurity from "../assets/privacy-and-security.png";
import { basePath } from "../utils/base-path";

const steps = [
  {
    number: 1,
    title: "警告ダイアログが表示される",
    description:
      'アプリをダブルクリックすると「"collect"は開いていません」という警告が表示されます。「完了」をクリックして閉じてください。',
    image: collectNotOpened,
    alt: '"collect"は開いていませんという警告ダイアログ',
  },
  {
    number: 2,
    title: "プライバシーとセキュリティで許可する",
    description:
      "「システム設定」→「プライバシーとセキュリティ」を開くと、下部に「このまま開く」ボタンが表示されます。クリックして許可してください。",
    image: privacyAndSecurity,
    alt: "システム設定のプライバシーとセキュリティ画面",
  },
  {
    number: 3,
    title: "確認ダイアログで「このまま開く」をクリック",
    description:
      "再度確認ダイアログが表示されます。「このまま開く」をクリックするとアプリが起動します。次回以降はこの手順は不要です。",
    image: openCollect,
    alt: "アプリを開く確認ダイアログ",
  },
];

export default function Guide() {
  return (
    <>
      <header className="border-b border-white/10 px-6 py-10">
        <div className="mx-auto flex max-w-5xl flex-col gap-6 sm:flex-row sm:items-center sm:justify-between">
          <div className="space-y-2">
            <p className="text-xs uppercase tracking-widest text-slate-400">
              Moocs Collect
            </p>
            <p className="text-sm text-slate-400">Mac インストール注意事項</p>
          </div>
          <a
            href={basePath("/")}
            className="group inline-flex items-center gap-2 text-sm font-medium text-slate-200 transition hover:text-sky-300"
          >
            <ArrowLeft className="h-4 w-4 transition-transform group-hover:-translate-x-0.5" />
            ホームへ戻る
          </a>
        </div>
      </header>

      <main className="flex-1 px-6">
        <section className="mx-auto max-w-5xl py-24">
          <div className="space-y-12">
            <div className="space-y-6">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-400">
                macOS をご利用の方へ
              </span>
              <h1 className="text-4xl font-semibold leading-tight text-slate-100 sm:text-5xl">
                初回起動時の手順
              </h1>
              <p className="text-lg leading-relaxed text-slate-300">
                macOS では、App Store
                以外からダウンロードしたアプリは初回起動時にセキュリティの警告が表示されます。以下の手順に従って許可してください。
              </p>
            </div>

            <ol className="space-y-16">
              {steps.map((step) => (
                <li key={step.number} className="space-y-6">
                  <div className="flex items-start gap-4">
                    <span className="tabular-nums flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-sky-400/40 bg-sky-400/10 text-sm font-semibold text-sky-300">
                      {step.number}
                    </span>
                    <div className="space-y-2 pt-0.5">
                      <h2 className="text-lg font-semibold text-slate-100">
                        {step.title}
                      </h2>
                      <p className="text-sm leading-relaxed text-slate-400">
                        {step.description}
                      </p>
                    </div>
                  </div>
                  <div className="ml-12">
                    <img
                      src={step.image}
                      alt={step.alt}
                      className="rounded-xl border border-white/10 shadow-lg max-h-128"
                    />
                  </div>
                </li>
              ))}
            </ol>
          </div>
        </section>
      </main>
    </>
  );
}
