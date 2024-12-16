#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

static QWM_FOLDER: &str = "qwm_src";
static POSTGRES_SCRIPT: &str = "init.sql";
static NSSM_EXECUTABLE: &str = "nssm.exe";
static QWM_SERVICE_NAME: &str = "QWM";

fn get_installer_dir() -> Result<PathBuf, String> {
    env::current_exe()
        .map_err(|e| format!("Failed to locate current directory: {}", e))
        .and_then(|path| {
            path.parent()
                .map(PathBuf::from)
                .ok_or("Failed to resolve parent directory".into())
        })
}

#[tauri::command]
fn copy_qwm_folder(target_path: String) -> Result<String, String> {
    let base_dir = get_installer_dir()?;
    let source_path = base_dir.join(QWM_FOLDER);
    let target_dir = Path::new(&target_path);

    if !source_path.exists() {
        return Err("QWM Ordner nicht gefunden, bitte in den selben Ordner packen wie den Installer!".into());
    }

    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    copy_dir_all(&source_path, &target_dir).map_err(|e| e.to_string())?;

    Ok("QWM folder copied successfully!".into())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            // Only copy directories that contain "QWM" in their name
            if src_path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| !name.starts_with("migrations"))
                .unwrap_or(false)
            {
                fs::create_dir_all(&dst_path)?;
                copy_dir_all(&src_path, &dst_path)?;
            }
        } else {
            if src_path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name == "postgres_installer.exe")
                .unwrap_or(false)
            {
                continue;
            }
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}


#[tauri::command]
fn install_postgres(installation_path: String) -> Result<String, String> {
    let base_dir = get_installer_dir()?;
    let postgres_installer = base_dir.join(QWM_FOLDER).join("postgres_installer.exe");

    if !postgres_installer.exists() {
        return Err(format!(
            "PostgreSQL installer not found: {}",
            postgres_installer.display()
        ));
    }

    let output = Command::new(&postgres_installer)
        .args([
            "--mode", "unattended",
            "--superpassword", "qwm",
            "--servicename", "postgresql", 
            "--serverport", "5432",
            "--datadir", &format!("{}\\data", installation_path),
        ])
        .output()
        .map_err(|e| format!("Failed to execute PostgreSQL installer: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "PostgreSQL installation failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok("PostgreSQL installed successfully.".into())
}

#[tauri::command]
fn configure_database(installation_path: String) -> Result<String, String> {
    let base_dir = get_installer_dir()?;
    let sql_script_path = base_dir.join(QWM_FOLDER).join("migrations").join(POSTGRES_SCRIPT);

    let psql_path = r"C:\Program Files\PostgreSQL\15\bin\psql.exe";

    if !sql_script_path.exists() {
        return Err(format!(
            "SQL script not found at path: {}",
            sql_script_path.display()
        ));
    }

    let superuser = "postgres";
    let password = "qwm";

    let output = Command::new(psql_path)
        .env("PGPASSWORD", password) 
        .args([
            "-U", superuser,                  
            "-d", "postgres",                   
            "-f", &sql_script_path.to_string_lossy(),
        ])
        .output()
        .map_err(|e| format!("Failed to execute SQL script: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "SQL script execution failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok("Database configured successfully.".to_string())
}

#[tauri::command]
fn configure_service(installation_path: String) -> Result<String, String> {
    let base_dir = get_installer_dir()?;
    let nssm_path = base_dir.join(QWM_FOLDER).join("QWM").join(NSSM_EXECUTABLE);
    let start_bat_path = Path::new(&installation_path).join("QWM").join("start.bat");

    if !nssm_path.exists() {
        return Err(format!("NSSM executable not found: {}", nssm_path.display()));
    }

    if !start_bat_path.exists() {
        return Err(format!(
            "start.bat not found in the QWM folder: {}",
            start_bat_path.display()
        ));
    }

    Command::new(&nssm_path)
        .args(["install", QWM_SERVICE_NAME, &start_bat_path.to_string_lossy()])
        .spawn()
        .map_err(|e| format!("Failed to configure service using NSSM: {}", e))?
        .wait()
        .map_err(|e| format!("NSSM tool encountered an issue during configuration: {}", e))?;

    Ok("Service configured successfully.".into())
}

#[tauri::command]
fn start_service(installation_path: String) -> Result<String, String> {
    let base_dir = get_installer_dir()?;
    let batch_file_path = base_dir.join(QWM_FOLDER).join("start_service.bat");

    if !batch_file_path.exists() {
        return Err(format!(
            "The batch file 'start_service.bat' was not found: {}",
            batch_file_path.display()
        ));
    }

    Command::new("cmd")
        .args(["/C", batch_file_path.to_string_lossy().as_ref()])
        .spawn()
        .map_err(|e| format!("Failed to execute the batch file: {}", e))?
        .wait()
        .map_err(|e| format!("Batch file execution encountered an issue: {}", e))?;

    Ok("Service started successfully.".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            copy_qwm_folder,
            install_postgres,
            configure_database,
            configure_service,
            start_service
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
