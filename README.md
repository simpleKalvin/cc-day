# CC-Day

macOS 菜单栏农历日历。

![CC-Day 截图](.github/assets/screenshot.png)

## 功能特性

- 动态日期图标 — 托盘图标显示当前日期
- 农历信息展示 — 农历日期、节气、节日等
- 三套主题切换 — 淡墨水彩 / 莫兰迪雅粉 / 赤金宫墙
- 轻量级菜单栏应用，不占用 Dock 位置

## 平台支持

当前仅支持 macOS，后续版本将支持 Windows / Linux。

## 安装

前往 [GitHub Releases](https://github.com/simpleKalvin/cc-day/releases) 下载 `.dmg` 安装包，双击打开后拖拽到 Applications 文件夹。

### 首次启动

由于应用未经过 Apple 公证，macOS 会阻止直接打开。请按以下步骤操作：

1. 在 Finder 中找到 Applications 文件夹下的 **CC-Day**
2. **右键点击** CC-Day → 选择「打开」
3. 在弹出的对话框中再次点击「打开」

> ⚠️ 必须使用**右键 → 打开**的方式，直接双击会被 macOS 拦截。此操作只需执行一次，之后即可正常双击启动。

<!-- 截图占位：首次启动弹窗提示 -->
![首次启动提示](.github/assets/first-launch.png)

## 从源码构建

前置依赖：

- [Node.js](https://nodejs.org/) 22+
- [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/)

```bash
pnpm install
pnpm tauri build
```

产物在 `src-tauri/target/release/bundle/` 目录下。

## 开发

```bash
pnpm install
pnpm tauri dev
```

## 技术栈

Tauri v2 / React 19 / TypeScript / Rust / Tailwind CSS

## 致谢

- [lunar-javascript](https://github.com/6tail/lunar-javascript) — 农历计算库

## 许可证

[Apache License 2.0](LICENSE)
