# macOS 全屏下日历弹窗修复 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 修复 iTerm2/VSCode 等应用全屏时，点击状态栏托盘图标日历弹窗无法显示的问题。

**Architecture:** 两层修复 — (1) 窗口构建时设置 `NSStatusWindowLevel`(25) 替代 Tauri 默认的 `NSFloatingWindowLevel`(3)，使窗口层级高于全屏应用；(2) 托盘点击时调用 `NSApp.activate(ignoringOtherApps:)` 激活应用，使窗口在另一个应用的全屏 Space 中浮现。所有改动合并为一次 commit。

**Tech Stack:** Rust, Tauri 2.0, objc2 (macOS only)

---

## 根因分析

### 已有的正确配置

| 配置项 | 状态 | 位置 |
|---|---|---|
| `LSUIElement = true` | ✅ 已有 | `src-tauri/Info.plist` |
| `visible_on_all_workspaces(true)` | ✅ 已有 | `src-tauri/src/lib.rs:42` |
| `NSWindowCollectionBehaviorFullScreenAuxiliary` (bit 8) | ✅ 已有 | `src-tauri/src/lib.rs:50-53` |
| `IsShowingFlag` 竞态保护 | ✅ 已有 | `src-tauri/src/show_guard.rs` |

### 缺失的关键配置

