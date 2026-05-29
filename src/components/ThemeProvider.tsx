import { createContext, useEffect, useState, type ReactNode } from "react";
import { listen } from "@tauri-apps/api/event";
import type { ThemeId } from "../types";

interface ThemeContextValue {
  theme: ThemeId;
  setTheme: (id: ThemeId) => void;
}

export const ThemeContext = createContext<ThemeContextValue>({
  theme: "ink-wash",
  setTheme: () => {},
});

const STORAGE_KEY = "cc-day-theme";

const VALID_THEMES: ThemeId[] = ["ink-wash", "morandi", "palace"];

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setThemeState] = useState<ThemeId>(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved && VALID_THEMES.includes(saved as ThemeId)) {
      return saved as ThemeId;
    }
    return "ink-wash";
  });

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem(STORAGE_KEY, theme);
  }, [theme]);

  useEffect(() => {
    const unlisten = listen<string>("theme-change", (event) => {
      if (VALID_THEMES.includes(event.payload as ThemeId)) {
        setThemeState(event.payload as ThemeId);
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const setTheme = (id: ThemeId) => {
    setThemeState(id);
  };

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}
