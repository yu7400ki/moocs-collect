import { penguin } from "@lucide/lab";
import { Apple, Download, Grid2X2, Icon } from "lucide-react";

interface DownloadButtonProps {
  platform: string;
  url: string;
  isPrimary?: boolean;
}

export function DownloadButton({
  platform,
  url,
  isPrimary = false,
}: DownloadButtonProps) {
  const baseClass =
    "inline-flex items-center gap-3 rounded-full px-5 py-2.5 text-sm font-medium transition focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2";
  const variantClass = isPrimary
    ? "bg-sky-400 text-slate-950 hover:bg-sky-300 focus-visible:outline-sky-200"
    : "border border-slate-600 text-slate-200 hover:border-slate-400 hover:text-slate-100 focus-visible:outline-slate-300";

  return (
    <a href={url} download className={`${baseClass} ${variantClass}`}>
      <PlatformIcon platform={platform} />
      <span>{getPlatformDisplayName(platform)}</span>
      <Download className="h-4 w-4" />
    </a>
  );
}

function PlatformIcon({ platform }: { platform: string }) {
  const iconClass = "h-5 w-5";

  switch (platform) {
    case "windows-x86_64":
      return <Grid2X2 className={iconClass} />;
    case "darwin-x86_64":
    case "darwin-aarch64":
      return <Apple className={iconClass} />;
    case "linux-x86_64":
      return <Icon className={iconClass} iconNode={penguin} />;
    default:
      return <Download className={iconClass} />;
  }
}

function getPlatformDisplayName(platform: string): string {
  const names: Record<string, string> = {
    "windows-x86_64": "Windows",
    "darwin-x86_64": "macOS (Intel)",
    "darwin-aarch64": "macOS (Apple Silicon)",
    "linux-x86_64": "Linux",
  };

  return names[platform] || platform;
}
