#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::ffi::OsStr;
use std::path::Path;
use zip::read::ZipArchive;
use std::fs::File;
use std::io::{self, Error, ErrorKind};
use tauri::api::{shell};
use tauri::{CustomMenuItem, Manager, AppHandle, Menu, Submenu};

static SOURCE_ZIP_PATH: &str = "qwm.zip";

#[tauri::command]
fn backend_add(number: i32) -> i32 {
   println!("Backend was called with an argument: {}", number);
   number + 2
}

#[tauri::command]
fn unzip_files(app_handle: AppHandle, installation_path: String) -> Result<String, String> {
    
    let path_resolver = app_handle.path_resolver();
    
    let zip_path = path_resolver
        .resolve_resource(SOURCE_ZIP_PATH)
        .expect("failed to resolve zip path");
  
    println!("File path is: {:?}", zip_path);

    let mut zip_content = file_extractor(zip_path.to_str().unwrap()).map_err(|e| e.to_string())?;
    zip_content.extract(installation_path.clone()).map_err(|e| e.to_string())?;
    
    Ok(installation_path)
}

fn file_extractor(zip_filename: &str) -> Result<ZipArchive<File>, Error> {
    let path = Path::new(zip_filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => return Err(why)
    };

    if path.extension() == Some(OsStr::new("zip")) {
        let archive_contents = ZipArchive::new(file)?;
        Ok(archive_contents)
    } else {
        Err(io::Error::new(ErrorKind::InvalidInput, "The file is not a ZIP archive"))
    }
}

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![backend_add])
        .invoke_handler(tauri::generate_handler![unzip_files])
        .menu(
            tauri::Menu::os_default("QWM Installer").add_submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new(
                    "Online Documentation",
                    "Online Documentation",
                )
                .into()]),
            )),
        )
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Online Documentation" => {
                    let url = "https://github.com/Uninen/tauri-vue-template".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let main_window = _app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
