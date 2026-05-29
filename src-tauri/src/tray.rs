use chrono::{Datelike, Local};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, Runtime,
};

use crate::icon::{generate_date_icon, icon_to_tauri_image};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let today_day = Local::now().day();
    let icon_img = generate_date_icon(today_day);
    let icon = icon_to_tauri_image(&icon_img);

    let menu = MenuBuilder::new(app)
        .item(&MenuItemBuilder::with_id("theme_ink_wash", "淡墨水彩").build(app)?)
        .item(&MenuItemBuilder::with_id("theme_morandi", "莫兰迪雅粉").build(app)?)
        .item(&MenuItemBuilder::with_id("theme_palace", "赤金宫墙").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("quit", "退出").build(app)?)
        .build()?;

    TrayIconBuilder::with_id("main")
        .icon(icon)
        .tooltip("CC-Day 农历日历")
        .icon_as_template(true)
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "theme_ink_wash" => {
                    let _ = app.emit("theme-change", "ink-wash");
                }
                "theme_morandi" => {
                    let _ = app.emit("theme-change", "morandi");
                }
                "theme_palace" => {
                    let _ = app.emit("theme-change", "palace");
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            let app = tray.app_handle();
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    if let Some(window) = app.get_webview_window("calendar") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
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
