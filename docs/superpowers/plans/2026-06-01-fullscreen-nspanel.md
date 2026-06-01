# Fullscreen Popup Fix via tauri-nspanel v2.1 — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace manual objc2 NSWindow manipulation with tauri-nspanel v2.1 NSPanel API to fix calendar popup not appearing over fullscreen apps on macOS.

**Architecture:** `PanelBuilder` creates an NSPanel subclass directly (no `WebviewWindowBuilder` → `to_panel()` conversion). `window_did_resign_key` notification triggers auto-hide (replaces `Focused(false)`). `nonactivating_panel()` style mask + `full_screen_auxiliary()` collection behavior handle fullscreen overlays. All unsafe objc2 code removed.

**Tech Stack:** tauri 2.x (with `macos-private-api`), tauri-nspanel v2.1 (git dep), Rust 2021 edition

---

## File Structure

| File | Responsibility |
|------|---------------|
| `src-tauri/Cargo.toml` | Dependencies: add tauri-nspanel, `macos-private-api` feature; remove objc2 |
| `src-tauri/src/lib.rs` | App setup: plugin registration, panel creation (macOS) or window creation (fallback), midnight icon thread |
| `src-tauri/src/tray.rs` | Tray icon: click handling with `get_webview_panel()` on macOS |
| `src-tauri/src/show_guard.rs` | **DELETE** — replaced by panel resign_key event |

---

### Task 1: Update Cargo.toml Dependencies