| 问题 | 影响 | 参考 |
|---|---|---|
| **窗口层级不够高** | `always_on_top` 仅设 `NSFloatingWindowLevel`(3)，全屏应用远高于此 | [Tauri #5566](https://github.com/tauri-apps/tauri/issues/5566), [SO](https://stackoverflow.com/questions/10905045) |
| **未激活应用** | 在另一个应用全屏 Space 中，窗口不会自动浮现，需 `activateIgnoringOtherApps:` | [SO](https://stackoverflow.com/questions/23503943), [SO](https://stackoverflow.com/questions/36205834) |

### macOS 窗口层级参考

| Level 名称 | 数值 | 说明 |
|---|---|---|
| `NSNormalWindowLevel` | 0 | 普通窗口 |
| `NSFloatingWindowLevel` | 3 | Tauri `always_on_top` 使用的层级 |
| `NSTornOffMenuWindowLevel` | 18 | — |
| `NSMainMenuWindowLevel` | 24 | 菜单栏 |
| **`NSStatusWindowLevel`** | **25** | **状态栏层级（本次修复目标）** |
| `NSPopUpMenuWindowLevel` | 101 | 弹出菜单 |
| `NSScreenSaverWindowLevel` | 1000 | 屏保 |

选择 `NSStatusWindowLevel`(25) 的理由：与状态栏同级，足以覆盖全屏应用，但不会过度遮挡系统 UI（如屏保、登录窗口）。类似 Alfred、iStat Menus 等菜单栏工具也使用此层级。

---

## 修改文件清单

| 文件 | 操作 | 职责 |
|---|---|---|
| `src-tauri/src/lib.rs` | 修改 | 窗口构建后设置 `NSStatusWindowLevel` |
| `src-tauri/src/tray.rs` | 修改 | 点击时调用 `activateIgnoringOtherApps:` |

---

### Task 1: lib.rs — 设置窗口层级为 NSStatusWindowLevel

**Files:**
- Modify: `src-tauri/src/lib.rs:45-55`

**背景：** 当前 `#[cfg(target_os = "macos")]` 块已设置 `collectionBehavior` 的 `fullScreenAuxiliary` 位。需要在此基础上追加 `setLevel:` 调用。

- [ ] **Step 1: 在 macOS cfg 块中追加 setLevel 调用**

将 `src-tauri/src/lib.rs` 中现有的 `#[cfg(target_os = "macos")]` 块（约第 45-55 行）从：

```rust
#[cfg(target_os = "macos")]
{
    use objc2::runtime::AnyObject;
    let ns_window: *mut AnyObject = window.ns_window().unwrap() as *mut AnyObject;
    unsafe {
        let behavior: usize = msg_send![ns_window, collectionBehavior];
        let full_screen_aux: usize = 1 << 8;
        let new_behavior = behavior | full_screen_aux;
        let _: () = msg_send![ns_window, setCollectionBehavior: new_behavior];
    }
}
```

改为：

```rust
#[cfg(target_os = "macos")]
{
    use objc2::runtime::AnyObject;
    let ns_window: *mut AnyObject = window.ns_window().unwrap() as *mut AnyObject;
    unsafe {
        let behavior: usize = msg_send![ns_window, collectionBehavior];
        let full_screen_aux: usize = 1 << 8;
        let new_behavior = behavior | full_screen_aux;
        let _: () = msg_send![ns_window, setCollectionBehavior: new_behavior];

        // NSStatusWindowLevel = 25，与状态栏同级，可浮现在全屏应用之上
        let status_level: i32 = 25;
        let _: () = msg_send![ns_window, setLevel: status_level];
    }
}
```

- [ ] **Step 2: 验证编译通过**

Run: `cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check`
Expected: 编译成功，无错误

---

### Task 2: tray.rs — 点击时激活应用

**Files:**
- Modify: `src-tauri/src/tray.rs:42-69`

**背景：** 当 iTerm2/VSCode 全屏时，macOS 不会自动将非活跃应用的窗口带到前台。需要在 show 之前调用 `NSApp.activate(ignoringOtherApps: true)`，让窗口在当前（全屏）Space 上浮现。

- [ ] **Step 1: 在 tray.rs 文件顶部添加 macOS 条件导入**

在 `src-tauri/src/tray.rs` 文件顶部 `use crate::...` 之后，添加：

```rust
#[cfg(target_os = "macos")]
use objc2::{class, msg_send};
```

- [ ] **Step 2: 在 tray 点击事件中添加 activate 调用**

将 `tray.rs` 中 `on_tray_icon_event` 闭包内的点击处理逻辑从：

```rust
TrayIconEvent::Click {
    button: MouseButton::Left,
    button_state: MouseButtonState::Down,
    ..
} => {
    if let Some(window) = app.get_webview_window("calendar") {
        if let Ok(Some(tray_rect)) = tray.rect() {
            let pos: tauri::PhysicalPosition<f64> = tray_rect.position.to_physical(1.0);
            let size: tauri::PhysicalSize<f64> = tray_rect.size.to_physical(1.0);
            let window_width = 320.0;
            let x = (pos.x as f64 + size.width as f64 / 2.0) - window_width / 2.0;
            let y = pos.y as f64 + size.height as f64 + 4.0;
            let _ = window.set_position(tauri::Position::Physical(
                tauri::PhysicalPosition::new(x as i32, y as i32),
            ));
        }
        if let Some(flag) = app.try_state::<crate::show_guard::IsShowingFlag>() {
            flag.will_show();
        }
        let _ = window.show();
        let _ = window.set_focus();
    }
}
```

改为：

```rust
TrayIconEvent::Click {
    button: MouseButton::Left,
    button_state: MouseButtonState::Down,
    ..
} => {
    if let Some(window) = app.get_webview_window("calendar") {
        if let Ok(Some(tray_rect)) = tray.rect() {
            let pos: tauri::PhysicalPosition<f64> = tray_rect.position.to_physical(1.0);
            let size: tauri::PhysicalSize<f64> = tray_rect.size.to_physical(1.0);
            let window_width = 320.0;
            let x = (pos.x as f64 + size.width as f64 / 2.0) - window_width / 2.0;
            let y = pos.y as f64 + size.height as f64 + 4.0;
            let _ = window.set_position(tauri::Position::Physical(
                tauri::PhysicalPosition::new(x as i32, y as i32),
            ));
        }

        // macOS: 激活应用，使窗口能浮现在全屏应用之上
        #[cfg(target_os = "macos")]
        {
            let app: *mut objc2::runtime::AnyObject = unsafe { msg_send![class!(NSApplication), sharedApplication] };
            unsafe { let _: () = msg_send![app, activateIgnoringOtherApps: true]; }
        }

        if let Some(flag) = app.try_state::<crate::show_guard::IsShowingFlag>() {
            flag.will_show();
        }
        let _ = window.show();
        let _ = window.set_focus();
    }
}
```

- [ ] **Step 3: 验证编译通过**

Run: `cd /Users/eskyfun/develop/private_project/cc-day/src-tauri && cargo check`
Expected: 编译成功，无错误

---

### Task 3: 集成测试 — 手动验证

此 Bug 涉及 macOS 窗口管理行为，无法用单元测试覆盖，需手动验证。

- [ ] **Step 1: 启动开发构建**

Run: `cd /Users/eskyfun/develop/private_project/cc-day && pnpm tauri dev`
Expected: 应用启动，托盘图标显示日期

- [ ] **Step 2: 验证常规桌面行为（非全屏）**

1. 在常规桌面（无全屏应用）点击托盘图标
2. 预期：日历窗口正常显示在托盘图标下方
3. 点击窗口外部（桌面或其他应用）
4. 预期：窗口正常隐藏
5. 再次点击托盘图标
6. 预期：窗口正常显示

- [ ] **Step 3: 验证全屏应用下行为**

1. 打开 iTerm2 或 VSCode，进入全屏模式（Ctrl+Cmd+F）
2. 鼠标移到屏幕顶部显示菜单栏
3. 点击 CC-Day 托盘图标
4. 预期：日历窗口正常浮现在全屏应用上方
5. 点击窗口外部（回到全屏应用）
6. 预期：窗口正常隐藏

- [ ] **Step 4: 验证快速点击无闪现**

在全屏应用下多次快速点击托盘图标（开→关→开）
预期：每次都能正常显示/隐藏，无闪现或卡死现象

---

### Task 4: 合并 Commit

确认所有验证通过后，将修改合并为一次 commit。

- [ ] **Step 1: 提交所有变更**

```bash
cd /Users/eskyfun/develop/private_project/cc-day
git add src-tauri/src/lib.rs src-tauri/src/tray.rs
git commit -m "fix: raise window level to NSStatusWindowLevel and activate app on tray click for fullscreen popup"
```

---

## 参考来源

- [Tauri Issue #11488](https://github.com/tauri-apps/tauri/issues/11488) — visibleOnAllWorkspaces 不在全屏应用之上
- [Tauri Issue #5566](https://github.com/tauri-apps/tauri/issues/5566) — setLevel_ 在 fullscreen 下不生效
- [SO: How to force NSWindow in front of fullscreen apps](https://stackoverflow.com/questions/10905045/how-to-force-an-nswindow-to-be-in-front-of-every-app-even-fullscreen-apps)
- [SO: Add NSWindow in front of fullscreen app](https://stackoverflow.com/questions/23503943/add-nswindow-in-front-of-fullscreen-app) — Agent 应用方案
- [SO: Allow NSPanel to float above fullscreen apps](https://stackoverflow.com/questions/36205834/allow-an-nswindow-nspanel-to-float-above-full-screen-apps)
- [SO: Why does my NSWindow not float above fullscreen apps](https://stackoverflow.com/questions/58934673/why-does-my-nswindow-not-float-above-full-screen-apps-when-created-programmatica)
- [Apple: NSWindow.Level](https://developer.apple.com/documentation/appkit/nswindow/level-swift.struct)
