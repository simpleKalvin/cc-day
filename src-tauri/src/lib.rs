mod tray;
mod icon;

use chrono::{Datelike, Local, Timelike};
use std::thread;
use std::time::Duration;
use tauri::{WebviewUrl, WebviewWindowBuilder};

use crate::icon::{generate_date_icon, icon_to_tauri_image};

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_app_version])
        .setup(|app| {
            tray::create_tray(app.handle())?;

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
            .build()?;

            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(false) = event {
                    let _ = window_clone.hide();
                }
            });

            let app_handle = app.handle().clone();
            thread::spawn(move || {
                loop {
                    let now = Local::now();
                    let secs_until_midnight = ((24 - now.hour()) * 3600
                        - now.minute() * 60
                        - now.second()) as u64
                        + 60;
                    thread::sleep(Duration::from_secs(secs_until_midnight));

                    if let Some(tray) = app_handle.tray_by_id("main") {
                        let day = Local::now().day();
                        let icon_img = generate_date_icon(day);
                        let icon = icon_to_tauri_image(&icon_img);
                        let _ = tray.set_icon(Some(icon));
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
