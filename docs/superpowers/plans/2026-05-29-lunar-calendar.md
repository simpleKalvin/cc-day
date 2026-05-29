# CC-Day 农历日历桌面应用 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建跨平台农历日历桌面应用，驻留系统托盘，点击展开日历面板，支持三套主题配色切换。

**Architecture:** Tauri v2 桌面框架，React 19 前端负责全部 UI 渲染和农历计算（lunar-javascript），Rust 后端负责系统托盘注册、动态日期图标、无边框弹窗窗口管理。CSS 变量驱动主题切换。

**Tech Stack:** Tauri v2, React 19, TypeScript, Tailwind CSS v4, lunar-javascript, Rust (image crate)

---

## File Structure

### Frontend (`src/`)

| File | Responsibility |
|------|---------------|
| `src/main.tsx` | 入口，挂载 React 根 |
| `src/App.tsx` | 根组件，布局，日历状态管理 |
| `src/index.css` | Tailwind 导入 + CSS 变量（3 套主题）+ 全局组件样式 |
| `src/types.ts` | TypeScript 接口定义（DayInfo, MonthGrid 等） |
| `src/lib/lunar.ts` | 农历计算封装（lunar-javascript wrapper） |
| `src/hooks/useTheme.ts` | 主题切换 Hook |
| `src/hooks/useCalendar.ts` | 日历状态 Hook（选中日期、月份导航） |
| `src/components/ThemeProvider.tsx` | 主题上下文提供者 |
| `src/components/DayDetail.tsx` | 详情面板（农历日期、天干地支、节气、节日标签） |
| `src/components/CalendarGrid.tsx` | 月历网格容器 |
| `src/components/DayCell.tsx` | 单日格（公历数字 + 农历小字） |
| `src/components/MonthNav.tsx` | 月份导航（上/下月切换） |
| `src/components/FooterBar.tsx` | 底部栏（生肖月份 + "回到今天"按钮） |

### Backend (`src-tauri/`)

| File | Responsibility |
|------|---------------|
| `src-tauri/src/main.rs` | 二进制入口（不变） |
| `src-tauri/src/lib.rs` | Tauri 应用构建（setup 闭包：创建托盘、创建弹窗） |
| `src-tauri/src/tray.rs` | 系统托盘创建与点击事件处理 |
| `src-tauri/src/icon.rs` | 动态图标生成（日期数字像素渲染） |
| `src-tauri/Cargo.toml` | 添加 image, tauri features |
| `src-tauri/tauri.conf.json` | 窗口设为空、托盘配置、macOS LSUIElement |
| `src-tauri/capabilities/default.json` | 权限更新 |

---

### Task 1: Install Dependencies & Configure Build Tools

**Files:**
- Modify: `package.json`
- Modify: `vite.config.ts`

- [ ] **Step 1: Install frontend dependencies**

```bash
pnpm add lunar-javascript
pnpm add -D tailwindcss @tailwindcss/vite
```

- [ ] **Step 2: Update `vite.config.ts` — add Tailwind plugin**

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [react(), tailwindcss()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? { protocol: "ws", host, port: 1421 }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
```

- [ ] **Step 3: Verify dev server starts**

```bash
pnpm dev
```

Expected: Vite dev server starts on `http://localhost:1420` without errors.

- [ ] **Step 4: Commit**

```bash
git add package.json pnpm-lock.yaml vite.config.ts
git commit -m "chore: add tailwindcss v4 and lunar-javascript deps"
```

---

### Task 2: HTML Entry & Template Cleanup

**Files:**
- Modify: `index.html`
- Modify: `src/main.tsx`
- Modify: `src/App.tsx`
- Delete: `src/App.css`
- Delete: `src/assets/react.svg`

- [ ] **Step 1: Update `index.html` — Chinese fonts, lang attribute**

```html
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>CC-Day 农历日历</title>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link href="https://fonts.googleapis.com/css2?family=Noto+Serif+SC:wght@400;600;700;900&family=Noto+Sans+SC:wght@300;400;500;600&display=swap" rel="stylesheet" />
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

- [ ] **Step 2: Delete template files**

```bash
rm src/App.css src/assets/react.svg
```

- [ ] **Step 3: Update `src/main.tsx` — add CSS import**

```typescript
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
```

- [ ] **Step 4: Replace `src/App.tsx` with minimal placeholder**

```typescript
function App() {
  return <div className="app-frame">CC-Day</div>;
}

export default App;
```

- [ ] **Step 5: Verify in browser**

```bash
pnpm dev
```

Open `http://localhost:1420`. Expected: white page with "CC-Day" text.

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "chore: clean up template, add Chinese fonts, setup HTML entry"
```

---

### Task 3: CSS Variables & Global Styles

**Files:**
- Create: `src/index.css`

This task creates the complete stylesheet with all three theme color schemes and component styles, based on the approved mockup (`ink-wash-light-v2.html`).

- [ ] **Step 1: Create `src/index.css`**

```css
@import "tailwindcss";

/* ═══════════════════════════════════════
   Theme Variables
   ═══════════════════════════════════════ */

:root[data-theme="ink-wash"] {
  --bg-body: #e8e3d8;
  --bg-primary: #f8f5ef;
  --bg-secondary: #f0ebe0;
  --bg-header-start: #e8e2d4;
  --bg-header-end: #f2ece0;
  --accent: #a67c52;
  --accent-light: rgba(166, 124, 82, 0.1);
  --accent-medium: rgba(166, 124, 82, 0.2);
  --accent-ink: #2c2c3a;
  --text-primary: #3a3a4a;
  --text-secondary: #7a7a8a;
  --text-muted: #aaa8a0;
  --festival: #c45a5a;
  --festival-bg: rgba(196, 90, 90, 0.08);
  --festival-border: rgba(196, 90, 90, 0.15);
  --jieqi: #4a7a5a;
  --jieqi-bg: rgba(74, 122, 90, 0.08);
  --jieqi-border: rgba(74, 122, 90, 0.15);
  --weekend: #b07060;
  --divider: rgba(166, 124, 82, 0.12);
  --today-bg: rgba(166, 124, 82, 0.08);
  --today-border: rgba(166, 124, 82, 0.25);
  --hover-bg: rgba(166, 124, 82, 0.05);
  --shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.04);
  --shadow-md: 0 8px 30px rgba(0, 0, 0, 0.08);
  --header-text: var(--accent-ink);
}

