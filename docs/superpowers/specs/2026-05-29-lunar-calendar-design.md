# CC-Day 农历日历桌面应用设计

## 概述

跨平台桌面日历工具，主打中国农历展示。Mac 顶部菜单栏 / Windows 右下角系统托盘驻留，点击展开日历面板。

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 |
| 前端 | React 19 + TypeScript |
| UI 组件 | Shadcn UI + Tailwind CSS（CSS 变量驱动主题） |
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

### 布局（详情在上 + 月历在下）

```
┌────────────────────────────┐
│  详情区（主题渐变背景）      │
│  2026年5月28日 星期四        │
│  农历五月初二               │
│  丙午年 甲午月 庚子日        │
│  [节气标签] [宜忌标签]      │
├────────────────────────────┤
│  ◀  2026年5月  ▶           │
│  日 一 二 三 四 五 六       │
│  27 28 29 30 31  1  2      │
│  初 初 初 初 初 初 初       │
│  ...                       │
├────────────────────────────┤
│  马月 · 丙午    [回到今天]  │
└────────────────────────────┘
```

### 组件拆分

| 组件 | 职责 |
|------|------|
| TrayIcon | Rust 端动态生成图标，显示当天日期数字 |
| DayDetail | 当日详情面板（农历日期、天干地支、节气、节日） |
| CalendarGrid | 月历网格，每格显示公历数字 + 农历小字 |
| MonthNav | 月份导航（上/下月切换） |
| FooterBar | 底部信息栏（生肖月份 + "回到今天"按钮） |
| ThemeProvider | 主题上下文提供者，管理主题切换与自定义 |

### 详情区展示内容

- 农历日期（如"五月初二"）
- 天干地支（年月日柱）
- 节气（当天或最近的节气）
- 传统节日（端午、中秋等）
- 公历节日（元旦、国庆等）
- 宜忌标签（可选展示）

### 配色方案

#### 主题架构

所有配色通过 CSS 变量（`--bg-primary`, `--accent` 等）驱动，组件不硬编码颜色值。运行时通过切换根元素的 CSS 变量集实现主题切换。

```css
:root[data-theme="ink-wash"] { /* 淡墨水彩变量 */ }
:root[data-theme="morandi"]   { /* 莫兰迪雅粉变量 */ }
:root[data-theme="palace"]    { /* 赤金宫墙变量 */ }
```

#### 方案 A：淡墨水彩（ink-wash）

暖白宣纸底色 + 金墨点缀，水墨晕染渐变，文雅含蓄。

| 变量 | 值 | 用途 |
|------|-----|------|
| `--bg-primary` | `#f8f5ef` | 主背景 |
| `--bg-secondary` | `#f0ebe0` | 次级背景（footer） |
| `--bg-header-start` | `#e8e2d4` | 详情区渐变起点 |
| `--bg-header-end` | `#f2ece0` | 详情区渐变终点 |
| `--accent` | `#a67c52` | 金色强调（今天、按钮） |
| `--accent-ink` | `#2c2c3a` | 墨色（标题、正文） |
| `--text-primary` | `#3a3a4a` | 主文字 |
| `--text-secondary` | `#7a7a8a` | 次级文字 |
| `--text-muted` | `#aaa8a0` | 弱化文字（非本月日期） |
| `--festival` | `#c45a5a` | 节日红 |
| `--festival-bg` | `rgba(196,90,90,0.08)` | 节日标签背景 |
| `--jieqi` | `#4a7a5a` | 节气绿 |
| `--jieqi-bg` | `rgba(74,122,90,0.08)` | 节气标签背景 |
| `--weekend` | `#b07060` | 周末文字 |
| `--divider` | `rgba(166,124,82,0.12)` | 分割线 |

#### 方案 B：莫兰迪雅粉（morandi）

低饱和粉灰调，柔和温婉，欧式艺术气质，农历标题用粉色强调。

| 变量 | 值 | 用途 |
|------|-----|------|
| `--bg-primary` | `#f4efe8` | 主背景 |
| `--bg-secondary` | `#ece5db` | 次级背景 |
| `--bg-header-start` | `#e8dcd2` | 详情区渐变起点 |
| `--bg-header-end` | `#f0e8de` | 详情区渐变终点 |
| `--accent` | `#b07080` | 粉色强调（今天、按钮、农历标题） |
| `--accent-ink` | `#4a4240` | 深棕文字 |
| `--text-primary` | `#4a4240` | 主文字 |
| `--text-secondary` | `#8a8078` | 次级文字 |
| `--text-muted` | `#b0a89e` | 弱化文字 |
| `--festival` | `#c47070` | 节日红 |
| `--festival-bg` | `rgba(196,112,112,0.1)` | 节日标签背景 |
| `--jieqi` | `#7a9a7a` | 节气绿 |
| `--jieqi-bg` | `rgba(122,154,122,0.1)` | 节气标签背景 |
| `--weekend` | `#a08878` | 周末文字 |
| `--divider` | `rgba(176,112,128,0.12)` | 分割线 |

#### 方案 C：赤金宫墙（palace）

宫墙朱红 header + 白底日历，故宫气质，庄重大气，节日感强。

| 变量 | 值 | 用途 |
|------|-----|------|
| `--bg-primary` | `#faf6f0` | 主背景 |
| `--bg-secondary` | `#f2ebe0` | 次级背景 |
| `--bg-header-start` | `#c4342e` | 详情区渐变起点（朱红） |
| `--bg-header-end` | `#d44838` | 详情区渐变终点 |
| `--accent` | `#c4342e` | 朱红强调（今天、按钮） |
| `--accent-gold` | `#c8a44a` | 金色（宜忌标签、装饰） |
| `--accent-ink` | `#3a2a20` | 深棕文字 |
| `--text-primary` | `#3a2a20` | 主文字 |
| `--text-secondary` | `#8a7a6a` | 次级文字 |
| `--text-muted` | `#b0a898` | 弱化文字 |
| `--festival` | `#c4342e` | 节日红 |
| `--festival-bg` | `rgba(196,52,46,0.08)` | 节日标签背景 |
| `--jieqi` | `#5a8a5a` | 节气绿 |
| `--jieqi-bg` | `rgba(90,138,90,0.08)` | 节气标签背景 |
| `--weekend` | `#a06050` | 周末文字 |
| `--divider` | `rgba(200,164,74,0.15)` | 分割线 |
| `--header-text` | `#ffffff` | 详情区文字（白底红底上用白字） |

### 主题切换

- 默认主题：淡墨水彩（ink-wash）
- 切换入口：右键托盘图标 → "主题" 子菜单，或详情区长按弹出选择
- 主题偏好持久化到本地存储（Tauri 的 `Store` 插件或 `localStorage`）
- React 端通过 `ThemeContext` 提供 `useTheme()` hook，组件消费 CSS 变量，不感知具体色值

### 自定义配色预留

为后续用户自定义配色预留扩展点：

1. **变量接口标准化**：所有主题必须提供同一套 CSS 变量，自定义主题只需填写变量值
2. **设置面板入口**：右键托盘菜单 "自定义主题..." 打开设置弹窗，提供颜色选择器让用户调整各变量
3. **主题数据结构**：
   ```typescript
   interface ThemeConfig {
     id: string;
     name: string;
     variables: Record<string, string>; // CSS 变量键值对
     isBuiltIn?: boolean;
   }
   ```
4. **存储位置**：用户自定义主题 JSON 存储在 Tauri app data 目录（`app_data/themes/`）
5. **主题导入/导出**：支持 JSON 文件导入导出，方便分享主题

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
