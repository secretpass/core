import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import { ThemeProvider } from "@/features/theme";
import { App } from "./app";

createRoot(document.getElementById("root") || document.body).render(
  <StrictMode>
    <ThemeProvider>
      <App />
    </ThemeProvider>
  </StrictMode>,
);