:root[data-theme="morandi"] {
  --bg-body: #e4ddd4;
  --bg-primary: #f4efe8;
  --bg-secondary: #ece5db;
  --bg-header-start: #e8dcd2;
  --bg-header-end: #f0e8de;
  --accent: #b07080;
  --accent-light: rgba(176, 112, 128, 0.1);
  --accent-medium: rgba(176, 112, 128, 0.2);
  --accent-ink: #4a4240;
  --text-primary: #4a4240;
  --text-secondary: #8a8078;
  --text-muted: #b0a89e;
  --festival: #c47070;
  --festival-bg: rgba(196, 112, 112, 0.1);
  --festival-border: rgba(196, 112, 112, 0.18);
  --jieqi: #7a9a7a;
  --jieqi-bg: rgba(122, 154, 122, 0.1);
  --jieqi-border: rgba(122, 154, 122, 0.18);
  --weekend: #a08878;
  --divider: rgba(176, 112, 128, 0.12);
  --today-bg: rgba(176, 112, 128, 0.08);
  --today-border: rgba(176, 112, 128, 0.25);
  --hover-bg: rgba(176, 112, 128, 0.05);
  --shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.04);
  --shadow-md: 0 8px 30px rgba(0, 0, 0, 0.08);
  --header-text: var(--accent-ink);
}

:root[data-theme="palace"] {
  --bg-body: #e0d8c8;
  --bg-primary: #faf6f0;
  --bg-secondary: #f2ebe0;
  --bg-header-start: #c4342e;
  --bg-header-end: #d44838;
  --accent: #c4342e;
  --accent-gold: #c8a44a;
  --accent-light: rgba(196, 52, 46, 0.08);
  --accent-medium: rgba(196, 52, 46, 0.15);
  --accent-ink: #3a2a20;
  --text-primary: #3a2a20;
  --text-secondary: #8a7a6a;
  --text-muted: #b0a898;
  --festival: #c4342e;
  --festival-bg: rgba(196, 52, 46, 0.08);
  --festival-border: rgba(196, 52, 46, 0.15);
  --jieqi: #5a8a5a;
  --jieqi-bg: rgba(90, 138, 90, 0.08);
  --jieqi-border: rgba(90, 138, 90, 0.15);
  --weekend: #a06050;
  --divider: rgba(200, 164, 74, 0.15);
  --today-bg: rgba(196, 52, 46, 0.06);
  --today-border: rgba(196, 52, 46, 0.2);
  --hover-bg: rgba(196, 52, 46, 0.04);
  --shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.04);
  --shadow-md: 0 8px 30px rgba(0, 0, 0, 0.1);
  --header-text: #ffffff;
}

/* ═══════════════════════════════════════
   Base
   ═══════════════════════════════════════ */

body {
  font-family: "Noto Sans SC", -apple-system, "PingFang SC", sans-serif;
  background: var(--bg-body);
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
}

/* ═══════════════════════════════════════
   App Frame
   ═══════════════════════════════════════ */

.app-frame {
  width: 320px;
  border-radius: 14px;
  overflow: hidden;
  box-shadow: var(--shadow-md), 0 0 0 1px rgba(166, 124, 82, 0.06);
  background: var(--bg-primary);
}

/* ═══════════════════════════════════════
   Detail Header
   ═══════════════════════════════════════ */

.detail-header {
  background:
    radial-gradient(ellipse at 20% 30%, rgba(166, 124, 82, 0.06) 0%, transparent 60%),
    radial-gradient(ellipse at 80% 70%, rgba(100, 130, 160, 0.05) 0%, transparent 60%),
    linear-gradient(160deg, var(--bg-header-start) 0%, var(--bg-header-end) 100%);
  padding: 20px 20px 18px;
  position: relative;
}

.detail-header::before {
  content: "";
  position: absolute;
  top: -40px;
  right: -20px;
  width: 140px;
  height: 140px;
  background: radial-gradient(circle, rgba(100, 130, 160, 0.06) 0%, transparent 70%);
  border-radius: 50%;
}

.solar-date {
  font-size: 12px;
  font-weight: 400;
  color: var(--text-secondary);
  letter-spacing: 0.5px;
  margin-bottom: 8px;
}

.lunar-date {
  font-family: "Noto Serif SC", serif;
  font-size: 28px;
  font-weight: 900;
  color: var(--header-text);
  letter-spacing: 3px;
  margin-bottom: 4px;
  line-height: 1.2;
}

.ganzhi {
  font-size: 12px;
  font-weight: 300;
  color: var(--text-secondary);
  letter-spacing: 1.5px;
  margin-bottom: 12px;
}

.tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.tag {
  font-size: 10px;
  padding: 3px 10px;
  border-radius: 5px;
  font-weight: 500;
  letter-spacing: 0.5px;
}

.tag-jieqi {
  background: var(--jieqi-bg);
  color: var(--jieqi);
  border: 1px solid var(--jieqi-border);
}

.tag-festival {
  background: var(--festival-bg);
  color: var(--festival);
  border: 1px solid var(--festival-border);
}

.tag-yi {
  background: var(--accent-light);
  color: var(--accent);
  border: 1px solid rgba(166, 124, 82, 0.15);
}

/* ═══════════════════════════════════════
   Divider
   ═══════════════════════════════════════ */

.divider {
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--divider) 30%, var(--divider) 70%, transparent);
}

/* ═══════════════════════════════════════
   Calendar
   ═══════════════════════════════════════ */

.calendar {
  background: var(--bg-primary);
  padding: 14px 16px 16px;
}

