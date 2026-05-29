# UI 改进实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 为 CC-Day 农历日历弹窗引入页内导航（设置页/关于页）、改造托盘菜单、温和放大字体。

**Architecture:** 前端用一个 `page` state 切换日历/设置/关于三个页面。新增 `NavBar` 通用组件、`SettingsPage`、`AboutPage`。Rust 端托盘菜单改为偏好/版本/退出，通过 Tauri event 通知前端切换页面。新增 `get_app_version` command 获取版本号。

**Tech Stack:** React 19, TypeScript, Tauri v2, Rust, CSS (vanilla)

---

### Task 1: 更新类型定义

**Files:**
- Modify: `src/types.ts`

- [ ] **Step 1: 新增 PageId 类型**

在 `src/types.ts` 末尾添加：

```ts
export type PageId = "calendar" | "settings" | "about";
```

- [ ] **Step 2: 新增主题元数据类型**

在 `src/types.ts` 末尾继续添加：

```ts
export interface ThemeMeta {
  id: ThemeId;
  name: string;
  description: string;
  gradient: string;
}
```

- [ ] **Step 3: Commit**

```bash
git add src/types.ts
git commit -m "feat: add PageId and ThemeMeta types"
```

---

### Task 2: 导出主题元数据

**Files:**
- Modify: `src/components/ThemeProvider.tsx`

- [ ] **Step 1: 在 ThemeProvider.tsx 中添加主题元数据常量和导出**

在 `VALID_THEMES` 常量后面添加：

```ts
import type { ThemeMeta } from "../types";

export const THEME_LIST: ThemeMeta[] = [
  { id: "ink-wash", name: "淡墨水彩", description: "水墨淡雅，温润如玉", gradient: "linear-gradient(135deg, #e8e3d8, #f8f5ef)" },
  { id: "morandi", name: "莫兰迪雅粉", description: "柔雅低饱和，静谧温柔", gradient: "linear-gradient(135deg, #e4ddd4, #f4efe8)" },
  { id: "palace", name: "赤金宫墙", description: "红墙金瓦，恢弘大气", gradient: "linear-gradient(135deg, #e0d8c8, #faf6f0)" },
];
```

- [ ] **Step 2: Commit**

```bash
git add src/components/ThemeProvider.tsx
git commit -m "feat: export theme metadata list"
```

---

### Task 3: 创建 NavBar 组件

**Files:**
- Create: `src/components/NavBar.tsx`

- [ ] **Step 1: 创建 NavBar.tsx**

```tsx
interface NavBarProps {
  title: string;
  onBack: () => void;
}

export function NavBar({ title, onBack }: NavBarProps) {
  return (
    <div className="nav-bar">
      <button className="nav-back" onClick={onBack}>
        ← 返回
      </button>
      <span className="nav-title">{title}</span>
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/NavBar.tsx
git commit -m "feat: add NavBar component"
```

---

### Task 4: 创建 SettingsPage 组件

**Files:**
- Create: `src/components/SettingsPage.tsx`

- [ ] **Step 1: 创建 SettingsPage.tsx**

```tsx
import { NavBar } from "./NavBar";
import { THEME_LIST } from "./ThemeProvider";
import { useTheme } from "../hooks/useTheme";

interface SettingsPageProps {
  onBack: () => void;
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const { theme, setTheme } = useTheme();

  return (
    <div className="settings-page">
      <NavBar title="偏好设置" onBack={onBack} />
      <div className="settings-content">
        <div className="settings-label">选择主题</div>
        <div className="theme-list">
          {THEME_LIST.map((t) => {
            const isActive = theme === t.id;
            return (
              <div
                key={t.id}
                className={`theme-card${isActive ? " is-active" : ""}`}
                onClick={() => setTheme(t.id)}
              >
                <div
                  className="theme-preview"
                  style={{ background: t.gradient }}
                />
                <div className="theme-info">
                  <div className="theme-name">{t.name}</div>
                  <div className="theme-desc">{t.description}</div>
                </div>
                {isActive && (
                  <div className="theme-check">
                    ✓
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/SettingsPage.tsx
git commit -m "feat: add SettingsPage with theme switching"
```

---

### Task 5: 创建 AboutPage 组件

**Files:**
- Create: `src/components/AboutPage.tsx`

- [ ] **Step 1: 创建 AboutPage.tsx**

