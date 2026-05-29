# 修复：macOS 全屏应用下窗口无法正常显示

## 问题

CC-Day 日历窗口在常规桌面下点击托盘图标可正常显示，但当屏幕有全屏应用（如 iTerm2、VSCode）运行时，窗口无法显示或闪现后瞬间隐藏。

### 根因一：Space 限制

Tauri 底层 NSWindow 默认钉在创建时的常规桌面 Space。全屏应用运行在独立虚拟桌面中，系统限制窗口跨 Space 显示。需要赋予 `CanJoinAllSpaces` 和 `FullScreenAuxiliary` 权限。

### 根因二：失焦竞态

在全屏应用下 show() 窗口时，窗口未能第一时间抢占焦点，立即触发 `WindowEvent::Focused(false)` 监听器，导致窗口刚显示就被 hide()。

## 方案：Tauri 2.0 原生 API + AtomicBool Flag

### 段 1：窗口构建 — 跨 Space + 全屏浮动

在 `WebviewWindowBuilder` 链式调用中增加：

```rust
.visible_on_all_workspaces(true)
```

Tauri 2.0 底层映射到 `CanJoinAllSpaces`。

`build()` 之后，用 `#[cfg(target_os = "macos")]` 包裹一段 cocoa 调用，补充设置 `NSWindowCollectionBehaviorFullScreenAuxiliary`，确保窗口能浮在全屏应用上方。

依赖变更：`Cargo.toml` 通过 `[target.'cfg(target_os = "macos")'.dependencies]` 条件引入 `cocoa` crate。

### 段 2：失焦保护 — AtomicBool Flag

引入 `Arc<AtomicBool>` 作为 show/hide 竞态防护：

- **show 前**：`is_showing.store(true, SeqCst)`
- **Focused(false) 事件**：检查 flag，若为 true 则忽略（不 hide）
- **Focused(true) 事件**：`is_showing.store(false, SeqCst)`，恢复正常失焦响应

`is_showing` 通过 `app.manage()` 存入 Tauri 状态管理，tray 模块从 `app.state()` 取出使用。

时序保证：
1. `store(true)` 先于 `show()`
2. show() 后即使系统先发出 `Focused(false)`，被 flag 拦截
3. 窗口真正获焦后 `Focused(true)` 将 flag 置 false
4. 此后正常失焦触发 hide

### 段 3：跨平台安全

| 代码区域 | macOS | Windows |
|---|---|---|
| `visible_on_all_workspaces(true)` | Tauri 原生支持 | Tauri 忽略，无副作用 |
| `FullScreenAuxiliary` cocoa 代码 | `#[cfg(target_os = "macos")]` 包裹 | 不编译 |
| `is_showing` AtomicBool | 纯 Rust，全平台生效 | 同上 |
| `cocoa` 依赖 | 条件引入 | 不编译 |

## 变更清单

| 文件 | 变更内容 |
|---|---|
| `src-tauri/Cargo.toml` | 增加 `cocoa` 条件依赖 |
| `src-tauri/src/lib.rs` | 窗口构建加 `visible_on_all_workspaces(true)` + macOS cocoa 补丁 + `is_showing` 状态管理 + 失焦事件改写 |
| `src-tauri/src/tray.rs` | 从状态取 `is_showing`，show 前置 flag |

不变更：`tauri.conf.json`、`Info.plist`、前端代码。
