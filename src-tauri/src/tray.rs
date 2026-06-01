use chrono::{Datelike, Local};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Runtime,
};

#[cfg(not(target_os = "macos"))]
use tauri::Manager;

#[cfg(target_os = "macos")]
use tauri_nspanel::{ManagerExt, NSPoint, NSRect};
use tauri_nspanel::objc2_app_kit::NSScreen;

#[cfg(target_os = "macos")]
use tauri_nspanel::objc2;

use crate::icon::{generate_date_icon, icon_to_tauri_image};

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
                #[cfg(target_os = "macos")]
                if let Ok(panel) = app.get_webview_panel("calendar") {
                    if let Some(tray) = app.tray_by_id("main") {
                        if let Ok(Some(tray_rect)) = tray.rect() {
                            let pos: tauri::PhysicalPosition<f64> = tray_rect.position.to_physical(1.0);
                            let size: tauri::PhysicalSize<f64> = tray_rect.size.to_physical(1.0);
                            let window_width = 320.0;
                            let x = (pos.x as f64 + size.width as f64 / 2.0) - window_width / 2.0;
                            let tauri_y = pos.y as f64 + size.height as f64 + 4.0;
                            unsafe {
                                let screen: objc2::rc::Retained<NSScreen> = objc2::msg_send![objc2::class!(NSScreen), mainScreen];
                                let screen_frame: NSRect = objc2::msg_send![&*screen, frame];
                                let screen_height = screen_frame.size.height;
                                let y = screen_height - tauri_y - 420.0;
                                let ns_point = NSPoint::new(x, y);
                                let _: () = objc2::msg_send![panel.as_panel(), setFrameOrigin: ns_point];
                            }
                        }
                    }
                    panel.show_and_make_key();
                }
            }
            "nav_about" => {
                let _ = app.emit("navigate-to", "about");
                #[cfg(target_os = "macos")]
                if let Ok(panel) = app.get_webview_panel("calendar") {
                    if let Some(tray) = app.tray_by_id("main") {
                        if let Ok(Some(tray_rect)) = tray.rect() {
                            let pos: tauri::PhysicalPosition<f64> = tray_rect.position.to_physical(1.0);
                            let size: tauri::PhysicalSize<f64> = tray_rect.size.to_physical(1.0);
                            let window_width = 320.0;
                            let x = (pos.x as f64 + size.width as f64 / 2.0) - window_width / 2.0;
                            let tauri_y = pos.y as f64 + size.height as f64 + 4.0;
                            unsafe {
                                let screen: objc2::rc::Retained<NSScreen> = objc2::msg_send![objc2::class!(NSScreen), mainScreen];
                                let screen_frame: NSRect = objc2::msg_send![&*screen, frame];
                                let screen_height = screen_frame.size.height;
                                let y = screen_height - tauri_y - 420.0;
                                let ns_point = NSPoint::new(x, y);
                                let _: () = objc2::msg_send![panel.as_panel(), setFrameOrigin: ns_point];
                            }
                        }
                    }
                    panel.show_and_make_key();
                }
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
                        eprintln!("[DEBUG] Tray click received");
                        match app.get_webview_panel("calendar") {
                            Ok(panel) => {
                                eprintln!("[DEBUG] get_webview_panel OK");
                                if let Ok(Some(tray_rect)) = tray.rect() {
                                    let pos: tauri::PhysicalPosition<f64> =
                                        tray_rect.position.to_physical(1.0);
                                    let size: tauri::PhysicalSize<f64> =
                                        tray_rect.size.to_physical(1.0);
                                    let window_width = 320.0;
                                    let x = (pos.x as f64 + size.width as f64 / 2.0)
                                        - window_width / 2.0;
                                    let tauri_y = pos.y as f64 + size.height as f64 + 4.0;
                                    // Tauri uses top-left origin; macOS uses bottom-left origin
                                    unsafe {
                                        let screen: objc2::rc::Retained<NSScreen> =
                                            objc2::msg_send![objc2::class!(NSScreen), mainScreen];
                                        let screen_frame: NSRect =
                                            objc2::msg_send![&*screen, frame];
                                        let screen_height = screen_frame.size.height;
                                        let y = screen_height - tauri_y - 420.0;
                                        let ns_point = NSPoint::new(x, y);
                                        let _: () = objc2::msg_send![panel.as_panel(), setFrameOrigin: ns_point];
                                    }
                                }
                                panel.show_and_make_key();
                                eprintln!("[DEBUG] show_and_make_key called");
                            }
                            Err(e) => {
                                eprintln!("[DEBUG] get_webview_panel ERR: {:?}", e);
                            }
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