**Files:**
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: Add `macos-private-api` to tauri features and add tauri-nspanel, remove objc2**

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon", "macos-private-api"] }
tauri-plugin-opener = "2"
tauri-nspanel = { git = "https://github.com/ahkohd/tauri-nspanel", branch = "v2.1" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
image = "0.25"
chrono = "0.4"
```

Note: tauri-nspanel is a regular dependency (not macOS-only) so `.plugin(tauri_nspanel::init())` works on all platforms. The crate provides stub implementations on non-macOS. Remove the existing `[target.'cfg(target_os = "macos")'.dependencies]` block that contains `objc2 = "0.5"` — objc2 is now pulled in transitively through tauri-nspanel.

- [ ] **Step 2: Verify Cargo.toml parses correctly**

Run: `cargo read-manifest --manifest-path src-tauri/Cargo.toml 2>&1 || head -1`
Expected: No parse errors. (cargo-read-manifest may error if not installed; `cat src-tauri/Cargo.toml` as fallback to verify syntax)

---

### Task 2: Rewrite lib.rs — Panel Creation & App Setup

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Replace the entire file content**

```rust
mod tray;
mod icon;

use chrono::{Datelike, Local, Timelike};
use std::thread;
use std::time::Duration;
use tauri::{Manager, WebviewUrl};

#[cfg(target_os = "macos")]
use tauri_nspanel::{
    tauri_panel, CollectionBehavior, PanelBuilder, PanelLevel, StyleMask,
};

#[cfg(not(target_os = "macos"))]
use tauri::WebviewWindowBuilder;

use crate::icon::{generate_date_icon, icon_to_tauri_image};

#[cfg(target_os = "macos")]
tauri_panel! {
    panel!(CalendarPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
    })
    panel_event!(CalendarPanelEventHandler {
        window_did_resign_key(notification: &NSNotification) -> ()
    })
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_nspanel::init())
        .invoke_handler(tauri::generate_handler![get_app_version])
        .setup(|app| {
            tray::create_tray(app.handle())?;

            #[cfg(target_os = "macos")]
            {
                let panel = PanelBuilder::<_, CalendarPanel>::new(app, "calendar")
                    .url(WebviewUrl::App("index.html".into()))
                    .title("CC-Day")
                    .size(tauri::Size::Logical(tauri::LogicalSize {
                        width: 320.0,
                        height: 420.0,
                    }))
                    .level(PanelLevel::Floating)
                    .collection_behavior(
                        CollectionBehavior::new()
                            .full_screen_auxiliary()
                            .can_join_all_spaces(),
                    )
                    .style_mask(StyleMask::empty().nonactivating_panel())
                    .hides_on_deactivate(false)
                    .no_activate(true)
                    .with_window(|w| w.decorations(false).resizable(false).visible(false))
                    .build()?;

                let handler = CalendarPanelEventHandler::new();
                let window_ref = panel.to_window().unwrap();
                handler.window_did_resign_key(move |_notification| {
                    let _ = window_ref.hide();
                });
                panel.set_event_handler(Some(handler.as_ref()));
            }

            #[cfg(not(target_os = "macos"))]
            {
                WebviewWindowBuilder::new(app, "calendar", WebviewUrl::App("index.html".into()))
                    .title("CC-Day")
                    .inner_size(320.0, 420.0)
                    .decorations(false)
                    .always_on_top(true)
                    .visible(false)
                    .resizable(false)
                    .visible_on_all_workspaces(true)
                    .build()?;
            }

            let app_handle = app.handle().clone();
            thread::spawn(move || loop {
                let now = Local::now();
                let secs_until_midnight =
                    ((24 - now.hour()) * 3600 - now.minute() * 60 - now.second()) as u64 + 60;
                thread::sleep(Duration::from_secs(secs_until_midnight));

                if let Some(tray) = app_handle.tray_by_id("main") {
                    let day = Local::now().day();
                    let icon_img = generate_date_icon(day);
                    let icon = icon_to_tauri_image(&icon_img);
                    let _ = tray.set_icon(Some(icon));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2: Verify it compiles (macOS target)**

Run: `cd src-tauri && cargo check 2>&1`
Expected: Compilation succeeds. If there are errors about missing types, verify Cargo.toml changes from Task 1 were applied.

---

### Task 3: Rewrite tray.rs — Simplify Tray Click

**Files:**
- Modify: `src-tauri/src/tray.rs`

- [ ] **Step 1: Replace the entire file content**

```rust
use chrono::{Datelike, Local};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, Runtime,
};

use crate::icon::{generate_date_icon, icon_to_tauri_image};

#[cfg(target_os = "macos")]
use tauri_nspanel::ManagerExt;

pub fn create_tray<R: Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let today_day = Local::now().day();
    let icon_img = generate_date_icon(today_day);
    let icon = icon_to_tauri_image(&icon_img);

    let menu = MenuBuilder::new(app)
        .item(&MenuItemBuilder::with_id("nav_settings", "偏好").build(app)?)
        .item(&MenuItemBuilder::with_id("nav_about", "版本").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("quit", "退出").build(app)?)
        .build()?;

    TrayIconBuilder::with_id("main")
        .icon(icon)
        .tooltip("CC-Day 农历日历")
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "nav_settings" => {
                let _ = app.emit("navigate-to", "settings");
            }
            "nav_about" => {
                let _ = app.emit("navigate-to", "about");
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            let app = tray.app_handle();
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Down,
                    ..
                } => {
                    #[cfg(target_os = "macos")]
                    {
                        if let Ok(panel) = app.get_webview_panel("calendar") {
                            if let Ok(Some(tray_rect)) = tray.rect() {
                                let pos: tauri::PhysicalPosition<f64> =
                                    tray_rect.position.to_physical(1.0);
                                let size: tauri::PhysicalSize<f64> =
                                    tray_rect.size.to_physical(1.0);
                                let window_width = 320.0;
                                let x = (pos.x as f64 + size.width as f64 / 2.0)
                                    - window_width / 2.0;
                                let y = pos.y as f64 + size.height as f64 + 4.0;
                                if let Some(window) = panel.to_window() {
                                    let _ = window.set_position(
                                        tauri::Position::Physical(
                                            tauri::PhysicalPosition::new(x as i32, y as i32),
                                        ),
                                    );
                                }
                            }
                            panel.show_and_make_key();
                        }
                    }

                    #[cfg(not(target_os = "macos"))]
                    {
                        if let Some(window) = app.get_webview_window("calendar") {
                            if let Ok(Some(tray_rect)) = tray.rect() {
                                let pos: tauri::PhysicalPosition<f64> =
                                    tray_rect.position.to_physical(1.0);
                                let size: tauri::PhysicalSize<f64> =
                                    tray_rect.size.to_physical(1.0);
                                let window_width = 320.0;
                                let x = (pos.x as f64 + size.width as f64 / 2.0)
                                    - window_width / 2.0;
                                let y = pos.y as f64 + size.height as f64 + 4.0;
                                let _ = window.set_position(tauri::Position::Physical(
                                    tauri::PhysicalPosition::new(x as i32, y as i32),
                                ));
                            }
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cd src-tauri && cargo check 2>&1`
Expected: Compilation succeeds.

---

### Task 4: Delete show_guard.rs

**Files:**
- Delete: `src-tauri/src/show_guard.rs`

- [ ] **Step 1: Delete the file**

Run: `rm src-tauri/src/show_guard.rs`

- [ ] **Step 2: Verify compilation still passes**

Run: `cd src-tauri && cargo check 2>&1`
Expected: Compilation succeeds. The `mod show_guard;` declaration and `use crate::show_guard::IsShowingFlag;` were already removed in Task 2's lib.rs rewrite.

---

### Task 5: Build and Smoke Test

- [ ] **Step 1: Full release build**

Run: `pnpm tauri build 2>&1 | tail -20`
Expected: Build succeeds with exit code 0. Binary produced at `src-tauri/target/release/bundle/`.

- [ ] **Step 2: Manual smoke test checklist**

Launch the app and verify:
1. Tray icon appears in menu bar
2. Click tray icon → calendar panel appears below menu bar
3. Click away (another app or desktop) → panel hides
4. Open VSCode/iTerm2 in fullscreen → click tray icon → panel appears over fullscreen app
5. Rapid click tray icon → no crash, panel shows/hides correctly

- [ ] **Step 3: Commit all changes as a single commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/lib.rs src-tauri/src/tray.rs
git add -u src-tauri/src/show_guard.rs
git commit -m "$(cat <<'EOF'
fix: replace manual objc2 NSWindow manipulation with tauri-nspanel v2.1

Use PanelBuilder with full_screen_auxiliary + can_join_all_spaces
collection behaviors to fix calendar popup not appearing over
fullscreen apps (VSCode, iTerm2) on macOS. Remove all unsafe objc2
code, show_guard race condition workaround, and manual
activateIgnoringOtherApps/setLevel/makeKeyAndOrderFront calls.

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
EOF
)"
```