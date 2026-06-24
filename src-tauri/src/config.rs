use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

// ==========================================
// 🛠️ AppConfig Struct: Isme saara maal-taal rahega!
// ==========================================
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    // Jahan hum apne Cargo projects ko dhoondenge (Home Directory by default)
    #[serde(rename = "scanDirectory")]
    pub scan_directory: String,
    
    // Naye project banane ki default jagah (Documents Directory by default)
    #[serde(rename = "newProjectDirectory")]
    pub new_project_directory: String,
}

// ==========================================
// 📍 Functions: Jugaad aur Kaam ki Cheezein
// ==========================================

/// **Jugaad 1**: AppData folder ka rasta nikalne ki koshish.
/// Agar OS ne mana kar diya toh error return karenge.
fn get_config_dir(app: &tauri::App) -> Result<PathBuf, String> {
    // AppData folder path nikal rahe hain Tauri ke standard API se
    app.path()
        .app_data_dir()
        .map_err(|e| format!("Arre yaar, AppData directory ka pata nahi mila: {}", e))
}

/// **Jugaad 2**: Agar file bilkul hi gayab hai, toh default maal taiyaar karo.
/// Default me scan_directory = Home, aur newProjectDirectory = Documents.
fn get_default_config(app: &tauri::App) -> AppConfig {
    // Home directory nikalne ki koshish, fail hui toh default string (bhagwan bharose)
    let home_path = app.path()
        .home_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
        
    // Documents directory nikalne ki koshish, fail hui toh default string
    let doc_path = app.path()
        .document_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    println!("[Config Manager] Bhai koi purani file nahi mili, toh hum default paths set kar rahe hain.");
    println!(" -> Scan: {}", home_path);
    println!(" -> Projects: {}", doc_path);

    AppConfig {
        scan_directory: home_path,
        new_project_directory: doc_path,
    }
}

/// **Asli Jadugar**: Yeh function check karega app.json ko.
/// Agar pehle se hai toh chupchaap load karega (Never Overwrite).
/// Agar nahi hai toh ek dum chamchamata hua naya file bana dega!
pub fn init_config(app: &tauri::App) -> Result<AppConfig, String> {
    // 1. Pehle AppData folder ka path lo
    let config_dir = get_config_dir(app)?;
    
    // 2. Check karo ki folder exist karta hai ki nahi. Nahi karta toh bana do!
    if !config_dir.exists() {
        println!("[Config Manager] Roaming me humara folder nahi tha, toh humne bana diya.");
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Bhai directory banane me panga ho gaya: {}", e))?;
    }
    
    // 3. app.json file ka full path set karo
    let config_file_path = config_dir.join("app.json");
    
    // 4. Checking... File exist karti hai ya gayab hai?
    if config_file_path.exists() {
        // --- CASE A: Wah! Purani file mil gayi, ab ise read karo (NEVER OVERWRITE!) ---
        println!("[Config Manager] Badhiya hai! app.json pehle se maujood hai. Load kar rahe hain...");
        
        let content = fs::read_to_string(&config_file_path)
            .map_err(|e| format!("File read karne me fail ho gaye: {}", e))?;
            
        let config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Arre yaar, JSON parsing fat gayi! File kharab hai shayad: {}", e))?;
            
        println!("[Config Manager] Mubarak ho! Config bina kisi pange ke load ho gayi: {:?}", config);
        Ok(config)
    } else {
        // --- CASE B: Oho! File nahi mili, toh fresh default banayenge ---
        println!("[Config Manager] Arre, app.json nahi mili! Chalo naya banate hain...");
        
        let default_config = get_default_config(app);
        
        // Pretty formatting ke sath JSON me badlo taaki user bhi dekh sake
        let serialized_content = serde_json::to_string_pretty(&default_config)
            .map_err(|e| format!("JSON serialization fail ho gaya: {}", e))?;
            
        // File me write kar do
        fs::write(&config_file_path, serialized_content)
            .map_err(|e| format!("app.json file write karne me error: {}", e))?;
            
        println!("[Config Manager] Nayi default config file yahan banayi gayi: {}", config_file_path.display());
        Ok(default_config)
    }
}
