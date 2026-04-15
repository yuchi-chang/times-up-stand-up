use std::fs;
use tauri::{AppHandle, Manager, PhysicalPosition};
use tauri_plugin_store::StoreExt;

use crate::timer::TimerMutex;

#[tauri::command]
pub async fn show_reminder(app: AppHandle) -> Result<(), String> {
    let app_clone = app.clone();
    app.run_on_main_thread(move || {
        let _ = show_reminder_window(app_clone);
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_timer_settings(app: AppHandle) -> Result<(), String> {
    let store = app
        .store("settings.json")
        .map_err(|e| e.to_string())?;

    let interval_minutes = store
        .get("interval_minutes")
        .and_then(|v| v.as_u64())
        .unwrap_or(30);

    let state = app.state::<TimerMutex>();
    let mut timer = state.lock().unwrap();
    timer.update_interval(interval_minutes * 60);

    Ok(())
}

#[tauri::command]
pub fn load_gif_base64(app: AppHandle) -> Result<String, String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    let gif_path = store
        .get("gif_path")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    if gif_path.is_empty() {
        return Ok(String::new());
    }

    let bytes = fs::read(&gif_path).map_err(|e| format!("Failed to read GIF '{}': {}", gif_path, e))?;
    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
}

#[tauri::command]
pub async fn show_pet(app: AppHandle) -> Result<(), String> {
    let app_clone = app.clone();
    app.run_on_main_thread(move || {
        let _ = show_pet_window(app_clone);
    })
    .map_err(|e| e.to_string())
}

pub fn show_pet_window(app: AppHandle) -> tauri::Result<()> {
    if app.get_webview_window("pet").is_some() {
        return Ok(());
    }

    let mut builder = tauri::WebviewWindowBuilder::new(
        &app,
        "pet",
        tauri::WebviewUrl::App("pet.html".into()),
    )
    .title("Desktop Pet")
    .inner_size(200.0, 170.0)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false);

    #[cfg(target_os = "windows")]
    {
        builder = builder.transparent(true).shadow(false);
    }

    builder.build()?;

    Ok(())
}

pub fn show_reminder_window(app: AppHandle) -> tauri::Result<()> {
    let win_width: f64 = 340.0;
    let win_height: f64 = 220.0;

    // If reminder window already exists, reuse it
    if let Some(existing) = app.get_webview_window("reminder") {
        // Reposition and show
        if let Ok(Some(monitor)) = app.primary_monitor() {
            let screen_size = monitor.size();
            let screen_pos = monitor.position();
            let scale = monitor.scale_factor();
            let x = screen_pos.x + screen_size.width as i32
                - (win_width * scale) as i32
                - (16.0 * scale) as i32;
            let y = screen_pos.y + screen_size.height as i32
                - (win_height * scale) as i32
                - (48.0 * scale) as i32;
            let _ = existing.set_position(PhysicalPosition::new(x, y));
        }
        let _ = existing.show();
        let _ = existing.set_focus();
        // Reload the page to reset animation and timer
        let _ = existing.eval("window.location.reload()");
        return Ok(());
    }

    // Calculate position BEFORE creating window
    let position = app
        .primary_monitor()
        .ok()
        .flatten()
        .map(|monitor| {
            let screen_size = monitor.size();
            let screen_pos = monitor.position();
            let scale = monitor.scale_factor();

            let x = screen_pos.x + screen_size.width as i32
                - (win_width * scale) as i32
                - (16.0 * scale) as i32;
            let y = screen_pos.y + screen_size.height as i32
                - (win_height * scale) as i32
                - (48.0 * scale) as i32;

            PhysicalPosition::new(x, y)
        });

    let mut builder = tauri::WebviewWindowBuilder::new(
        &app,
        "reminder",
        tauri::WebviewUrl::App("reminder.html".into()),
    )
    .title("Stand Up!")
    .inner_size(win_width, win_height)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .focused(true);

    if let Some(pos) = position {
        builder = builder.position(pos.x as f64, pos.y as f64);
    }

    let _window = builder.build()?;

    Ok(())
}
