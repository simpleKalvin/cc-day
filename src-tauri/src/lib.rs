mod tray;
mod icon;
mod show_guard;

#[cfg(target_os = "macos")]
use objc2::msg_send;

use chrono::{Datelike, Local, Timelike};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

use crate::icon::{generate_date_icon, icon_to_tauri_image};
use crate::show_guard::IsShowingFlag;

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
            .visible_on_all_workspaces(true)
            .build()?;

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