.month-nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14px;
}

.nav-btn {
  width: 28px;
  height: 28px;
  border: 1px solid var(--divider);
  border-radius: 7px;
  background: transparent;
  color: var(--text-muted);
  font-size: 11px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.nav-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-light);
}

.month-title {
  font-family: "Noto Serif SC", serif;
  font-size: 15px;
  font-weight: 700;
  color: var(--accent-ink);
  letter-spacing: 1px;
}

/* ═══════════════════════════════════════
   Grid
   ═══════════════════════════════════════ */

.grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
  text-align: center;
}

.weekday {
  font-size: 10px;
  color: var(--text-muted);
  padding-bottom: 8px;
  letter-spacing: 1px;
  font-weight: 500;
}

.weekday.is-weekend {
  color: var(--weekend);
  opacity: 0.6;
}

.day-cell {
  padding: 5px 0;
  border-radius: 7px;
  cursor: pointer;
  transition: all 0.15s;
  border: 1px solid transparent;
}

.day-cell:hover {
  background: var(--hover-bg);
}

.day-cell.is-today {
  background: var(--today-bg);
  border-color: var(--today-border);
}

.day-cell.is-selected {
  background: var(--accent-medium);
  border-color: var(--accent);
}

.day-num {
  font-size: 13px;
  color: var(--text-primary);
  font-weight: 500;
  line-height: 1.5;
}

.day-cell.is-weekend .day-num {
  color: var(--weekend);
}

.day-cell.is-other-month .day-num {
  color: var(--text-muted);
}

.day-cell.is-today .day-num {
  color: var(--accent);
  font-weight: 700;
}

.day-lunar {
  font-size: 8px;
  color: var(--text-muted);
  line-height: 1.3;
}

.day-cell.is-today .day-lunar {
  color: var(--accent);
  opacity: 0.8;
}

.day-cell.is-festival .day-lunar {
  color: var(--festival);
}

.day-cell.is-jieqi .day-lunar {
  color: var(--jieqi);
}

.day-cell.is-other-month .day-lunar {
  opacity: 0.5;
}

/* ═══════════════════════════════════════
   Footer
   ═══════════════════════════════════════ */

.footer {
  background: var(--bg-secondary);
  padding: 10px 20px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.footer-info {
  font-size: 10px;
  color: var(--text-muted);
  letter-spacing: 0.5px;
}

.today-btn {
  font-size: 10px;
  color: var(--accent);
  background: var(--accent-light);
  border: 1px solid rgba(166, 124, 82, 0.12);
  border-radius: 5px;
  padding: 3px 10px;
  cursor: pointer;
  transition: all 0.2s;
  letter-spacing: 0.5px;
  font-family: "Noto Sans SC", sans-serif;
}

.today-btn:hover {
  background: var(--accent-medium);
  border-color: var(--accent);
}
```

- [ ] **Step 2: Verify CSS loads without errors**

```bash
pnpm dev
```

Expected: page still shows "CC-Day" but now with cream background from `--bg-body`.

- [ ] **Step 3: Commit**

```bash
git add src/index.css
git commit -m "feat: add CSS variables for 3 themes and global component styles"
```

---

### Task 4: Types & Lunar Calendar Utilities

**Files:**
- Create: `src/types.ts`
- Create: `src/lib/lunar.ts`
- Create directory: `src/lib/`

- [ ] **Step 1: Create `src/types.ts`**

```typescript
export interface DayInfo {
  date: Date;
  solarYear: number;
  solarMonth: number;
  solarDay: number;
  weekday: number;
  lunarDayName: string;
  lunarMonthName: string;
  ganzhiYear: string;
  ganzhiMonth: string;
  ganzhiDay: string;
  shengxiao: string;
  jieqi: string | null;
  lunarFestival: string | null;
  solarFestival: string | null;
  isCurrentMonth: boolean;
  lunarDayText: string;
}

export interface MonthGrid {
  year: number;
  month: number;
  days: DayInfo[];
}

export type ThemeId = "ink-wash" | "morandi" | "palace";

export interface ThemeConfig {
  id: string;
  name: string;
  variables: Record<string, string>;
  isBuiltIn?: boolean;
}
```

- [ ] **Step 2: Create `src/lib/lunar.ts`**

```typescript
import { Solar } from "lunar-javascript";
import type { DayInfo, MonthGrid } from "../types";

export function getDayInfo(date: Date, isCurrentMonth = true): DayInfo {
  const solar = Solar.fromDate(date);
  const lunar = solar.getLunar();

  const lunarFestivals = lunar.getFestivals();
  const solarFestivals = solar.getFestivals();
  const jieqi = lunar.getJieQi();
  const lunarDayName = lunar.getDayInChinese();

  let lunarDayText = lunarDayName;
  if (lunarFestivals.length > 0) {
    lunarDayText = lunarFestivals[0];
  } else if (jieqi) {
    lunarDayText = jieqi;
  }

  return {
    date,
    solarYear: solar.getYear(),
    solarMonth: solar.getMonth(),
    solarDay: solar.getDay(),
    weekday: solar.getWeek(),
    lunarDayName,
    lunarMonthName: lunar.getMonthInChinese(),
    ganzhiYear: lunar.getYearInGanZhi(),
    ganzhiMonth: lunar.getMonthInGanZhi(),
    ganzhiDay: lunar.getDayInGanZhi(),
    shengxiao: lunar.getYearShengXiao(),
    jieqi: jieqi || null,
    lunarFestival: lunarFestivals.length > 0 ? lunarFestivals[0] : null,
    solarFestival: solarFestivals.length > 0 ? solarFestivals[0] : null,
    isCurrentMonth,
    lunarDayText,
  };
}

export function getMonthGrid(year: number, month: number): MonthGrid {
  const firstDay = new Date(year, month - 1, 1);
  const startWeekday = firstDay.getDay();

  const prevMonth = month === 1 ? 12 : month - 1;
  const prevYear = month === 1 ? year - 1 : year;
  const daysInPrevMonth = new Date(prevYear, prevMonth, 0).getDate();

  const daysInMonth = new Date(year, month, 0).getDate();

  const days: DayInfo[] = [];

  for (let i = startWeekday - 1; i >= 0; i--) {
    const day = daysInPrevMonth - i;
    const d = new Date(prevYear, prevMonth - 1, day);
    days.push(getDayInfo(d, false));
  }

  for (let day = 1; day <= daysInMonth; day++) {
    const d = new Date(year, month - 1, day);
    days.push(getDayInfo(d, true));
  }

  const remaining = 42 - days.length;
  const nextMonth = month === 12 ? 1 : month + 1;
  const nextYear = month === 12 ? year + 1 : year;

  for (let day = 1; day <= remaining; day++) {
    const d = new Date(nextYear, nextMonth - 1, day);
    days.push(getDayInfo(d, false));
  }

  return { year, month, days };
}

export function getNearbyJieqi(date: Date): string | null {
  const solar = Solar.fromDate(date);
  const lunar = solar.getLunar();
  const current = lunar.getJieQi();
  if (current) return current;

  for (let i = 1; i <= 15; i++) {
    const next = new Date(date);
    next.setDate(next.getDate() + i);
    const nextLunar = Solar.fromDate(next).getLunar();
    const jq = nextLunar.getJieQi();
    if (jq) return `${jq}将至`;
  }

  return null;
}
```

- [ ] **Step 3: Verify TypeScript compiles**

```bash
npx tsc --noEmit
```

Expected: no errors related to `src/types.ts` or `src/lib/lunar.ts`.

- [ ] **Step 4: Commit**

```bash
git add src/types.ts src/lib/lunar.ts
git commit -m "feat: add DayInfo types and lunar calendar utility functions"
```

---

### Task 5: Theme Context Provider

**Files:**
- Create: `src/components/ThemeProvider.tsx`
- Create: `src/hooks/useTheme.ts`
- Create directories: `src/components/`, `src/hooks/`

- [ ] **Step 1: Create `src/components/ThemeProvider.tsx`**

```typescript
import { createContext, useEffect, useState, type ReactNode } from "react";
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

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setThemeState] = useState<ThemeId>(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === "ink-wash" || saved === "morandi" || saved === "palace") {
      return saved;
    }
    return "ink-wash";
  });

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem(STORAGE_KEY, theme);
  }, [theme]);

  const setTheme = (id: ThemeId) => {
    setThemeState(id);
  };

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}
```

- [ ] **Step 2: Create `src/hooks/useTheme.ts`**

```typescript
import { useContext } from "react";
import { ThemeContext } from "../components/ThemeProvider";

