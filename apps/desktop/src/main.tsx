import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Providers } from "./components/providers";
import { store } from "./components/providers/jotai";
import { getSettings } from "./features/settings/services/settings";
import "./index.css";
import "unfonts.css";
import { settingsAtom } from "./features/settings/atoms/settings";

async function initSettings() {
  const settings = await getSettings();
  await store.set(settingsAtom, settings);
}

initSettings().then(() => {
  const rootElement = document.getElementById("root");
  if (rootElement && !rootElement.innerHTML) {
    const root = createRoot(rootElement);
    root.render(
      <StrictMode>
        <Providers />
      </StrictMode>,
    );
  }
});
