import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./app";

const root = document.getElementById("root");

if (root === null) {
  throw new Error("Root element not found");
}

createRoot(root).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