```tsx
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { NavBar } from "./NavBar";

interface AboutPageProps {
  onBack: () => void;
}

export function AboutPage({ onBack }: AboutPageProps) {
  const [version, setVersion] = useState("...");

  useEffect(() => {
    invoke<string>("get_app_version").then(setVersion).catch(() => setVersion("unknown"));
  }, []);

  return (
    <div className="about-page">
      <NavBar title="关于" onBack={onBack} />
      <div className="about-content">
        <div className="about-icon">📅</div>
        <div className="about-name">CC-Day</div>
        <div className="about-version">版本 {version}</div>
        <div className="about-divider" />
        <div className="about-desc">
          一款简洁优雅的农历日历<br />托盘常驻，随时查看
        </div>
        <div className="about-copyright">© 2026 CC-Day</div>
      </div>
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/AboutPage.tsx
git commit -m "feat: add AboutPage with version display"
```

---

### Task 6: 修改 FooterBar 添加「偏好」按钮

**Files:**
- Modify: `src/components/FooterBar.tsx`

- [ ] **Step 1: 更新 FooterBar 组件**

将 `src/components/FooterBar.tsx` 内容替换为：

```tsx
import type { DayInfo } from "../types";

interface FooterBarProps {
  day: DayInfo;
  onGoToToday: () => void;
  onOpenSettings: () => void;
}

export function FooterBar({ day, onGoToToday, onOpenSettings }: FooterBarProps) {
  return (
    <div className="footer">
      <div className="footer-left">
        <button className="footer-settings-btn" onClick={onOpenSettings}>
          偏好
        </button>
        <span className="footer-info">
          {day.shengxiao}月 · {day.ganzhiYear}
        </span>
      </div>
      <button className="today-btn" onClick={onGoToToday}>
        回到今天
      </button>
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/FooterBar.tsx
git commit -m "feat: add settings button to FooterBar"
```

---

### Task 7: 改造 App.tsx 页面路由 + 事件监听

**Files:**
- Modify: `src/App.tsx`

- [ ] **Step 1: 重写 App.tsx**

将 `src/App.tsx` 内容替换为：

```tsx
import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrent } from "@tauri-apps/api/window";
import { ThemeProvider } from "./components/ThemeProvider";
import { useCalendar } from "./hooks/useCalendar";
import { DayDetail } from "./components/DayDetail";
import { CalendarGrid } from "./components/CalendarGrid";
import { FooterBar } from "./components/FooterBar";
import { SettingsPage } from "./components/SettingsPage";
import { AboutPage } from "./components/AboutPage";
import type { PageId } from "./types";

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
      <CalendarGrid
        monthGrid={monthGrid}
        selectedDate={selectedDate}
        today={today}
        onSelectDate={selectDate}
        viewYear={viewYear}
        viewMonth={viewMonth}
        onPrevMonth={prevMonth}
        onNextMonth={nextMonth}
      />
      <div className="divider" />
      {selectedDayInfo && (
        <FooterBar day={selectedDayInfo} onGoToToday={goToToday} onOpenSettings={goToSettings} />
      )}
    </div>
  );
}

function AppContent() {
  const [page, setPage] = useState<PageId>("calendar");

  const goToSettings = () => setPage("settings");
  const goToAbout = () => setPage("about");
  const goToCalendar = () => setPage("calendar");

  useEffect(() => {
    const unlisten = listen<string>("navigate-to", async (event) => {
      const pageId = event.payload as PageId;
      setPage(pageId);
      await getCurrent().show();
      await getCurrent().setFocus();
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  if (page === "settings") {
    return (
      <div className="app-frame">
        <SettingsPage onBack={goToCalendar} />
      </div>
    );
  }

  if (page === "about") {
    return (
      <div className="app-frame">
        <AboutPage onBack={goToCalendar} />
      </div>
    );
  }

  return <CalendarApp />;
}
```

注意：这里 `CalendarApp` 需要能访问 `goToSettings`，所以需要在 `AppContent` 中定义并通过 props 传递，或者用 context。更简洁的方式是把 `goToSettings` 提到 props 层。实际上 `CalendarApp` 在 `AppContent` 内部，可以直接闭包访问。让我们调整一下结构：

将 `src/App.tsx` 内容替换为：

