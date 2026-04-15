mod commands;
mod timer;
mod tray;

use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tokio::time::{sleep, Duration};

use commands::{show_reminder_window, show_pet_window};
use timer::TimerState;

pub fn read_interval_minutes(app: &tauri::AppHandle) -> u64 {
    app.store("settings.json")
        .ok()
        .and_then(|store| {
            store.get("interval_minutes")
                .and_then(|v: serde_json::Value| v.as_u64())
        })
        .unwrap_or(30)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "pet" {
                    return; // Let pet window close normally
                }
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .setup(|app| {
            // Read initial interval from store, default to 30 minutes
            let interval_minutes = read_interval_minutes(app.handle());

            // Initialize timer state
            let timer_state = Mutex::new(TimerState::new(interval_minutes * 60));
            app.manage(timer_state);

            // Setup system tray
            tray::setup_tray(app.handle())?;

            // Spawn background timer task
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_secs(1)).await;

                    let should_trigger = {
                        let state = app_handle.state::<timer::TimerMutex>();
                        let timer = state.lock().unwrap();
                        timer.is_expired()
                    };

                    if should_trigger {
                        let pet_mode = app_handle.store("settings.json")
                            .ok()
                            .and_then(|store| store.get("pet_mode").and_then(|v| v.as_bool()))
                            .unwrap_or(false);

                        let app_clone = app_handle.clone();
                        let _ = app_handle.run_on_main_thread(move || {
                            if pet_mode {
                                let _ = show_pet_window(app_clone);
                            } else {
                                let _ = show_reminder_window(app_clone);
                            }
                        });

                        // Reset timer for next cycle
                        let interval = read_interval_minutes(&app_handle);
                        let state = app_handle.state::<timer::TimerMutex>();
                        let mut timer = state.lock().unwrap();
                        timer.update_interval(interval * 60);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::show_reminder,
            commands::show_pet,
            commands::update_timer_settings,
            commands::load_gif_base64,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
