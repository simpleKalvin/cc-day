# macOS 全屏窗口显示修复 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 修复 macOS 全屏应用运行时，点击托盘图标日历窗口无法显示的 Bug。

**Architecture:** 三层修复 — (1) 窗口构建时启用跨 Space + 全屏浮动权限，(2) 引入 AtomicBool flag 防止 show/hide 竞态，(3) 通过 Tauri 状态管理在 lib.rs 和 tray.rs 之间共享 flag。

**Tech Stack:** Rust, Tauri 2.0, cocoa (macOS only)

---

### Task 1: 定义 IsShowingFlag 状态结构体

**Files:**
- Create: `src-tauri/src/show_guard.rs`
- Modify: `src-tauri/src/lib.rs:1`

- [ ] **Step 1: 创建 show_guard 模块文件**

创建 `src-tauri/src/show_guard.rs`，内容如下：

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Tauri 状态管理的 newtype wrapper。
/// show 前置 true，首次 Focused(true) 后置 false，期间忽略 Focused(false)。
pub struct IsShowingFlag(pub Arc<AtomicBool>);

impl IsShowingFlag {
    pub fn will_show(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
}
```

- [ ] **Step 2: 在 lib.rs 中注册模块**

在 `src-tauri/src/lib.rs` 顶部（第 1 行附近）增加模块声明：

```rust
mod show_guard;
```

- [ ] **Step 3: 验证编译通过**

Run: `cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check`
Expected: 编译成功，无错误

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/show_guard.rs src-tauri/src/lib.rs
git commit -m "feat: add IsShowingFlag state for show/hide race protection"
```

---

### Task 2: Cargo.toml 添加 macOS 条件 cocoa 依赖

**Files:**
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: 在 Cargo.toml 末尾添加条件依赖**

在 `src-tauri/Cargo.toml` 文件末尾追加：

```toml
[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.26"
```

- [ ] **Step 2: 验证编译通过**

Run: `cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check`
Expected: 编译成功，cocoa 仅在 macOS 目标下被拉入

- [ ] **Step 3: Commit**

```bash
git add src-tauri/Cargo.toml
git commit -m "feat: add cocoa dependency for macOS-only NSWindow manipulation"
```

---

### Task 3: 改造 lib.rs — 跨 Space + 全屏浮动 + ShowGuard 状态管理

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: 修改窗口构建器，增加 visible_on_all_workspaces**

将 `src-tauri/src/lib.rs` 中 `WebviewWindowBuilder` 的 `.resizable(false)` 行后面、`.build()?` 行前面，插入：

```rust
.visible_on_all_workspaces(true)
```

完整的 builder 链应为：

```rust
let window = WebviewWindowBuilder::new(
    app,
    "calendar",
    WebviewUrl::App("index.html".into()),
)
.title("CC-Day")
.inner_size(320.0, 420.0)
.decorations(false)
.always_on_top(true)
.visible(false)
.resizable(false)
.visible_on_all_workspaces(true)
.build()?;
```

- [ ] **Step 2: 在 build() 之后添加 macOS FullScreenAuxiliary 补丁**

在 `.build()?;` 之后，`let is_showing = ...` 之前，插入：

```rust
#[cfg(target_os = "macos")]
{
    let ns_window: cocoa::base::id = window.ns_window().unwrap() as _;
    unsafe {
        let behavior: cocoa::foundation::NSUInteger = msg_send![ns_window, collectionBehavior];
        let full_screen_aux: cocoa::foundation::NSUInteger = 1 << 8; // NSWindowCollectionBehaviorFullScreenAuxiliary
        let new_behavior = behavior | full_screen_aux;
        let _: () = msg_send![ns_window, setCollectionBehavior: new_behavior];
    }
}
```

`msg_send!` 宏来自 `objc` crate（cocoa 的传递依赖），需在 `lib.rs` 文件顶部添加：

```rust
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
```

> `NSWindowCollectionBehaviorFullScreenAuxiliary` 的值是 `1 << 8`（即 256），直接用字面量避免依赖 cocoa 的常量绑定是否导出的不确定性。

- [ ] **Step 3: 用 IsShowingFlag 替换原有失焦逻辑**

在 lib.rs 顶部添加导入：

```rust
use crate::show_guard::IsShowingFlag;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
```

在 `setup` 闭包中，将原有失焦逻辑：

```rust
let window_clone = window.clone();
window.on_window_event(move |event| {
    if let tauri::WindowEvent::Focused(false) = event {
        let _ = window_clone.hide();
    }
});
```

替换为：

```rust
let is_showing = Arc::new(AtomicBool::new(false));
app.manage(IsShowingFlag(is_showing.clone()));

let window_clone = window.clone();
let flag_for_event = is_showing.clone();
window.on_window_event(move |event| {
    match event {
        tauri::WindowEvent::Focused(false) => {
            if !flag_for_event.load(Ordering::SeqCst) {
                let _ = window_clone.hide();
            }
        }
        tauri::WindowEvent::Focused(true) => {
            flag_for_event.store(false, Ordering::SeqCst);
        }
        _ => {}
    }
});
```

- [ ] **Step 4: 验证编译通过**

Run: `cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/show_guard.rs src-tauri/src/lib.rs
git commit -m "feat: add visible_on_all_workspaces, FullScreenAuxiliary, and focus race guard"
```

---

### Task 4: 改造 tray.rs — show 前置 flag

**Files:**
- Modify: `src-tauri/src/tray.rs`

- [ ] **Step 1: 在 tray 点击事件中添加 is_showing flag**

在 `tray.rs` 的 `on_tray_icon_event` 闭包中，`window.show()` 调用之前，插入：

```rust
if let Some(flag) = app.try_state::<crate::show_guard::IsShowingFlag>() {
    flag.will_show();
}
```

完整点击逻辑变为（仅展示变更部分，原有定位逻辑保持不变）：

```rust
TrayIconEvent::Click {
    button: MouseButton::Left,
    button_state: MouseButtonState::Down,
    ..
} => {
    if let Some(window) = app.get_webview_window("calendar") {
        if let Ok(Some(tray_rect)) = tray.rect() {
            // ... 原有定位逻辑完全不变 ...
        }
        // ↓ 新增：show 前置 flag
        if let Some(flag) = app.try_state::<crate::show_guard::IsShowingFlag>() {
            flag.will_show();
        }
        let _ = window.show();
        let _ = window.set_focus();
    }
}
```

- [ ] **Step 2: 验证编译通过**

Run: `cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check`
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/tray.rs
git commit -m "feat: set is_showing flag before window.show() in tray click"
```

---

### Task 5: 集成测试 — 手动验证

本 Bug 修复涉及 macOS 窗口管理行为，无法用单元测试覆盖，需手动验证。

- [ ] **Step 1: 启动开发构建**

Run: `cd /Users/eskyfun/develop/private_project/cc-day && pnpm tauri dev`
Expected: 应用启动，托盘图标显示日期

- [ ] **Step 2: 验证常规桌面行为**

1. 在常规桌面（无全屏应用）点击托盘图标
2. 预期：日历窗口正常显示在托盘图标下方
3. 点击窗口外部（桌面或其他应用）
4. 预期：窗口正常隐藏

- [ ] **Step 3: 验证全屏应用下行为**

1. 打开 iTerm2 或 VSCode，进入全屏模式（Ctrl+Cmd+F）
2. 在全屏应用内点击顶部状态栏的 CC-Day 托盘图标
3. 预期：日历窗口正常浮现在全屏应用上方
4. 点击窗口外部（回到全屏应用）
5. 预期：窗口正常隐藏

- [ ] **Step 4: 验证窗口无闪现/消失**

在全屏应用下多次快速点击托盘图标（开→关→开）
预期：每次都能正常显示/隐藏，无闪现或卡死现象

- [ ] **Step 5: 最终 Commit（如有遗漏修复）**

```bash
git add -A
git commit -m "fix: minor adjustments from manual testing"
```
