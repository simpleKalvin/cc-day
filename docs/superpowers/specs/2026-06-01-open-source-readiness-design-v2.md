# CC-Day 开源项目准备设计（v2）

## 背景

CC-Day 是一个基于 Tauri v2 + React 的 macOS 菜单栏农历日历应用，当前版本 0.1.0。项目使用了 macOS 专属的 NSPanel API（`tauri-nspanel`）和 `macos-private-api` feature，因此第一阶段仅面向 macOS 平台发布。

## 约束

- 面向中文用户，文档全部中文
- 许可证：Apache License 2.0
- 当前仅支持 macOS（后续版本再加 Windows/Linux）
- 暂不做代码签名，README 中说明 Gatekeeper 处理方式
- 持续迭代的产品，需要 CI + 自动发布

## 变更清单

### 1. LICENSE 文件

根目录添加标准 Apache License 2.0 全文。年份 2025，版权人 simpleKalvin。

### 2. package.json 修复

- 删除 `"private": true`
- 添加 `"description": "macOS 菜单栏农历日历"`
- 添加 `"license": "Apache-2.0"`
- 添加 `"keywords": ["lunar-calendar", "tauri", "tray", "calendar", "chinese-calendar", "macos"]`
- 添加 `"repository": { "type": "git", "url": "https://github.com/simpleKalvin/cc-day" }`
- 添加 `"homepage": "https://github.com/simpleKalvin/cc-day"`

### 3. Cargo.toml 修复

- `authors` 改为 `["simpleKalvin"]`

### 4. .gitignore 修复

将 `.DS_Store.superpowers/` 拆为两行：
```
.DS_Store
.superpowers/
```

### 5. tauri.conf.json 修改

- `identifier` 改为 `com.simplekalvin.cc-day`

### 6. README.md 重写

替换当前 Tauri 模板内容，结构如下：

```
# CC-Day
一句话描述：macOS 菜单栏农历日历

## 功能特性
- 动态日期图标（托盘图标显示当前日期）
- 农历信息展示（农历日期、节气、节日等）
- 三套主题切换（淡墨水彩 / 莫兰迪雅粉 / 赤金宫墙）
- 轻量级菜单栏应用，不占用 Dock 位置

## 平台支持
当前仅支持 macOS，后续版本将支持 Windows/Linux。

## 安装
前往 GitHub Releases 下载 .dmg 安装包。
首次打开如遇"无法验证开发者"提示，请右键点击 → 打开。

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

### 7. GitHub Actions CI

文件：`.github/workflows/ci.yml`

触发：push 和 PR 到 `main` 分支。

仅 macOS 构建（`macos-latest`）：
1. Checkout
2. 安装 pnpm + Node.js 22
3. 安装 Rust stable
4. `pnpm build`（TypeScript + Vite）
5. `pnpm tauri build`（全量构建）

构建成功即通过。

### 8. GitHub Actions Release

文件：`.github/workflows/release.yml`

触发：推送 `v*` tag（如 `v0.1.0`）。

仅 macOS 构建：
1. Checkout
2. 安装 pnpm + Node.js 22
3. 安装 Rust stable
4. `pnpm tauri build`
5. 使用 `tauri-action` 上传到 GitHub Release

产物：`.dmg`。

### 9. GitHub 模板

**`.github/ISSUE_TEMPLATE/bug_report.md`：**
中文 bug 报告模板：问题描述、复现步骤、期望行为、实际行为、系统信息。

**`.github/ISSUE_TEMPLATE/feature_request.md`：**
中文功能请求模板：功能描述、使用场景、期望效果。

**`.github/PULL_REQUEST_TEMPLATE.md`：**
中文 PR 模板：变更说明、关联 issue、测试方式、截图。

## 不在范围内

- CHANGELOG.md
- CONTRIBUTING.md
- 测试框架搭建
- CSP 安全加固
- 代码签名 / notarization
- Windows / Linux 支持
- README 截图/GIF 自动化
