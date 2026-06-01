# 修复 PanelNotFound 及点击空白无法隐藏

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 修复 tray 第二次点击报 `PanelNotFound` 以及点击空白区域无法隐藏 panel 的问题。

**Architecture:** 根因是 `panel.to_window()` 是破坏性操作，会从 panel 注册表移除 panel 并清除事件处理器。修复方案是用 `app.get_webview_window()` 替代，Tauri 窗口管理器仍持有窗口引用，`set_position()` 可正常工作。

**Tech Stack:** Rust, Tauri 2.0, tauri-nspanel v2.1

---

## 根因分析

### 调用链

```
tray click → get_webview_panel("calendar") → Ok(panel)
  → panel.to_window()            ← 破坏性操作
    → remove_webview_panel()     ← 从注册表移除
    → set_event_handler(None)    ← 清除 window_did_resign_key 回调
    → swizzle class: NSPanel → NSWindow
  → window.set_position(...)
  → panel.show_and_make_key()

下次 tray click → get_webview_panel("calendar") → Err(PanelNotFound)
```

### 两个 bug 的根因

| Bug | 根因 | 代码位置 |
|-----|------|----------|
| 第二次点击报 `PanelNotFound` | `to_window()` 调用 `remove_webview_panel()` 从 HashMap 移除了 panel | `tray.rs:71` |
| 点击空白无法隐藏 | `to_window()` 调用 `set_event_handler(None)` 清除了 `window_did_resign_key` 回调 | `tray.rs:71` |

### 为什么 `get_webview_window()` 可以替代

`PanelBuilder::build()` 的创建流程：
1. `WebviewWindowBuilder::build()` → 创建窗口，注册到 **Tauri 窗口管理器**
2. `window.to_panel()` → swizzle 类为 NSPanel，注册到 **panel 管理器**

两个管理器互不干扰。Tauri 窗口管理器仍持有 "calendar" 窗口引用。`get_webview_window("calendar")` 返回的 `WebviewWindow` 底层对象与 panel 共享同一个 Objective-C 对象，`set_position()` 最终调用 `setFrameOrigin:`，NSPanel 作为 NSWindow 子类完全支持。

---

## 修改文件清单

| 文件 | 操作 | 职责 |
|------|------|------|
| `src-tauri/src/tray.rs` | 修改 | (1) Manager import 改为无条件；(2) to_window() 替换为 get_webview_window() |

---

### Task 1: 修改 tray.rs

**Files:**
- Modify: `src-tauri/src/tray.rs`

- [ ] **Step 1: 将 `Manager` import 改为无条件导入**

将第 9-10 行：

```rust
#[cfg(not(target_os = "macos"))]
use tauri::Manager;
```

改为：

```rust
use tauri::Manager;
```

- [ ] **Step 2: 将 `panel.to_window()` 替换为 `app.get_webview_window()`**

将 macOS tray 点击处理中的第 71 行：

```rust
if let Some(window) = panel.to_window() {
```

改为：

```rust
if let Some(window) = app.get_webview_window("calendar") {
```

- [ ] **Step 3: 验证编译通过**

Run: `cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check`
Expected: 编译成功，无错误

---

### Task 2: 手动验证

- [ ] **Step 1: 启动开发构建**

Run: `cd /Users/eskyfun/develop/private_project/cc-day && pnpm tauri dev`
Expected: 应用启动，托盘图标显示日期

- [ ] **Step 2: 验证常规桌面行为**

1. 点击 tray 图标 → 预期：panel 显示
2. 点击空白区域 → 预期：panel 隐藏
3. 再次点击 tray → 预期：panel 再次显示（之前报 `PanelNotFound`）
4. 重复 3 次 → 预期：每次都正常显示/隐藏

- [ ] **Step 3: 验证全屏应用下行为**

1. 打开 iTerm2 或 VSCode，进入全屏模式
2. 点击 tray → 预期：panel 浮现在全屏应用上方
3. 点击空白 → 预期：panel 隐藏
4. 再次点击 tray → 预期：panel 再次显示

- [ ] **Step 4: 验证 debug 日志**

预期日志（两次点击间点击过空白）：
```
[DEBUG] Tray click received
[DEBUG] get_webview_panel OK
[DEBUG] show_and_make_key called
[DEBUG] Tray click received
[DEBUG] get_webview_panel OK          ← 之前是 ERR: PanelNotFound
[DEBUG] show_and_make_key called
```

---

### Task 3: 合并 Commit

确认验证通过后提交。

- [ ] **Step 1: 提交变更**

```bash
cd /Users/eskyfun/develop/private_project/cc-day
git add src-tauri/src/tray.rs
git commit -m "fix: replace panel.to_window() with get_webview_window() to prevent panel deregistration"
```
