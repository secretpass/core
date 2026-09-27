"use client";

import {
  createContext,
  type ReactNode,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import { UiTheme } from "@/constants";

const ThemeContext = createContext({
  theme: UiTheme.System,
  true_theme: UiTheme.Light,
  setTheme: (_theme: UiTheme) => {},
});

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setLocalTheme] = useState<UiTheme>(UiTheme.System);

  const setTheme = (theme: UiTheme) => {
    localStorage.setItem("ui-theme", theme);
    updateDocumentTheme(theme);
    setLocalTheme(theme);
  };

  const true_theme = useMemo(() => {
    if (theme !== UiTheme.System || typeof document === "undefined") {
      return theme;
    }
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? UiTheme.Dark
      : UiTheme.Light;
  }, [theme]);

  useEffect(() => {
    if (!document) return;

    setLocalTheme(
      (localStorage.getItem("ui-theme") as UiTheme) || UiTheme.System,
    );

    window.addEventListener("storage", (event) => {
      if (event.key === "ui-theme" && event.newValue !== event.oldValue) {
        const new_theme = (event.newValue || UiTheme.Light) as UiTheme;
        updateDocumentTheme(new_theme);
        setLocalTheme(new_theme);
      }
    });
  }, []);

  return (
    <ThemeContext.Provider value={{ theme, true_theme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}

function updateDocumentTheme(theme: string) {
  if (theme === UiTheme.System) {
    theme = window.matchMedia("(prefers-color-scheme: dark)").matches
      ? UiTheme.Dark
      : UiTheme.Light;
  }
  document.documentElement?.setAttribute("data-theme", theme);
}

export function useTheme() {
  return useContext(ThemeContext);
}
