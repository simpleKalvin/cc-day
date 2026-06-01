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
                let panel = PanelBuilder::<_, CalendarPanel>::new(app.handle(), "calendar")
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
                    .build()
                    .expect("PanelBuilder build failed");
                eprintln!("[DEBUG] Panel created successfully, label=calendar");

                let handler = CalendarPanelEventHandler::new();
                let panel_ref = panel.clone();
                handler.window_did_resign_key(move |_notification| {
                    eprintln!("[DEBUG] window_did_resign_key fired, is_visible={}", panel_ref.is_visible());
                    if panel_ref.is_visible() {
                        panel_ref.hide();
                    }
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