```tsx
import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrent } from "@tauri-apps/api/window";
import { ThemeProvider } from "./components/ThemeProvider";
import { useCalendar } from "./hooks/useCalendar";
import { DayDetail } from "./components/DayDetail";
import { CalendarGrid } from "./components/CalendarGrid";
import { FooterBar } from "./components/FooterBar";
import { SettingsPage } from "./components/SettingsPage";
import { AboutPage } from "./components/AboutPage";
import type { PageId } from "./types";

function CalendarView({ onOpenSettings }: { onOpenSettings: () => void }) {
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
      <CalendarGrid
        monthGrid={monthGrid}
        selectedDate={selectedDate}
        today={today}
        onSelectDate={selectDate}
        viewYear={viewYear}
        viewMonth={viewMonth}
        onPrevMonth={prevMonth}
        onNextMonth={nextMonth}
      />
      <div className="divider" />
      {selectedDayInfo && (
        <FooterBar day={selectedDayInfo} onGoToToday={goToToday} onOpenSettings={onOpenSettings} />
      )}
    </div>
  );
}

function AppContent() {
  const [page, setPage] = useState<PageId>("calendar");

  const goToCalendar = () => setPage("calendar");

  useEffect(() => {
    const unlisten = listen<string>("navigate-to", async (event) => {
      const pageId = event.payload as PageId;
      setPage(pageId);
      await getCurrent().show();
      await getCurrent().setFocus();
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  if (page === "settings") {
    return (
      <div className="app-frame">
        <SettingsPage onBack={goToCalendar} />
      </div>
    );
  }

  if (page === "about") {
    return (
      <div className="app-frame">
        <AboutPage onBack={goToCalendar} />
      </div>
    );
  }

  return <CalendarView onOpenSettings={() => setPage("settings")} />;
}

export default function App() {
  return (
    <ThemeProvider>
      <AppContent />
    </ThemeProvider>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/App.tsx
git commit -m "feat: add page routing and navigate-to event listener"
```

---

### Task 8: 字体放大 CSS 修改

**Files:**
- Modify: `src/index.css`

- [ ] **Step 1: 更新字体尺寸**

在 `src/index.css` 中逐一修改以下选择器的 font-size / 尺寸：

| 选择器 | 属性 | 旧值 | 新值 |
|--------|------|------|------|
| `.nav-btn` | `width` / `height` | `24px` | `28px` |
| `.nav-btn` | `font-size` | `10px` | `12px` |
| `.month-title` | `font-size` | `15px` | `18px` |
| `.weekday` | `font-size` | `10px` | `12px` |
| `.day-num` | `font-size` | `12px` | `14px` |
| `.day-lunar` | `font-size` | `8px` | `10px` |

- [ ] **Step 2: Commit**

```bash
git add src/index.css
git commit -m "feat: enlarge font sizes by ~20%"
```

---

### Task 9: 新增页面 CSS 样式

**Files:**
- Modify: `src/index.css`

- [ ] **Step 1: 在 `src/index.css` 文件末尾添加新样式**

在文件最末尾追加：

```css
/* ═══════════════════════════════════════
   Footer Update
   ═══════════════════════════════════════ */

.footer-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.footer-settings-btn {
  font-size: 12px;
  color: var(--text-secondary);
  background: var(--accent-light);
  border: 1px solid var(--divider);
  border-radius: 5px;
  padding: 3px 10px;
  cursor: pointer;
  transition: all 0.2s;
  font-family: "Noto Sans SC", sans-serif;
}

.footer-settings-btn:hover {
  color: var(--accent);
  border-color: var(--accent);
  background: var(--accent-medium);
}

/* ═══════════════════════════════════════
   NavBar
   ═══════════════════════════════════════ */

.nav-bar {
  padding: 14px 16px;
  display: flex;
  align-items: center;
  border-bottom: 1px solid var(--divider);
  flex-shrink: 0;
}

.nav-back {
  font-size: 14px;
  color: var(--accent);
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
  font-family: "Noto Sans SC", sans-serif;
  transition: opacity 0.2s;
}

.nav-back:hover {
  opacity: 0.7;
}

.nav-title {
  flex: 1;
  text-align: center;
  font-weight: 600;
  font-size: 15px;
  color: var(--text-primary);
  margin-right: 42px;
}

/* ═══════════════════════════════════════
   Settings Page
   ═══════════════════════════════════════ */

.settings-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.settings-content {
  padding: 16px;
  flex: 1;
}

.settings-label {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 14px;
  letter-spacing: 0.5px;
}

.theme-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.theme-card {
  border: 1px solid var(--divider);
  border-radius: 10px;
  padding: 12px;
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  transition: all 0.15s;
}

.theme-card:hover {
  background: var(--hover-bg);
}

.theme-card.is-active {
  border: 2px solid var(--accent);
  background: var(--accent-light);
}

.theme-preview {
  width: 48px;
  height: 48px;
  border-radius: 8px;
  border: 1px solid var(--divider);
  flex-shrink: 0;
}

.theme-info {
  flex: 1;
}

.theme-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.theme-desc {
  font-size: 11px;
  color: var(--text-secondary);
  margin-top: 2px;
}

.theme-check {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--accent);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 12px;
  flex-shrink: 0;
}

/* ═══════════════════════════════════════
   About Page
   ═══════════════════════════════════════ */

.about-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.about-content {
  padding: 32px 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  flex: 1;
  justify-content: center;
}

.about-icon {
  font-size: 56px;
}

.about-name {
  font-family: "Noto Serif SC", serif;
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: 2px;
}

.about-version {
  font-size: 13px;
  color: var(--text-secondary);
}

.about-divider {
  height: 1px;
  width: 60%;
  background: var(--divider);
  margin: 4px 0;
}

.about-desc {
  font-size: 12px;
  color: var(--text-muted);
  text-align: center;
  line-height: 1.6;
}

.about-copyright {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 8px;
}
```

