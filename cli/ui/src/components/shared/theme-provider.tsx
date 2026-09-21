"use client";

import {createContext, type ReactNode, useContext, useEffect, useState} from "react";
import { UiTheme } from "@/constants";



const ThemeContext = createContext({theme: UiTheme.Light, setTheme: (_theme: UiTheme) => {}});

export default function ThemeProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [theme, setLocalTheme] = useState<UiTheme>(UiTheme.System);

  const setTheme = (theme: UiTheme) => {
    localStorage.setItem("ui-theme", theme);
    updateDocumentTheme(theme);
    setLocalTheme(theme);
  }

  useEffect(() => {
    if(!document) return;

    setLocalTheme(localStorage.getItem("ui-theme") as UiTheme || UiTheme.System)

    window.addEventListener("storage", (event) => {
      if(event.key === "ui-theme" && event.newValue !== event.oldValue) {
        const new_theme = (event.newValue || UiTheme.Light) as UiTheme;
        updateDocumentTheme(new_theme);
        setLocalTheme(new_theme);
      }
    })
  }, []);

  return (
    <ThemeContext.Provider value={{theme, setTheme}}>{children}</ThemeContext.Provider>
  );
}

function updateDocumentTheme(theme: string) {
  if(theme === UiTheme.System) {
    theme = window.matchMedia("(prefers-color-scheme: dark)").matches ? UiTheme.Dark : UiTheme.Light;
  }
  document.body.parentElement?.setAttribute("data-theme", theme);
}

export function useTheme() {
  return useContext(ThemeContext)
}
