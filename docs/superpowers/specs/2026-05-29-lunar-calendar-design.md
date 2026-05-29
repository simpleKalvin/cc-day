# CC-Day 农历日历桌面应用设计

## 概述

跨平台桌面日历工具，主打中国农历展示。Mac 顶部菜单栏 / Windows 右下角系统托盘驻留，点击展开日历面板。

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 |
| 前端 | React 19 + TypeScript |
| UI 组件 | Shadcn UI + Tailwind CSS |
| 农历计算 | lunar-javascript（前端 JS 库，零网络依赖） |
| 后端 | Rust（托盘图标、窗口管理） |
| 构建 | pnpm scripts |
| 发布 | GitHub Actions + tauri-action |

## 架构

### 分工

- **Rust 端**：系统托盘注册、动态图标渲染（日期数字）、无边框弹窗窗口定位与显示/隐藏、点击外部关闭
- **React 端**：全部 UI 渲染，通过 `lunar-javascript` 在前端完成农历计算

### 无主窗口

启动即最小化到托盘，不显示 dock/taskbar 图标。`tauri.conf.json` 的 `app.windows` 设为空数组，仅存在弹窗窗口。

### 系统托盘图标

- 使用 Tauri v2 `TrayIconBuilder` API
- 动态显示当天日期数字（如 "26"）
- 实现方式：用 Rust `image` crate 在内存中绘制（白底 + 数字），或使用预生成的 1-31 号图标资源
- 每天零点自动更新

### 弹窗窗口

- 无边框、无标题栏
- 尺寸约 320 x 420 px，固定大小不可调整
- 点击托盘图标 → 显示/隐藏切换
- 点击窗口外部 → 自动关闭
  - macOS：监听 `windowDidResignKey`
  - Windows：监听 `WM_ACTIVATE`
- 窗口位置：Mac 菜单栏图标正下方，Windows 托盘图标正上方

## UI 设计

### 布局（方案 A：详情在上 + 月历在下）

```
┌────────────────────────────┐
│  渐变色详情区               │
│  2026年5月28日 星期四        │
│  农历五月初二               │
│  丙午年 甲午月 庚子日        │
│  [节气标签]                 │
├────────────────────────────┤
│  ◀  2026年5月  ▶           │
│  日 一 二 三 四 五 六       │
│  27 28 29 30 31  1  2      │
│  初 初 初 初 初 初 初       │
│  ...                       │
└────────────────────────────┘
```

### 组件拆分

| 组件 | 职责 |
|------|------|
| TrayIcon | Rust 端动态生成图标，显示当天日期数字 |
| DayDetail | 当日详情面板（农历日期、天干地支、节气、节日） |
| CalendarGrid | 月历网格，每格显示公历数字 + 农历小字 |
| MonthNav | 月份导航（上/下月切换） |

### 详情区展示内容

- 农历日期（如"五月初二"）
- 天干地支（年月日柱）
- 节气（当天或最近的节气）
- 传统节日（端午、中秋等）
- 公历节日（元旦、国庆等）

## 构建与发布

### 开发

```bash
pnpm dev          # 纯前端开发（浏览器调试 UI）
pnpm tauri dev    # 完整开发模式（Vite 热更新 + Rust 编译）
```

### 构建

```bash
pnpm tauri build  # 生产构建
```

产物：Mac → `.dmg` + `.app`，Windows → `.exe` + `.msi`

### pnpm scripts

```json
{
  "dev": "vite",
  "build": "tsc && vite build",
  "preview": "vite preview",
  "tauri": "tauri",
  "tauri:dev": "tauri dev",
  "tauri:build": "tauri build"
}
```

### 发布（GitHub Actions）

- 使用 `tauri-apps/tauri-action` 官方 Action
- 触发条件：推送 tag（如 `v0.1.0`）
- macOS 和 Windows runner 并行构建
- 产物自动上传到 GitHub Releases
- 工作流文件：`.github/workflows/release.yml`