- [ ] **Step 2: Commit**

```bash
git add src/index.css
git commit -m "feat: add NavBar, SettingsPage, AboutPage CSS styles"
```

---

### Task 10: Rust - 新增 get_app_version command

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: 添加 get_app_version command**

在 `src-tauri/src/lib.rs` 中，在 `use` 块后面添加：

```rust
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
```

- [ ] **Step 2: 注册 command**

在 `tauri::Builder::default()` 链中添加 `.invoke_handler`：

将 `run()` 函数中的 `.plugin(tauri_plugin_opener::init())` 后面添加：

```rust
.invoke_handler(tauri::generate_handler![get_app_version])
```

完整的 `run()` 函数 builder 链变为：

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![get_app_version])
    .setup(|app| {
        // ... 现有代码不变
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

- [ ] **Step 3: 验证 Rust 编译通过**

```bash
cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check
```

Expected: 编译成功，无错误

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: add get_app_version Tauri command"
```

---

### Task 11: Rust - 改造托盘菜单

**Files:**
- Modify: `src-tauri/src/tray.rs`

- [ ] **Step 1: 替换菜单项**

将 `create_tray` 函数中的菜单构建部分替换为：

```rust
let menu = MenuBuilder::new(app)
    .item(&MenuItemBuilder::with_id("nav_settings", "偏好").build(app)?)
    .item(&MenuItemBuilder::with_id("nav_about", "版本").build(app)?)
    .separator()
    .item(&MenuItemBuilder::with_id("quit", "退出").build(app)?)
    .build()?;
```

- [ ] **Step 2: 替换菜单事件处理**

将 `on_menu_event` 闭包替换为：

```rust
.on_menu_event(|app, event| {
    match event.id().as_ref() {
        "nav_settings" => {
            let _ = app.emit("navigate-to", "settings");
        }
        "nav_about" => {
            let _ = app.emit("navigate-to", "about");
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
})
```

- [ ] **Step 3: 验证 Rust 编译通过**

```bash
cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check
```

Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/tray.rs
git commit -m "feat: update tray menu to preferences/version/quit"
```

---

### Task 12: TypeScript 类型检查 + 集成验证

**Files:**
- 无新增/修改

- [ ] **Step 1: 运行 TypeScript 类型检查**

```bash
cd /Users/eskyfun/develop/private_project/cc-day && npx tsc --noEmit
```

Expected: 0 errors

如果有类型错误，逐一修复后重新检查。

- [ ] **Step 2: 启动开发服务器进行手动测试**

```bash
cd /Users/eskyfun/develop/private_project/cc-day && pnpm tauri dev
```

验证清单：
- [ ] 日历页面字体比之前大，可读性提升
- [ ] Footer 左侧有「偏好」按钮，风格与「回到今天」一致
- [ ] 点击「偏好」→ 页面切换到设置页，显示 3 个主题卡片
- [ ] 点击主题卡片 → 主题立即切换，✓ 标记跟随
- [ ] 点击「← 返回」→ 回到日历页面
- [ ] 右键托盘图标 → 菜单显示「偏好」「版本」「退出」
- [ ] 点击托盘「偏好」→ 弹窗打开并显示设置页
- [ ] 点击托盘「版本」→ 弹窗打开并显示关于页
- [ ] 关于页显示版本号 0.1.0
- [ ] 点击关于页「← 返回」→ 回到日历页面
- [ ] 点击「退出」→ 应用退出

- [ ] **Step 3: 最终 Commit（如有修复）**

```bash
git add -A
git commit -m "fix: address type check and integration issues"
```
