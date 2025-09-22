"use client";

import { useEffect, useState } from "react";
import type { LatestRelease, Platform } from "../types/release";
import { type DetectedOS, detectUserOS } from "../utils/detect-os";
import { DownloadButton } from "./download-button";

type Props = {
  latestRelease: LatestRelease;
};

type FetchState<T> =
  | {
      state: "loading";
    }
  | {
      state: "fulfilled";
      data: T;
    }
  | {
      state: "rejected";
      error: unknown;
    };

export function DownloadSection({ latestRelease }: Props) {
  const [userOS, setUserOS] = useState<FetchState<DetectedOS | null>>({
    state: "loading",
  });

  useEffect(() => {
    if (userOS.state !== "loading") return;
    detectUserOS()
      .then((os) => {
        setUserOS({ state: "fulfilled", data: os });
      })
      .catch((error) => {
        setUserOS({ state: "rejected", error });
      });
  }, [userOS]);

  const primaryPlatform =
    userOS.state === "fulfilled" && userOS.data
      ? Object.entries(getPrimaryPlatformFromOS(latestRelease, userOS.data))
      : [];

  const displayPlatforms = Object.entries(latestRelease.platforms).filter(
    ([platform]) => !primaryPlatform.some(([p]) => p === platform),
  );

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
        {primaryPlatform.map(([platform, { url }]) => (
          <DownloadButton
            key={platform}
            platform={platform}
            url={url}
            isPrimary
          />
        ))}
      </div>

      <div className="space-y-4">
        <p className="text-sm text-slate-300">
          {primaryPlatform.length > 0
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

function getPrimaryPlatformFromOS(
  release: LatestRelease,
  os: DetectedOS,
): Record<string, Platform> {
  let platforms: [string, Platform][];
  switch (os.os) {
    case "Windows":
      platforms = Object.hasOwn(release.platforms, "windows-x86_64")
        ? Object.entries(release.platforms).filter(
            ([platform]) => platform === "windows-x86_64",
          )
        : [];
      break;
    case "macOS":
      if (os.arch === "arm") {
        platforms = Object.hasOwn(release.platforms, "darwin-aarch64")
          ? Object.entries(release.platforms).filter(
              ([platform]) => platform === "darwin-aarch64",
            )
          : [];
      } else if (os.arch === "x86") {
        platforms = Object.hasOwn(release.platforms, "darwin-x86_64")
          ? Object.entries(release.platforms).filter(
              ([platform]) => platform === "darwin-x86_64",
            )
          : [];
      } else {
        platforms = Object.entries(release.platforms).filter(([platform]) =>
          platform.startsWith("darwin"),
        );
      }
      break;
    case "Linux":
      platforms = Object.hasOwn(release.platforms, "linux-x86_64")
        ? Object.entries(release.platforms).filter(
            ([platform]) => platform === "linux-x86_64",
          )
        : [];
      break;
    default:
      platforms = [];
      break;
  }

  return Object.fromEntries(platforms);
}
