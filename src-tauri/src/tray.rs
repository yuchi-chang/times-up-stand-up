use tauri::{
    include_image,
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::commands::{show_reminder_window, show_pet_window};
use crate::read_interval_minutes;
use crate::timer::TimerMutex;
use tauri_plugin_store::StoreExt;

fn read_language(app: &AppHandle) -> String {
    app.store("settings.json")
        .ok()
        .and_then(|store| store.get("language").and_then(|v| v.as_str().map(|s| s.to_string())))
        .unwrap_or_else(|| "tw".to_string())
}

fn tr(lang: &str, key: &str) -> &'static str {
    match (lang, key) {
        ("en", "pause") => "Pause",
        ("en", "resume") => "Resume",
        ("en", "reset") => "Reset Timer",
        ("en", "settings") => "Settings...",
        ("en", "test") => "Test",
        ("en", "pet_on") => "Pet Mode: On",
        ("en", "pet_off") => "Pet Mode: Off",
        ("en", "quit") => "Quit",
        ("en", "tooltip") => "Stand-up Reminder",
        ("en", "win_title") => "Stand-up Reminder - Settings",
        (_, "pause") => "暫停",
        (_, "resume") => "繼續",
        (_, "reset") => "重置計時",
        (_, "settings") => "設定...",
        (_, "test") => "測試提醒",
        (_, "pet_on") => "桌寵模式：開",
        (_, "pet_off") => "桌寵模式：關",
        (_, "quit") => "退出",
        (_, "tooltip") => "站立提醒",
        (_, "win_title") => "站立提醒 - 設定",
        _ => "?",
    }
}

pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let lang = read_language(app);
    let l = lang.as_str();

    let pause_i = MenuItem::with_id(app, "toggle_pause", tr(l, "pause"), true, None::<&str>)?;
    let reset_i = MenuItem::with_id(app, "reset", tr(l, "reset"), true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", tr(l, "settings"), true, None::<&str>)?;
    let test_i = MenuItem::with_id(app, "test", tr(l, "test"), true, None::<&str>)?;
    let pet_mode_on = app.store("settings.json")
        .ok()
        .and_then(|store| store.get("pet_mode").and_then(|v| v.as_bool()))
        .unwrap_or(false);
    let pet_i = MenuItem::with_id(app, "pet",
        if pet_mode_on { tr(l, "pet_on") } else { tr(l, "pet_off") },
        true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", tr(l, "quit"), true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&pause_i, &reset_i, &settings_i, &test_i, &pet_i, &quit_i])?;

    TrayIconBuilder::new()
        .icon(include_image!("icons/tray-icon.png"))
        .tooltip(tr(l, "tooltip"))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                open_settings_window(tray.app_handle());
            }
        })
        .on_menu_event(move |app, event| {
            let lang = read_language(app);
            let l = lang.as_str();

            match event.id.as_ref() {
                "toggle_pause" => {
                    let state = app.state::<TimerMutex>();
                    let mut timer = state.lock().unwrap();
                    let running = timer.toggle_pause();
                    let _ = pause_i.set_text(if running { tr(l, "pause") } else { tr(l, "resume") });
                    drop(timer);
                }
                "reset" => {
                    let interval = read_interval_minutes(app);
                    let state = app.state::<TimerMutex>();
                    let mut timer = state.lock().unwrap();
                    timer.update_interval(interval * 60);
                    let _ = pause_i.set_text(tr(l, "pause"));
                }
                "settings" => {
                    open_settings_window(app);
                }
                "test" => {
                    let is_pet = app.store("settings.json")
                        .ok()
                        .and_then(|store| store.get("pet_mode").and_then(|v| v.as_bool()))
                        .unwrap_or(false);
                    if is_pet {
                        let _ = show_pet_window(app.clone());
                    } else {
                        let _ = show_reminder_window(app.clone());
                    }
                }
                "pet" => {
                    if let Ok(store) = app.store("settings.json") {
                        let current = store.get("pet_mode")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let _ = store.set("pet_mode", !current);
                        let _ = store.save();
                        let _ = pet_i.set_text(if !current { tr(l, "pet_on") } else { tr(l, "pet_off") });
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

fn open_settings_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    let lang = read_language(app);
    let title = tr(lang.as_str(), "win_title");

    let _window = tauri::WebviewWindowBuilder::new(
        app,
        "settings",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title(title)
    .inner_size(480.0, 460.0)
    .resizable(false)
    .center()
    .build();
}