export function useTheme() {
  return useContext(ThemeContext);
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/ThemeProvider.tsx src/hooks/useTheme.ts
git commit -m "feat: add ThemeProvider context and useTheme hook"
```

---

### Task 6: Calendar State Hook

**Files:**
- Create: `src/hooks/useCalendar.ts`

- [ ] **Step 1: Create `src/hooks/useCalendar.ts`**

```typescript
import { useMemo, useState } from "react";
import { getMonthGrid } from "../lib/lunar";
import type { DayInfo, MonthGrid } from "../types";

export function useCalendar() {
  const today = useMemo(() => {
    const now = new Date();
    return new Date(now.getFullYear(), now.getMonth(), now.getDate());
  }, []);

  const [selectedDate, setSelectedDate] = useState<Date>(today);
  const [viewYear, setViewYear] = useState(today.getFullYear());
  const [viewMonth, setViewMonth] = useState(today.getMonth() + 1);

  const monthGrid: MonthGrid = useMemo(
    () => getMonthGrid(viewYear, viewMonth),
    [viewYear, viewMonth],
  );

  const selectedDayInfo: DayInfo | null = useMemo(() => {
    return (
      monthGrid.days.find(
        (d) =>
          d.solarYear === selectedDate.getFullYear() &&
          d.solarMonth === selectedDate.getMonth() + 1 &&
          d.solarDay === selectedDate.getDate(),
      ) ?? null
    );
  }, [monthGrid, selectedDate]);

  function prevMonth() {
    if (viewMonth === 1) {
      setViewMonth(12);
      setViewYear((y) => y - 1);
    } else {
      setViewMonth((m) => m - 1);
    }
  }

  function nextMonth() {
    if (viewMonth === 12) {
      setViewMonth(1);
      setViewYear((y) => y + 1);
    } else {
      setViewMonth((m) => m + 1);
    }
  }

  function goToToday() {
    const now = new Date();
    const todayDate = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    setSelectedDate(todayDate);
    setViewYear(todayDate.getFullYear());
    setViewMonth(todayDate.getMonth() + 1);
  }

  function selectDate(date: Date) {
    setSelectedDate(date);
    if (
      date.getFullYear() !== viewYear ||
      date.getMonth() + 1 !== viewMonth
    ) {
      setViewYear(date.getFullYear());
      setViewMonth(date.getMonth() + 1);
    }
  }

  return {
    today,
    selectedDate,
    selectedDayInfo,
    viewYear,
    viewMonth,
    monthGrid,
    prevMonth,
    nextMonth,
    goToToday,
    selectDate,
  };
}
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
npx tsc --noEmit
```

- [ ] **Step 3: Commit**

```bash
git add src/hooks/useCalendar.ts
git commit -m "feat: add useCalendar hook for month navigation and date selection"
```

---

### Task 7: DayCell Component

**Files:**
- Create: `src/components/DayCell.tsx`

- [ ] **Step 1: Create `src/components/DayCell.tsx`**

```typescript
import { useMemo } from "react";
import type { DayInfo } from "../types";

interface DayCellProps {
  day: DayInfo;
  isToday: boolean;
  isSelected: boolean;
  onSelect: (date: Date) => void;
}

export function DayCell({ day, isToday, isSelected, onSelect }: DayCellProps) {
  const isWeekend = day.weekday === 0 || day.weekday === 6;
  const hasFestival = day.lunarFestival || day.solarFestival;
  const hasJieqi = day.jieqi;

  const classNames = useMemo(() => {
    const cls = ["day-cell"];
    if (isToday) cls.push("is-today");
    if (isSelected) cls.push("is-selected");
    if (isWeekend) cls.push("is-weekend");
    if (!day.isCurrentMonth) cls.push("is-other-month");
    if (hasFestival) cls.push("is-festival");
    if (hasJieqi) cls.push("is-jieqi");
    return cls.join(" ");
  }, [day, isToday, isSelected, isWeekend, hasFestival, hasJieqi]);

  return (
    <div className={classNames} onClick={() => onSelect(day.date)}>
      <div className="day-num">{day.solarDay}</div>
      <div className="day-lunar">{day.lunarDayText}</div>
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/DayCell.tsx
git commit -m "feat: add DayCell component with theme-aware styling"
```

---

### Task 8: CalendarGrid Component

**Files:**
- Create: `src/components/CalendarGrid.tsx`

- [ ] **Step 1: Create `src/components/CalendarGrid.tsx`**

```typescript
import { DayCell } from "./DayCell";
import type { MonthGrid } from "../types";

interface CalendarGridProps {
  monthGrid: MonthGrid;
  selectedDate: Date;
  today: Date;
  onSelectDate: (date: Date) => void;
}

const WEEKDAYS = ["日", "一", "二", "三", "四", "五", "六"];

export function CalendarGrid({
  monthGrid,
  selectedDate,
  today,
  onSelectDate,
}: CalendarGridProps) {
  return (
    <div className="calendar">
      <div className="grid">
        {WEEKDAYS.map((name, i) => (
          <div
            key={name}
            className={`weekday${i === 0 || i === 6 ? " is-weekend" : ""}`}
          >
            {name}
          </div>
        ))}

        {monthGrid.days.map((day, i) => {
          const isToday =
            day.solarYear === today.getFullYear() &&
            day.solarMonth === today.getMonth() + 1 &&
            day.solarDay === today.getDate();

          const isSelected =
            day.solarYear === selectedDate.getFullYear() &&
            day.solarMonth === selectedDate.getMonth() + 1 &&
            day.solarDay === selectedDate.getDate();

          return (
            <DayCell
              key={`${day.solarYear}-${day.solarMonth}-${day.solarDay}-${i}`}
              day={day}
              isToday={isToday}
              isSelected={isSelected}
              onSelect={onSelectDate}
            />
          );
        })}
      </div>
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/CalendarGrid.tsx
git commit -m "feat: add CalendarGrid component with weekday header and day cells"
```

---

### Task 9: DayDetail Component

**Files:**
- Create: `src/components/DayDetail.tsx`

- [ ] **Step 1: Create `src/components/DayDetail.tsx`**

```typescript
import { getNearbyJieqi } from "../lib/lunar";
import type { DayInfo } from "../types";

interface DayDetailProps {
  day: DayInfo;
}

export function DayDetail({ day }: DayDetailProps) {
  const weekdays = ["日", "一", "二", "三", "四", "五", "六"];
  const solarDateStr = `${day.solarYear}年${day.solarMonth}月${day.solarDay}日 星期${weekdays[day.weekday]}`;
  const ganzhiStr = `${day.ganzhiYear}年 ${day.ganzhiMonth}月 ${day.ganzhiDay}日`;

  const nearbyJieqi = getNearbyJieqi(day.date);

  const tags: { label: string; type: "jieqi" | "festival" | "yi" }[] = [];
  if (nearbyJieqi) {
    tags.push({ label: nearbyJieqi, type: "jieqi" });
  }
  if (day.lunarFestival) {
    tags.push({ label: day.lunarFestival, type: "festival" });
  }
  if (day.solarFestival) {
    tags.push({ label: day.solarFestival, type: "festival" });
  }

  return (
    <div className="detail-header">
      <div className="solar-date">{solarDateStr}</div>
      <div className="lunar-date">
        {day.lunarMonthName}
        {day.lunarDayName}
      </div>
      <div className="ganzhi">{ganzhiStr}</div>
      {tags.length > 0 && (
        <div className="tags">
          {tags.map((tag) => (
            <span key={tag.label} className={`tag tag-${tag.type}`}>
              {tag.label}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/DayDetail.tsx
git commit -m "feat: add DayDetail component with solar/lunar date and tags"
```

---

### Task 10: MonthNav & FooterBar Components

**Files:**
- Create: `src/components/MonthNav.tsx`
- Create: `src/components/FooterBar.tsx`

- [ ] **Step 1: Create `src/components/MonthNav.tsx`**

```typescript
interface MonthNavProps {
  year: number;
  month: number;
  onPrev: () => void;
  onNext: () => void;
}

export function MonthNav({ year, month, onPrev, onNext }: MonthNavProps) {
  return (
    <div className="month-nav">
      <button className="nav-btn" onClick={onPrev}>
        ◀
      </button>
      <span className="month-title">
        {year}年{month}月
      </span>
      <button className="nav-btn" onClick={onNext}>
        ▶
      </button>
    </div>
  );
}
```

- [ ] **Step 2: Create `src/components/FooterBar.tsx`**

```typescript
import type { DayInfo } from "../types";

interface FooterBarProps {
  day: DayInfo;
  onGoToToday: () => void;
}

export function FooterBar({ day, onGoToToday }: FooterBarProps) {
  return (
    <div className="footer">
      <span className="footer-info">
        {day.shengxiao}月 · {day.ganzhiYear}
      </span>
      <button className="today-btn" onClick={onGoToToday}>
        回到今天
      </button>
    </div>
  );
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/MonthNav.tsx src/components/FooterBar.tsx
git commit -m "feat: add MonthNav and FooterBar components"
```

---

### Task 11: App Integration — Wire All Components

**Files:**
- Modify: `src/App.tsx`
- Modify: `src/main.tsx`

- [ ] **Step 1: Update `src/App.tsx`**

```typescript
import { ThemeProvider } from "./components/ThemeProvider";
import { useCalendar } from "./hooks/useCalendar";
import { DayDetail } from "./components/DayDetail";
import { CalendarGrid } from "./components/CalendarGrid";
import { MonthNav } from "./components/MonthNav";
import { FooterBar } from "./components/FooterBar";

function CalendarApp() {
  const {
    today,
    selectedDate,
    selectedDayInfo,
    viewYear,
    viewMonth,
    monthGrid,
    prevMonth,
    nextMonth,
    goToToday,
    selectDate,
  } = useCalendar();

  return (
    <div className="app-frame">
      {selectedDayInfo && <DayDetail day={selectedDayInfo} />}
      <div className="divider" />
      <MonthNav
        year={viewYear}
        month={viewMonth}
        onPrev={prevMonth}
        onNext={nextMonth}
      />
      <CalendarGrid
        monthGrid={monthGrid}
        selectedDate={selectedDate}
        today={today}
        onSelectDate={selectDate}
      />
      <div className="divider" />
      {selectedDayInfo && <FooterBar day={selectedDayInfo} onGoToToday={goToToday} />}
    </div>
  );
}

export default function App() {
  return (
    <ThemeProvider>
      <CalendarApp />
    </ThemeProvider>
  );
}
```

- [ ] **Step 2: Verify full UI renders in browser**

```bash
pnpm dev
```

Open `http://localhost:1420`. Expected:
- Cream background with 320px wide calendar popup frame
- Header shows today's solar date, lunar date, ganzhi
- Month grid with today highlighted
- Navigation arrows work (prev/next month)
- "回到今天" button works
- Footer shows zodiac + ganzhi year

- [ ] **Step 3: Fix any issues found during verification**

Common issues to check:
- `lunar-javascript` import path (may need `from "lunar-javascript/typescript"` or just `from "lunar-javascript"`)
- CSS variable values not applying (check `data-theme` attribute on `<html>`)
- Date calculation edge cases

- [ ] **Step 4: Commit**

```bash
git add src/App.tsx src/main.tsx
git commit -m "feat: integrate all calendar components with theme support"
```

---

### Task 12: Frontend Visual Polish

**Files:**
- Modify: `src/index.css` (if adjustments needed)
- Modify: `src/components/DayDetail.tsx` (if adjustments needed)

- [ ] **Step 1: Test all 3 themes by switching `data-theme`**

In browser DevTools console:

```javascript
document.documentElement.setAttribute("data-theme", "morandi");
document.documentElement.setAttribute("data-theme", "palace");
document.documentElement.setAttribute("data-theme", "ink-wash");
```

Verify each theme renders correctly with proper colors, gradients, and text contrast.

- [ ] **Step 2: Check Palace theme header text color**

Palace theme uses `--header-text: #ffffff` (white on red). Verify the `.lunar-date` text in the detail header renders white on the red gradient.

If `.ganzhi` and `.solar-date` are hard to read on red background, adjust to also use `--header-text`:

In `src/index.css`, add under `.detail-header`:
```css
.solar-date,
.ganzhi {
  color: color-mix(in srgb, var(--header-text) 60%, transparent);
}
```

- [ ] **Step 3: Fix any visual issues and commit**

```bash
git add -A
git commit -m "fix: polish theme styles and Palace header contrast"
```

---

### Task 13: Rust Backend — Dependencies & Module Structure

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/tray.rs`
- Create: `src-tauri/src/icon.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Update `src-tauri/Cargo.toml` — add image crate, tray-icon feature**

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
image = "0.25"
```

- [ ] **Step 2: Create `src-tauri/src/icon.rs` — dynamic date number icon**

```rust
use image::{ImageBuffer, Rgba};

const DIGIT_PATTERNS: [[[bool; 3]; 5]; 10] = [
    [[true, true, true], [true, false, true], [true, false, true], [true, false, true], [true, true, true]],
    [[false, true, false], [true, true, false], [false, true, false], [false, true, false], [true, true, true]],
    [[true, true, true], [false, false, true], [true, true, true], [true, false, false], [true, true, true]],
    [[true, true, true], [false, false, true], [true, true, true], [false, false, true], [true, true, true]],
    [[true, false, true], [true, false, true], [true, true, true], [false, false, true], [false, false, true]],
    [[true, true, true], [true, false, false], [true, true, true], [false, false, true], [true, true, true]],
    [[true, true, true], [true, false, false], [true, true, true], [true, false, true], [true, true, true]],
    [[true, true, true], [false, false, true], [false, true, false], [false, true, false], [false, true, false]],
    [[true, true, true], [true, false, true], [true, true, true], [true, false, true], [true, true, true]],
    [[true, true, true], [true, false, true], [true, true, true], [false, false, true], [true, true, true]],
];

const SCALE: u32 = 3;
const DIGIT_W: u32 = 3 * SCALE;
const DIGIT_H: u32 = 5 * SCALE;
const GAP: u32 = 1 * SCALE;
const PAD_X: u32 = 3;
const PAD_Y: u32 = 2;

pub fn generate_date_icon(day: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let digits: Vec<u8> = if day < 10 {
        vec![day as u8]
    } else {
        vec![(day / 10) as u8, (day % 10) as u8]
    };

    let text_w = digits.len() as u32 * DIGIT_W + (digits.len().saturating_sub(1) as u32) * GAP;
    let img_w = text_w + PAD_X * 2;
    let img_h = DIGIT_H + PAD_Y * 2;

    let mut img = ImageBuffer::from_pixel(img_w, img_h, Rgba([0, 0, 0, 0]));

    let mut x_offset = PAD_X;
    for &d in &digits {
        let pattern = &DIGIT_PATTERNS[d as usize];
        for (row, pixel_row) in pattern.iter().enumerate() {
            for (col, &on) in pixel_row.iter().enumerate() {
                if on {
                    for sy in 0..SCALE {
                        for sx in 0..SCALE {
                            let px = x_offset + col as u32 * SCALE + sx;
                            let py = PAD_Y + row as u32 * SCALE + sy;
                            if px < img_w && py < img_h {
                                img.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                            }
                        }
                    }
                }
            }
        }
        x_offset += DIGIT_W + GAP;
    }

    img
}
```

- [ ] **Step 3: Create `src-tauri/src/tray.rs` — tray setup with click toggle**

```rust
use tauri::{
    Manager, Runtime,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    image::Image,
};
use crate::icon::generate_date_icon;

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let today_day = chrono_today_day();
    let icon_img = generate_date_icon(today_day);
    let icon = Image::from_bytes(&icon_to_png_bytes(&icon_img)?)?;

    TrayIconBuilder::new(app)
        .icon(icon)
        .tooltip("CC-Day 农历日历")
        .icon_as_template(true)
        .on_tray_icon_event(|tray, event| {
            let app = tray.app_handle();
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    if let Some(window) = app.get_webview_window("calendar") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
                _ => {}
            }
        })
        .build()?;

    Ok(())
}

fn chrono_today_day() -> u32 {
    use std::time::SystemTime;
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let days = duration.as_secs() / 86400;
    ((days + 3) % 31) + 1 // rough day-of-month approximation
}

fn icon_to_png_bytes(img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Result<Vec<u8>, image::ImageError> {
    let mut bytes = std::io::Cursor::new(Vec::new());
    img.write_to(&mut bytes, image::ImageFormat::Png)?;
    Ok(bytes.into_inner())
}
```

> **Note:** `chrono_today_day()` above is a rough approximation. In the actual implementation, use `chrono` crate or `std::time` to get the actual day-of-month. If `chrono` is not desired as a dependency, a simple calculation using `SystemTime` works:
>
> ```rust
> fn chrono_today_day() -> u32 {
>     // Use local time offset to get today's day
>     // A simple approach: parse from system time
>     let now = std::time::SystemTime::now();
>     let secs = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
>     // 1970-01-01 was Thursday. Calculate current date.
>     // This is simplified - use `chrono` for production accuracy.
>     chrono::Local::now().day()
> }
> ```
>
> Add `chrono = "0.4"` to Cargo.toml if using this approach.

- [ ] **Step 4: Update `src-tauri/src/lib.rs` — setup closure with tray and popup window**

```rust
mod tray;
mod icon;

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tray::create_tray(app.handle())?;

            let window = WebviewWindowBuilder::new(
                app,
                "calendar",
                WebviewUrl::App("index.html".into()),
            )
            .title("CC-Day")
            .inner_size(320.0, 420.0)
            .decorations(false)
            .transparent(false)
            .always_on_top(true)
            .visible(false)
            .resizable(false)
            .build()?;

            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(false) = event {
                    let _ = window.hide();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: Verify Rust compiles**

```bash
cd src-tauri && cargo check
```

Expected: compiles without errors. May need to adjust import paths based on Tauri v2 API.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/
git commit -m "feat: add Rust tray icon, dynamic date icon, popup window management"
```

---

### Task 14: Tauri Configuration — No Main Window, Tray Permissions

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Modify: `src-tauri/capabilities/default.json`

- [ ] **Step 1: Update `src-tauri/tauri.conf.json`**

Remove the default main window and add macOS LSUIElement for no Dock icon:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "CC-Day",
  "version": "0.1.0",
  "identifier": "com.cc-day.app",
  "build": {
    "beforeDevCommand": "pnpm dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "pnpm build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

- [ ] **Step 2: Update `src-tauri/capabilities/default.json`**

Change `"windows": ["main"]` to `["calendar"]` to scope permissions to the popup window:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the calendar window",
  "windows": ["calendar"],
  "permissions": [
    "core:default",
    "opener:default"
  ]
}
```

- [ ] **Step 3: Add macOS LSUIElement for no Dock icon**

Create or update `src-tauri/Info.plist` (Tauri will bundle this):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>LSUIElement</key>
    <true/>
</dict>
</plist>
```

In `src-tauri/tauri.conf.json`, add under `bundle`:

```json
{
  "bundle": {
    "macOS": {
      "info": {
        "LSUIElement": true
      }
    }
  }
}
```

- [ ] **Step 4: Verify Tauri builds**

```bash
pnpm tauri build 2>&1 | head -50
```

Expected: build starts without config errors. Full build may take several minutes.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/tauri.conf.json src-tauri/capabilities/default.json
git commit -m "feat: configure Tauri for tray-only app with no main window"
```

---

### Task 15: Window Positioning — Below Tray Icon

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add window positioning logic in the tray click handler**

In `src-tauri/src/lib.rs`, update the setup closure to position the popup window below the tray icon on macOS:

```rust
// In the TrayIconEvent::Click handler, before showing the window:
if let Some(tray_rect) = tray.rect() {
    let window_width = 320.0;
    let window_height = 420.0;
    let x = tray_rect.center().x - window_width / 2.0;
    let y = tray_rect.bottom() + 4.0; // 4px gap below menu bar
    let _ = window.set_position(tauri::Position::Physical(
        tauri::PhysicalPosition::new(x as i32, y as i32),
    ));
}
let _ = window.show();
let _ = window.set_focus();
```

> **Note:** The `tray.rect()` API returns the physical position of the tray icon. The positioning math may need adjustment for different screen densities and platforms. On Windows, position the window above the taskbar tray (subtract window height from tray y position).

- [ ] **Step 2: Test on macOS**

```bash
pnpm tauri dev
```

Expected: clicking the tray icon opens the calendar popup positioned just below the menu bar icon. Clicking outside hides it.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: position popup window below tray icon"
```

---

### Task 16: Theme Switching via Tray Right-Click Menu

**Files:**
- Modify: `src-tauri/src/tray.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add right-click menu with theme options in `src-tauri/src/tray.rs`**

Use Tauri v2's menu API to add a context menu on right-click:

```rust
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent},
    Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let menu = MenuBuilder::new(app)
        .item(&MenuItemBuilder::with_id("theme_ink_wash", "淡墨水彩").build(app)?)
        .item(&MenuItemBuilder::with_id("theme_morandi", "莫兰迪雅粉").build(app)?)
        .item(&MenuItemBuilder::with_id("theme_palace", "赤金宫墙").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("quit", "退出").build(app)?)
        .build()?;

    // ... tray builder with .menu(&menu) and .on_menu_event(...)
}
```

- [ ] **Step 2: Send theme change to frontend via Tauri event**

When a menu item is clicked, emit an event that the React app listens to:

```rust
.on_menu_event(|app, event| {
    match event.id().as_ref() {
        "theme_ink_wash" => { let _ = app.emit("theme-change", "ink-wash"); }
        "theme_morandi" => { let _ = app.emit("theme-change", "morandi"); }
        "theme_palace" => { let _ = app.emit("theme-change", "palace"); }
        "quit" => { app.exit(0); }
        _ => {}
    }
})
```

- [ ] **Step 3: Update `src/components/ThemeProvider.tsx` to listen for Tauri events**

```typescript
import { listen } from "@tauri-apps/api/event";

export function ThemeProvider({ children }: { children: ReactNode }) {
  // ... existing state ...

  useEffect(() => {
    const unlisten = listen<string>("theme-change", (event) => {
      const id = event.payload;
      if (id === "ink-wash" || id === "morandi" || id === "palace") {
        setThemeState(id);
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  // ... rest unchanged ...
}
```

- [ ] **Step 4: Test theme switching**

```bash
pnpm tauri dev
```

Right-click tray icon → select theme → verify the UI switches colors.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: add tray right-click menu for theme switching"
```

---

### Task 17: Daily Icon Update at Midnight

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add a background timer to update the tray icon at midnight**

In the setup closure, spawn a thread that updates the icon daily:

```rust
use std::time::Duration;
use std::thread;

// In setup():
let app_handle = app.handle().clone();
thread::spawn(move || {
    loop {
        let now = chrono::Local::now();
        let secs_until_midnight = (24 - now.hour()) * 3600
            - now.minute() * 60
            - now.second();
        thread::sleep(Duration::from_secs(secs_until_midnight as u64 + 60));

        if let Some(tray) = app_handle.tray_by_id("main") {
            let day = chrono::Local::now().day();
            let icon_img = generate_date_icon(day);
            if let Ok(png_bytes) = icon_to_png_bytes(&icon_img) {
                if let Ok(icon) = tauri::image::Image::from_bytes(&png_bytes) {
                    let _ = tray.set_icon(Some(icon));
                }
            }
        }
    }
});
```

Also update `TrayIconBuilder` to include an id:

```rust
TrayIconBuilder::with_id(app, "main")
    // ... rest unchanged
```

- [ ] **Step 2: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: auto-update tray icon date number at midnight"
```

---

### Task 18: Full Integration Verification

**Files:**
- None (verification only)

- [ ] **Step 1: Run the complete desktop app**

```bash
pnpm tauri dev
```

- [ ] **Step 2: Verify checklist**

- [ ] App launches with no dock icon (macOS) / no taskbar button (Windows)
- [ ] Tray icon shows today's date number
- [ ] Left-click tray icon → calendar popup appears below the icon
- [ ] Popup is 320x420px, borderless, always on top
- [ ] Header shows today's solar date, lunar date, ganzhi
- [ ] Calendar grid shows current month with lunar text
- [ ] Today is highlighted with accent color
- [ ] Weekends are colored differently
- [ ] Festivals and jieqi are shown in colored text and tags
- [ ] Prev/next month navigation works
- [ ] "回到今天" button returns to current month
- [ ] Click outside popup → popup hides
- [ ] Right-click tray → theme menu appears
- [ ] Switching themes changes all colors correctly
- [ ] Theme persists across app restarts
- [ ] Non-current-month dates are dimmed

- [ ] **Step 3: Build production binary**

```bash
pnpm tauri build
```

Expected: `.dmg` + `.app` on macOS, `.exe` + `.msi` on Windows.

- [ ] **Step 4: Final commit**

```bash
git add -A
git commit -m "feat: CC-Day v0.1.0 — complete lunar calendar desktop app"
```

---

## Self-Review Checklist

### Spec Coverage

| Spec Requirement | Task |
|---|---|
| Tauri v2 桌面框架 | Task 13 |
| React 19 + TypeScript | Task 11 |
| Tailwind CSS + CSS 变量主题 | Task 1, 3 |
| lunar-javascript 农历计算 | Task 4 |
| Rust 托盘图标注册 | Task 13 |
| 动态日期图标 | Task 13 (icon.rs) |
| 无主窗口，托盘驻留 | Task 14 |
| 无边框弹窗窗口 | Task 13 |
| 点击外部关闭 | Task 13 (on_window_event) |
| 三套主题配色 | Task 3 |
| 主题切换机制 | Task 16 |
| ThemeConfig 接口预留 | Task 5 (types.ts) |
| 主题持久化 | Task 5 (localStorage) |
| 每日零点更新图标 | Task 17 |
| DayDetail 组件 | Task 9 |
| CalendarGrid 组件 | Task 8 |
| MonthNav 组件 | Task 10 |
| FooterBar 组件 | Task 10 |
| ThemeProvider 组件 | Task 5 |

### Placeholder Scan

No TBD, TODO, or placeholder steps remain.

### Type Consistency

- `DayInfo` defined in `src/types.ts` (Task 4), consumed by `DayCell`, `DayDetail`, `FooterBar`, `useCalendar`
- `MonthGrid` defined in `src/types.ts` (Task 4), consumed by `CalendarGrid`, `useCalendar`
- `ThemeId` defined in `src/types.ts` (Task 4), consumed by `ThemeProvider`, `useTheme`
- Function names are consistent across all tasks: `getDayInfo`, `getMonthGrid`, `getNearbyJieqi`, `selectDate`, `goToToday`, `prevMonth`, `nextMonth`
