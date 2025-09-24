import { useAtomValue } from "jotai";
import { useLayoutEffect } from "react";
import { themeAtom } from "@/features/settings/atoms/theme";

function applyTheme(theme: "dark" | "light") {
  document.documentElement.classList.remove("dark", "light");
  document.documentElement.classList.add(theme);
  document.documentElement.style.setProperty("color-scheme", theme);
}

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const theme = useAtomValue(themeAtom);

  useLayoutEffect(() => {
    if (theme === "system") {
      const media = window.matchMedia("(prefers-color-scheme: dark)");
      applyTheme(media.matches ? "dark" : "light");
      const listener = (e: MediaQueryListEvent) => {
        applyTheme(e.matches ? "dark" : "light");
      };
      media.addEventListener("change", listener);
      return () => {
        media.removeEventListener("change", listener);
      };
    }
    applyTheme(theme);
  }, [theme]);

  return <>{children}</>;
}
