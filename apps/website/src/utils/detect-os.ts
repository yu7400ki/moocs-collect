export type DetectedOS =
  | {
      os: "Windows" | "macOS" | "Linux";
      arch: "arm" | "x86" | null;
    }
  | {
      os: null;
      arch: null;
    };

export async function detectUserOS(): Promise<DetectedOS | null> {
  if (typeof window === "undefined") return null;

  if (navigator?.userAgentData?.getHighEntropyValues) {
    const platform = navigator.userAgentData.platform;
    const hi = await navigator.userAgentData.getHighEntropyValues([
      "architecture",
    ]);
    if (/win/i.test(platform)) {
      return {
        os: "Windows",
        arch: hi.architecture === "x86" ? "x86" : null,
      };
    }
    if (/mac/i.test(platform)) {
      return {
        os: "macOS",
        arch:
          hi.architecture === "arm" || hi.architecture === "arm64"
            ? "arm"
            : hi.architecture === "x86" || hi.architecture === "x86_64"
              ? "x86"
              : null,
      };
    }
    if (/linux/i.test(platform)) {
      return {
        os: "Linux",
        arch: hi.architecture === "x86" ? "x86" : null,
      };
    }
  }

  const userAgent = window.navigator.userAgent;

  // Windows
  if (userAgent.includes("Windows")) {
    return {
      os: "Windows",
      arch: "x86",
    };
  }

  // macOS
  if (userAgent.includes("Mac")) {
    return {
      os: "macOS",
      arch: null,
    };
  }

  // Linux
  if (userAgent.includes("Linux")) {
    return {
      os: "Linux",
      arch: "x86",
    };
  }

  return {
    os: null,
    arch: null,
  };
}
