mod config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Humara pyara config manager load ho raha hai app launch hone se pehle!
            println!("[Lifecycle] App launch hone wala hai. Config check kar rahe hain...");
            match config::init_config(app) {
                Ok(cfg) => {
                    println!("[Lifecycle] Config load ho gayi: {:?}", cfg);
                }
                Err(e) => {
                    eprintln!("[Lifecycle] Error: {}", e);
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
