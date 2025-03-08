import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Providers } from "./components/providers";
import "./index.css";
import "unfonts.css";

const rootElement = document.getElementById("root");
if (rootElement && !rootElement.innerHTML) {
  const root = createRoot(rootElement);
  root.render(
    <StrictMode>
      <Providers />
    </StrictMode>,
  );
}
