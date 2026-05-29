# UI 改进设计：页面导航、主题切换、字体放大

日期：2026-05-29

## 概述

对 CC-Day 农历日历弹窗进行三项改进：引入页内导航（设置页/关于页）、改造托盘菜单、温和放大字体提升可读性。

## 1. 字体放大

温和放大约 20%，保持 320px 宽度内网格呼吸感。

| 元素 | 当前 | 放大后 |
|------|------|--------|
| 日历数字 `.day-num` | 12px | 14px |
| 农历文字 `.day-lunar` | 8px | 10px |
| 星期标题 `.weekday` | 10px | 12px |
| 月份标题 `.month-title` | 15px | 18px |
| 导航按钮 `.nav-btn` | 24×24px | 28×28px |
| 月份导航箭头字号 | 10px | 12px |

## 2. Footer 改造

当前 footer：左侧 `footer-info`（生肖月 · 干支年），右侧「回到今天」按钮。

改造后：
- 左侧新增文字按钮「偏好」，位于 `footer-info` 左边
- 按钮风格与「回到今天」一致（带底色和边框的小按钮），颜色使用 `--text-secondary`
- 点击触发页面切换到 settings 页

```
[ 偏好 ]  马月 · 丙午年          [ 回到今天 ]
```

## 3. 页面导航系统

### 状态管理

App.tsx 新增 `page` state，类型为 `'calendar' | 'settings' | 'about'`，默认 `'calendar'`。

### 导航栏

设置页和关于页共享统一的顶部导航栏组件 `NavBar`：

```
[ ← 返回 ]                    偏好设置
```

- 左侧：「← 返回」按钮，颜色为主题 accent 色
- 中间：页面标题，15px 加粗
- 右侧留空（与左侧按钮宽度对称）

### 页面切换

点击 footer「偏好」→ `setPage('settings')`
点击导航栏「← 返回」→ `setPage('calendar')`
托盘菜单「版本」→ 打开弹窗 + `setPage('about')`

页面切换时无动画，直接替换渲染（320px 弹窗内动画反而显多余）。

## 4. 设置页（主题切换）

### 布局

```
┌─────────────────────────────┐
│  ← 返回           偏好设置   │  ← NavBar
├─────────────────────────────┤
│  选择主题                    │
│                             │
│  ┌─────────────────────────┐│
│  │ ■ 淡墨水彩          ✓  ││  ← 选中项（accent 边框 + ✓）
│  │   水墨淡雅，温润如玉    ││
│  └─────────────────────────┘│
│  ┌─────────────────────────┐│
│  │ ■ 莫兰迪雅粉            ││
│  │   柔雅低饱和，静谧温柔  ││
│  └─────────────────────────┘│
│  ┌─────────────────────────┐│
│  │ ■ 赤金宫墙              ││
│  │   红墙金瓦，恢弘大气    ││
│  └─────────────────────────┘│
└─────────────────────────────┘
```

### 主题卡片

- 左侧：48×48px 色块预览（`border-radius: 8px`），展示主题的渐变色
- 中间：主题名（14px 加粗）+ 一行描述（11px muted）
- 右侧：选中项显示 20px 圆形 accent 色背景 + 白色 ✓
- 选中项：2px accent 边框 + 淡底色；未选中：1px 淡边框
- 点击卡片调用 `setTheme(id)`，主题立即生效

### 主题描述文案

| 主题 | 名称 | 描述 |
|------|------|------|
| ink-wash | 淡墨水彩 | 水墨淡雅，温润如玉 |
| morandi | 莫兰迪雅粉 | 柔雅低饱和，静谧温柔 |
| palace | 赤金宫墙 | 红墙金瓦，恢弘大气 |

## 5. 关于页面

居中布局：

```
┌─────────────────────────────┐
│  ← 返回               关于   │  ← NavBar
├─────────────────────────────┤
│                             │
│            📅               │  ← 56px 图标
│         CC-Day              │  ← 20px 衬线体加粗
│         版本 1.0.0          │  ← 13px muted
│         ──────              │  ← 分隔线
│    一款简洁优雅的农历日历    │  ← 12px muted
│    托盘常驻，随时查看        │
│      © 2026 CC-Day          │  ← 11px
│                             │
└─────────────────────────────┘
```

版本号从 `tauri.conf.json` 的 `version` 字段读取，通过 Tauri command 获取。

## 6. 托盘菜单改造（Rust）

### 当前菜单

```
淡墨水彩
莫兰迪雅粉
赤金宫墙
────────
退出
```

### 改造后菜单

```
偏好
版本
────────
退出
```

### 行为

- 「偏好」→ 发出 Tauri event `navigate-to`，payload 为 `"settings"`，前端监听后打开弹窗 + 切换到 settings 页
- 「版本」→ 发出 Tauri event `navigate-to`，payload 为 `"about"`，前端监听后打开弹窗 + 切换到 about 页
- 「退出」→ `app.exit(0)`，不变

### 前端监听

ThemeProvider（或 App）新增对 `navigate-to` event 的监听，收到后 `setPage(payload)` + 调用 Tauri API 显示窗口。

## 7. 文件变更清单

### 新增

| 文件 | 说明 |
|------|------|
| `src/components/SettingsPage.tsx` | 主题切换设置页 |
| `src/components/AboutPage.tsx` | 版本信息页 |
| `src/components/NavBar.tsx` | 通用返回导航栏 |

### 修改

| 文件 | 变更 |
|------|------|
| `src/App.tsx` | 新增 `page` state，根据 state 渲染不同页面，监听 `navigate-to` event |
| `src/components/FooterBar.tsx` | 新增「偏好」文字按钮，接收 `onOpenSettings` 回调 |
| `src/components/ThemeProvider.tsx` | 导出主题元数据（名称、描述、色块渐变）供设置页使用 |
| `src/index.css` | 字体尺寸放大，新增设置页/关于页/导航栏样式 |
| `src/types.ts` | 新增 `PageId` 类型，主题元数据类型 |
| `src-tauri/src/tray.rs` | 菜单项改为偏好/版本/退出，事件改为 `navigate-to` |
| `src-tauri/src/lib.rs` | 新增 `get_app_version` command |

## 8. 技术细节

### 版本号获取

新增 Tauri command `get_app_version`，返回 `tauri.conf.json` 中配置的版本字符串。前端在 about 页面调用 `invoke("get_app_version")` 获取。

### 主题切换联动

设置页点击主题卡片 → 调用 `setTheme(id)` → ThemeProvider 更新 `data-theme` 属性 + localStorage。已有机制无需改动，设置页只是新增了一个 UI 入口。

### 导航栏组件

```tsx
interface NavBarProps {
  title: string;
  onBack: () => void;
}
```

无动画、无过渡效果。页面直接替换。
