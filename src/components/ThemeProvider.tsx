import { createContext, useEffect, useState, type ReactNode } from "react";
import { listen } from "@tauri-apps/api/event";
import type { ThemeId, ThemeMeta } from "../types";

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

export const THEME_LIST: ThemeMeta[] = [
  { id: "ink-wash", name: "淡墨水彩", description: "水墨淡雅，温润如玉", gradient: "linear-gradient(135deg, #e8e3d8, #f8f5ef)" },
  { id: "morandi", name: "莫兰迪雅粉", description: "柔雅低饱和，静谧温柔", gradient: "linear-gradient(135deg, #e4ddd4, #f4efe8)" },
  { id: "palace", name: "赤金宫墙", description: "红墙金瓦，恢弘大气", gradient: "linear-gradient(135deg, #e0d8c8, #faf6f0)" },
];

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
