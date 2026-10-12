import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import { ThemeProvider } from "@/features/theme";
import { App } from "./app";
import { AnalyticsProvider } from "./features/analytics/provider";

createRoot(document.getElementById("root") || document.body).render(
  <StrictMode>
    <AnalyticsProvider>
      <ThemeProvider>
        <App />
      </ThemeProvider>
    </AnalyticsProvider>
  </StrictMode>,
);
