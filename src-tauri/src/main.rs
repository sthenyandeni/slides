// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database_plugin;
use tauri::{CustomMenuItem, Menu, Submenu, Error};
use tauri::WindowBuilder;
use tauri::AppHandle;

fn show_preferences(app: AppHandle) -> Result<(), Error> {
    let preferences_window = WindowBuilder::new(
        &app,
        "preferences_window".to_string(),
        tauri::WindowUrl::App("/settings".into())
    )
    .title("Preferences")
    .always_on_top(true)
    .build()?;

    Ok(())
}

fn main() {
    let file_submenu = Submenu::new("File", 
        Menu::new()
            .add_item(CustomMenuItem::new("preferences", "Preferences"))
    );
    let menu = Menu::new()
        .add_submenu(file_submenu);

    tauri::Builder::default()
        .setup(move |app| {
            let main_window = WindowBuilder::new(
                app,
                "main_window".to_string(),
                tauri::WindowUrl::App("/".into())
            )
            .menu(menu)
            .title("Slides")
            .inner_size(1600.0, 1200.0)
            .build()?;

            main_window.on_menu_event(move |event| {
                match event.menu_item_id() {
                    "preferences" => {
                        // show_preferences(app);
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .plugin(database_plugin::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
