use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use crate::config::AppConfig;
use crate::startup_check;

// ==========================================
// 📦 Payload Structs for Frontend
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemPaths {
    pub home: String,
    pub documents: String,
    pub drive: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolchainInfo {
    #[serde(rename = "cargoVersion")]
    pub cargo_version: String,
    #[serde(rename = "rustcVersion")]
    pub rustc_version: String,
}

// ==========================================
// 🛠️ Tauri Commands for Settings
// ==========================================

/// **Command 1**: AppData folder se app.json read karke config return karo.
#[tauri::command]
pub fn get_app_config(app: tauri::AppHandle) -> Result<AppConfig, String> {
    let config_dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("AppData directory ka pata nahi chal paya: {}", e))?;
        
    let config_file_path = config_dir.join("app.json");

    if config_file_path.exists() {
        let content = fs::read_to_string(&config_file_path)
            .map_err(|e| format!("Config file read karne me error: {}", e))?;
            
        let config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| format!("JSON parsing fat gayi: {}", e))?;
            
        Ok(config)
    } else {
        Err("Config file exist nahi karti. Pehle startup check chalna chahiye.".to_string())
    }
}

/// **Command 2**: Nayi config ko app.json me save karo (Never overwrite unless saved).
#[tauri::command]
pub fn save_app_config(app: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    let config_dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("AppData directory nikalne me error: {}", e))?;
        
    let config_file_path = config_dir.join("app.json");

    // Make sure folder exist karta hai
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Folder create karne me error: {}", e))?;
    }

    let serialized_content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("JSON serialization fail ho gaya: {}", e))?;
        
    fs::write(&config_file_path, serialized_content)
        .map_err(|e| format!("Config file write karne me error: {}", e))?;

    println!("[Settings Manager] Config successfully saved to {}", config_file_path.display());
    Ok(())
}

/// **Command 3**: System paths resolve karke return karo (Home, Documents, and Root Drive).
#[tauri::command]
pub fn get_system_paths(app: tauri::AppHandle) -> Result<SystemPaths, String> {
    let home_path = app.path()
        .home_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
        
    let doc_path = app.path()
        .document_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    // Windows par C:\ aur macOS/Linux par / return karo
    let drive_path = if cfg!(target_os = "windows") {
        "C:\\".to_string()
    } else {
        "/".to_string()
    };

    Ok(SystemPaths {
        home: home_path,
        documents: doc_path,
        drive: drive_path,
    })
}

/// **Command 4**: Cargo aur Rustc ke active versions return karo.
#[tauri::command]
pub fn get_toolchain_info() -> Result<ToolchainInfo, String> {
    match startup_check::get_toolchain_versions() {
        Ok((cargo, rustc)) => {
            Ok(ToolchainInfo {
                cargo_version: cargo,
                rustc_version: rustc,
            })
        }
        Err(e) => Err(format!("Toolchain info nikalne me error: {}", e)),
    }
}

/// **Command 5**: RFD crate ka use karke native folder picker dialog kholo.
/// Agar user cancel kar de toh None return hoga, varna Selected Path.
#[tauri::command]
pub fn pick_folder() -> Option<String> {
    println!("[Folder Picker] RFD native dialog box open kar rahe hain...");
    
    let folder = rfd::FileDialog::new()
        .set_title("Select Cargo Directory")
        .pick_folder();

    match &folder {
        Some(path) => {
            println!("[Folder Picker] User selected: {}", path.display());
        }
        None => {
            println!("[Folder Picker] User cancelled selection.");
        }
    }

    folder.map(|p| p.to_string_lossy().into_owned())
}
