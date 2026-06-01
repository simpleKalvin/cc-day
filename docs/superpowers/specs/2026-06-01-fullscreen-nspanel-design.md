# Fullscreen Popup Fix via tauri-nspanel v2.1 — Design Spec

> **Status**: Approved | **Date**: 2026-06-01 | **Skill**: brainstorming

## Goal

Fix the calendar popup not appearing when other apps (VSCode, iTerm2) are in fullscreen mode on macOS, by replacing manual objc2-based NSWindow manipulation with the `tauri-nspanel` v2.1 plugin.

## Approach: Full Rewrite (Plan B)

Replace all manual objc2 NSWindow manipulation with tauri-nspanel's NSPanel API. Use `PanelBuilder` for window creation, `panel_event!` for focus/blur handling, and native panel style masks for correct fullscreen behavior — eliminating all unsafe objc2 code.

## Architecture

```
┌──────────────────────────────────────┐
│ tauri::Builder                       │
│  .plugin(tauri_nspanel::init())      │
│  .setup(|app| {                      │
│    tray::create_tray(app);           │
│    create_calendar_panel(app);  ←── PanelBuilder + CalendarPanel
│  })                                  │
└──────────────────────────────────────┘
```

**Key changes vs current code:**
- `PanelBuilder` replaces `WebviewWindowBuilder` — creates an NSPanel subclass directly
- `panel_event!` handles `window_did_resign_key` for auto-hide — replaces unreliable `Focused(false)`
- `nonactivating_panel()` style mask eliminates need for `activateIgnoringOtherApps`
- `full_screen_auxiliary()` + `can_join_all_spaces()` collection behaviors enable fullscreen overlay
- `show_guard.rs` module removed — NSPanel resign-key events don't have the show/hide race condition

## Panel Definition

```rust
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
```

## Window Creation

```rust
let panel = PanelBuilder::<_, CalendarPanel>::new(app, "calendar")
    .url(WebviewUrl::App("index.html".into()))
    .title("CC-Day")
    .size(Size::Logical(LogicalSize { width: 320.0, height: 420.0 }))
    .level(PanelLevel::Floating)
    .collection_behavior(
        CollectionBehavior::new()
            .full_screen_auxiliary()
            .can_join_all_spaces()
    )
    .style_mask(StyleMask::empty().nonactivating_panel())
    .hides_on_deactivate(false)
    .no_activate(true)
    .with_window(|w| w.decorations(false).resizable(false).visible(false))
    .build()?;
```

- `PanelLevel::Floating` (NSFloatingWindowLevel=3) is sufficient with `full_screen_auxiliary()`
- `nonactivating_panel()` means panel clicks don't activate the app (no Dock icon jump)
- `no_activate(true)` prevents stealing focus on creation

## Event Handling (Auto-Hide)

```rust
let handler = CalendarPanelEventHandler::new();
let window_clone = panel.to_window().unwrap();

handler.window_did_resign_key(move |_notification| {
    let _ = window_clone.hide();
});

panel.set_event_handler(Some(handler.as_ref()));
```

`window_did_resign_key` is a system-level NSNotification — more reliable than Tauri's `Focused(false)` which can fire spuriously during show/hide transitions.

## Tray Click (Show)

```rust
if let Ok(panel) = app.get_webview_panel("calendar") {
    // Position under menu bar icon (same logic as before)
    if let Ok(Some(tray_rect)) = tray.rect() {
        // ... compute x, y ...
        let _ = panel.to_window().unwrap().set_position(...);
    }
    panel.show_and_make_key();
}
```

Removes: `activateIgnoringOtherApps`, manual `setLevel`, `makeKeyAndOrderFront`, `IsShowingFlag`.

## Files Changed

| File | Action | Description |
|------|--------|-------------|
| `src-tauri/Cargo.toml` | Modify | Add tauri-nspanel dep; add `macos-private-api` feature to tauri; remove objc2 |
| `src-tauri/src/lib.rs` | Rewrite | Plugin registration, PanelBuilder creation, event handler setup, midnight icon thread |
| `src-tauri/src/tray.rs` | Rewrite | Simplify tray click to `get_webview_panel()` + `show_and_make_key()` |
| `src-tauri/src/show_guard.rs` | **Delete** | No longer needed — panel events handle race condition |

## What Gets Removed

- All `#[cfg(target_os = "macos")]` unsafe blocks with `msg_send!` calls
- `use objc2::msg_send` and related imports
- `IsShowingFlag` state management (AtomicBool + Arc)
- `window.on_window_event(Focused)` handler
- `activateIgnoringOtherApps:` calls
- Manual `setLevel:` / `makeKeyAndOrderFront:` calls
- `objc2` dependency entirely

## Risk Assessment

- **tauri-nspanel requires tauri >= 2.8.5** with `macos-private-api` feature — current `tauri = "2"` resolves to latest, so this is compatible
- **`PanelBuilder` may behave slightly differently** from `WebviewWindowBuilder` for window lifecycle — mitigated by the `with_window()` escape hatch
- **Single commit rollback** — all changes committed together per user requirement