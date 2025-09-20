export function detectUserOS() {
  if (typeof window === "undefined") return null;

  const userAgent = window.navigator.userAgent;
  const platform = window.navigator.platform;

  // Windows
  if (userAgent.includes("Windows") || platform.includes("Win")) {
    return "windows-x86_64";
  }

  // macOS
  if (userAgent.includes("Mac") || platform.includes("Mac")) {
    // Check for Apple Silicon
    if (userAgent.includes("Intel")) {
      return "darwin-x86_64";
    }
    // Modern Macs are likely Apple Silicon
    return "darwin-aarch64";
  }

  // Linux
  if (userAgent.includes("Linux") || platform.includes("Linux")) {
    return "linux-x86_64";
  }

  return null;
}

export function getPlatformDisplayName(platform: string): string {
  const names: Record<string, string> = {
    "windows-x86_64": "Windows",
    "darwin-x86_64": "macOS (Intel)",
    "darwin-aarch64": "macOS (Apple Silicon)",
    "linux-x86_64": "Linux",
  };

  return names[platform] || platform;
}
