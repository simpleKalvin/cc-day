# CC-Day 开源项目准备设计

## 背景

CC-Day 是一个基于 Tauri v2 + React 的跨平台菜单栏农历日历应用，当前处于 0.1.0 阶段。功能层面已基本完成（动态日期图标、三套主题、农历信息展示），但作为开源项目缺少必要的文件和基础设施。目标是将项目补齐至可公开发布于 GitHub 的状态。

## 约束

- 面向中文用户，文档全部中文
- 许可证：Apache License 2.0
- 支持平台：macOS / Windows / Linux
- 持续迭代的产品，需要 CI + 自动发布

## 变更清单

### 1. LICENSE 文件

根目录添加标准 Apache License 2.0 全文。年份 2025，版权人 simpleKalvin。

### 2. 配置修复

**package.json：**
- 删除 `"private": true`
- 添加 `"description": "跨平台菜单栏农历日历"`
- 添加 `"license": "Apache-2.0"`
- 添加 `"keywords": ["lunar-calendar", "tauri", "tray", "calendar", "chinese-calendar"]`
- 添加 `"repository": { "type": "git", "url": "https://github.com/simpleKalvin/cc-day" }`
- 添加 `"homepage": "https://github.com/simpleKalvin/cc-day"`

**Cargo.toml：**
- `authors` 改为 `["simpleKalvin"]`

**.gitignore：**
- 修正 `.DS_Store.superpowers/` 为两行：`.DS_Store`（已有）和 `.superpowers/`

### 3. README.md 重写

替换当前 Tauri 模板内容，结构如下：

```
# CC-Day
一句话描述：跨平台菜单栏农历日历

## 功能特性
- 动态日期图标（托盘图标显示当前日期）
- 农历信息展示（农历日期、节气、节日等）
- 三套主题切换（淡墨水彩 / 莫兰迪雅粉 / 赤金宫墙）
- 跨平台支持（macOS / Windows / Linux）
- 轻量级菜单栏应用，不占用 Dock 位置

## 截图
预留截图位置

## 安装
前往 GitHub Releases 下载对应平台安装包

## 从源码构建
前置依赖 + 构建步骤

## 开发
pnpm dev / pnpm tauri dev

## 技术栈
Tauri v2 / React 19 / TypeScript / Rust / Tailwind CSS

## 许可证
Apache License 2.0

## 致谢
lunar-javascript
```

### 4. GitHub Actions CI

文件：`.github/workflows/ci.yml`

触发：push 和 PR 到 `main` 分支。

三平台矩阵构建（`ubuntu-latest` / `macos-latest` / `windows-latest`）：
1. Checkout
2. 安装 pnpm + Node.js 22
3. 安装 Rust stable
4. 安装 Linux 系统依赖（libwebkit2gtk-4.1-dev 等）
5. `pnpm build`（TypeScript + Vite）
6. `pnpm tauri build`（全量构建）

构建成功即通过。

### 5. GitHub Actions Release

文件：`.github/workflows/release.yml`

触发：推送 `v*` tag（如 `v0.1.0`）。

三平台矩阵构建：
1. Checkout
2. 安装 pnpm + Node.js 22
3. 安装 Rust stable
4. 安装 Linux 系统依赖
5. `pnpm tauri build`
6. 使用 `tauri-action` 上传到 GitHub Release

产物：macOS `.dmg`、Windows `.msi`/`.exe`、Linux `.AppImage`/`.deb`。

### 6. .github 模板

**`.github/ISSUE_TEMPLATE/bug_report.md`：**
中文 bug 报告模板，包含：问题描述、复现步骤、期望行为、实际行为、系统信息。

**`.github/ISSUE_TEMPLATE/feature_request.md`：**
中文功能请求模板，包含：功能描述、使用场景、期望效果。

**`.github/PULL_REQUEST_TEMPLATE.md`：**
中文 PR 模板，包含：变更说明、关联 issue、测试方式、截图。

## 不在范围内

以下项目暂不处理，待社区增长后再补充：
- CHANGELOG.md
- CONTRIBUTING.md
- 测试框架搭建
- CSP 安全加固
- README 截图/GIF 自动化
