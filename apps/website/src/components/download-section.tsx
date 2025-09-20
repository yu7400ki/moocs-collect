"use client";

import type { LatestRelease } from "../types/release";
import { detectUserOS } from "../utils/detect-os";
import { DownloadButton } from "./download-button";

type Props = {
  latestRelease: LatestRelease;
};

export function DownloadSection({ latestRelease }: Props) {
  const userOS = detectUserOS();

  const platforms = Object.entries(latestRelease.platforms);
  const primaryPlatformKey =
    userOS && Object.hasOwn(latestRelease.platforms, userOS) ? userOS : null;
  const primaryPlatform =
    primaryPlatformKey !== null
      ? {
          platform: primaryPlatformKey,
          ...latestRelease.platforms[primaryPlatformKey],
        }
      : null;

  const displayPlatforms =
    primaryPlatformKey !== null
      ? platforms.filter(([platform]) => platform !== primaryPlatformKey)
      : platforms;

  const publishedAt = new Date(latestRelease.pub_date).toLocaleDateString(
    "ja-JP",
  );

  return (
    <section className="space-y-10">
      <div className="flex flex-col gap-6 items-start">
        <div className="space-y-3">
          <p className="text-xs uppercase tracking-[0.25em] text-slate-400">
            最新リリース
          </p>
          <p className="text-3xl font-semibold text-slate-100">
            v{latestRelease.version}
          </p>
          <p className="text-sm text-slate-400">最終更新 {publishedAt}</p>
        </div>
        {primaryPlatform && (
          <DownloadButton
            platform={primaryPlatform.platform}
            url={primaryPlatform.url}
            isPrimary
          />
        )}
      </div>

      <div className="space-y-4">
        <p className="text-sm text-slate-300">
          {primaryPlatform
            ? "他のプラットフォームはこちら"
            : "ご利用の環境に合わせてダウンロードしてください"}
        </p>
        <div className="flex flex-wrap gap-3">
          {displayPlatforms.map(([platform, { url }]) => (
            <DownloadButton key={platform} platform={platform} url={url} />
          ))}
        </div>
      </div>

      {latestRelease.notes && (
        <div className="space-y-2 text-left text-sm leading-relaxed text-slate-400">
          <p className="text-xs uppercase tracking-[0.3em] text-slate-500">
            リリースノート
          </p>
          <div className="whitespace-pre-line">{latestRelease.notes}</div>
        </div>
      )}
    </section>
  );
}
