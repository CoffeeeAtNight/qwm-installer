use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

static QWM_FOLDER: &str = "qwm_src";
static POSTGRES_SCRIPT: &str = "migrations/init.sql";
static NSSM_EXECUTABLE: &str = "nssm.exe";

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
    let source_path = base_dir.join(QWM_FOLDER).join("QWM");
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
            fs::create_dir_all(&dst_path)?;
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

#[tauri::command]
fn install_postgres(installation_path: String) -> Result<String, String> {
    let base_dir = get_installer_dir()?;
    let postgres_installer = base_dir.join("qwm_src/postgres_installer.exe");

    // Run the PostgreSQL installer silently
    Command::new(postgres_installer)
        .args(["--mode", "unattended"])
        .spawn()
        .map_err(|e| format!("Failed to run PostgreSQL installer: {}", e))?
        .wait()
        .map_err(|e| format!("PostgreSQL installation failed: {}", e))?;

    let psql_path = "C:\\Program Files\\PostgreSQL\\bin\\psql.exe";
    let init_script = base_dir.join(POSTGRES_SCRIPT);

    Command::new(psql_path)
        .args(["-U", "postgres", "-f", &init_script.to_string_lossy()])
        .spawn()
        .map_err(|e| format!("Failed to execute SQL script: {}", e))?
        .wait()
        .map_err(|e| format!("SQL script execution failed: {}", e))?;

    Ok("PostgreSQL installed and database configured.".into())
}

#[tauri::command]
fn configure_service(installation_path: String) -> Result<String, String> {
    let base_dir = get_installer_dir()?;
    let nssm_path = base_dir.join("qwm_src").join(NSSM_EXECUTABLE);

    Command::new(nssm_path)
        .spawn()
        .map_err(|e| format!("Failed to start NSSM tool: {}", e))?
        .wait()
        .map_err(|e| format!("NSSM tool encountered an issue: {}", e))?;

    Ok("Service configured successfully.".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            copy_qwm_folder,
            install_postgres,
            configure_service
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
