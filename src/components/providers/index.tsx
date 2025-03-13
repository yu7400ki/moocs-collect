import { JotaiProvider } from "./jotai";
import { TanstackRouterProvider } from "./tanstack-router";
import { ThemeProvider } from "./theme";
import { Updater } from "./updater";

export function Providers() {
  return (
    <JotaiProvider>
      <ThemeProvider>
        <TanstackRouterProvider />
        <Updater />
      </ThemeProvider>
    </JotaiProvider>
  );
}
