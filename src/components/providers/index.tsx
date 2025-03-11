import { JotaiProvider } from "./jotai";
import { TanstackRouterProvider } from "./tanstack-router";
import { ThemeProvider } from "./theme";

export function Providers() {
  return (
    <JotaiProvider>
      <ThemeProvider>
        <TanstackRouterProvider />
      </ThemeProvider>
    </JotaiProvider>
  );
}
