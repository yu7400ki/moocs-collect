import { JotaiProvider } from "./jotai";
import { TanstackRouterProvider } from "./tanstack-router";

export function Providers() {
  return (
    <JotaiProvider>
      <TanstackRouterProvider />
    </JotaiProvider>
  );
}
