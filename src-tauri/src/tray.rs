use chrono::{Datelike, Local};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Runtime,
};

#[cfg(not(target_os = "macos"))]
use tauri::Manager;

#[cfg(target_os = "macos")]
use tauri_nspanel::{ManagerExt, NSPoint};
#[cfg(target_os = "macos")]
use tauri_nspanel::objc2;
#[cfg(target_os = "macos")]
use tauri_nspanel::objc2::MainThreadMarker;
#[cfg(target_os = "macos")]
use tauri_nspanel::objc2_app_kit::NSScreen;

use crate::icon::{generate_date_icon, icon_to_tauri_image};

/// Position a calendar NSPanel centered below the tray icon.
///
/// Iterates `NSScreen.screens` to find the display containing the tray icon,
/// so the panel appears correctly on multi-monitor setups (instead of always
/// computing coordinates relative to the primary display).
#[cfg(target_os = "macos")]
fn position_panel_below_tray<R: Runtime>(
    panel: &tauri_nspanel::PanelHandle<R>,
    tray_rect: tauri::Rect,
    panel_width: f64,
    panel_height: f64,
    y_offset: f64,
) {
    let phys_pos = tray_rect.position.to_physical::<f64>(1.0);
    let phys_size = tray_rect.size.to_physical::<f64>(1.0);
    let tray_x_px = phys_pos.x;
    let tray_y_px = phys_pos.y;
    let tray_w_px = phys_size.width;
    let tray_h_px = phys_size.height;

    let mtm = MainThreadMarker::new().expect("must be on main thread");

    let main_screen = NSScreen::mainScreen(mtm).expect("no main screen");
    let main_frame = main_screen.frame();
    let main_scale = main_screen.backingScaleFactor();

    let mut screen_height_pts = main_frame.size.height;
    let mut screen_scale = main_scale;
    let mut screen_origin_y = main_frame.origin.y;

    for screen in NSScreen::screens(mtm).iter() {
        let f = screen.frame();
        let scale = screen.backingScaleFactor();
        // tray_x_px is already in global native points — don't divide by scale
        if tray_x_px >= f.origin.x && tray_x_px <= f.origin.x + f.size.width {
            screen_height_pts = f.size.height;
            screen_scale = scale;
            screen_origin_y = f.origin.y;
            break;
        }
    }

    // Position coords are in native points; size is in physical pixels — convert to pts
    let tray_x_pts = tray_x_px;
    let tray_y_pts = tray_y_px;
    let tray_w_pts = tray_w_px / screen_scale;
    let tray_h_pts = tray_h_px / screen_scale;

    // DEBUG: log all coordinate values to diagnose multi-screen Y offset
    eprintln!(
        "[tray-debug] tray_px=({:.1}, {:.1}) tray_size_px=({:.1}, {:.1})",
        tray_x_px, tray_y_px, tray_w_px, tray_h_px
    );
    eprintln!(
        "[tray-debug] matched_screen: height_pts={:.1} scale={:.1} origin_y={:.1}",
        screen_height_pts, screen_scale, screen_origin_y
    );
    eprintln!(
        "[tray-debug] tray_pts=({:.1}, {:.1}) tray_size_pts=({:.1}, {:.1})",
        tray_x_pts, tray_y_pts, tray_w_pts, tray_h_pts
    );
    for (i, screen) in NSScreen::screens(mtm).iter().enumerate() {
        let f = screen.frame();
        eprintln!(
            "[tray-debug] screen[{}]: origin=({:.1}, {:.1}) size=({:.1}, {:.1}) scale={:.1}",
            i, f.origin.x, f.origin.y, f.size.width, f.size.height,
            screen.backingScaleFactor()
        );
    }

    let x = (tray_x_pts + tray_w_pts / 2.0) - panel_width / 2.0;
    let y = screen_height_pts + screen_origin_y - (tray_y_pts + tray_h_pts + y_offset) - panel_height;

    eprintln!(
        "[tray-debug] computed panel origin: x={:.1} y={:.1}",
        x, y
    );

    unsafe {
        let _: () = objc2::msg_send![panel.as_panel(), setFrameOrigin: NSPoint::new(x, y)];
    }
}

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
                            position_panel_below_tray(&panel, tray_rect, 320.0, 420.0, 4.0);
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
                            position_panel_below_tray(&panel, tray_rect, 320.0, 420.0, 4.0);
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
                        match app.get_webview_panel("calendar") {
                        Ok(panel) => {
                            if let Ok(Some(tray_rect)) = tray.rect() {
                                position_panel_below_tray(&panel, tray_rect, 320.0, 420.0, 4.0);
                            }
                            panel.show_and_make_key();
                        }
                        Err(e) => {
                            eprintln!("[tray] get_webview_panel ERR: {:?}", e);
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
