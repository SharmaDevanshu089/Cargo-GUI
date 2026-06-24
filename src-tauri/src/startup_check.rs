use std::process::Command;
use crate::error_popup;

/// **Toolchain Checker**: Check karo ki command properly installed hai ya nahi.
/// Agar chal gayi toh version nikal ke dega, varna error phekega!
pub fn get_command_version(cmd_name: &str) -> Result<String, String> {
    // Spawns process to run: cargo --version or rustc --version
    let output = Command::new(cmd_name)
        .arg("--version")
        .output()
        .map_err(|e| format!("Bhai {} command chal hi nahi rahi! Path check karo: {}", cmd_name, e))?;

    // Check karo ki command ne error toh nahi return kiya
    if !output.status.success() {
        return Err(format!("Bhai {} chal toh gayi par non-zero status return kiya.", cmd_name));
    }

    // Byte array ko readable string me badlo
    let version_str = String::from_utf8(output.stdout)
        .map_err(|e| format!("Bhai output ko string me badalne me fat gaya: {}", e))?
        .trim()
        .to_string();

    Ok(version_str)
}

/// **Asli Maal**: Rustc aur Cargo dono ke versions return karega as a tuple.
/// Isse frontend ya diagnostics me directly call kar sakte hain.
/// Returns: Ok((cargo_version, rustc_version))
pub fn get_toolchain_versions() -> Result<(String, String), String> {
    // 1. Cargo check karo
    let cargo_ver = get_command_version("cargo")?;
    // 2. Rustc check karo
    let rustc_ver = get_command_version("rustc")?;
    
    Ok((cargo_ver, rustc_ver))
}

/// **Startup Ka Rakshak**: Application launch hone se pehle toolchain check karega.
/// Agar Cargo ya Rustc gayab hain, toh crash (panic) hone se pehle popup dikhayega!
pub fn check_toolchain_on_startup() {
    println!("[Startup Check] Checking Rust toolchain (rustc & cargo)...");
    match get_toolchain_versions() {
        Ok((cargo_ver, rustc_ver)) => {
            // Wah bhai! Dono mil gaye!
            println!("[Startup Check] Badhiya! Toolchain detected successfully.");
            println!(" -> Cargo: {}", cargo_ver);
            println!(" -> Rustc: {}", rustc_ver);
        }
        Err(e) => {
            // Oho! Toolchain missing hai!
            let error_title = "Cargo GUI - Toolchain Error";
            let error_msg = format!(
                "Rust and Cargo toolchain was not found on your system!\n\n\
                Detail: {}\n\n\
                Please install Rust from https://rustup.rs and make sure 'cargo' and 'rustc' are added to your system's PATH environment variable so the GUI can work properly.",
                e
            );

            // Pop up dikhao screen par
            error_popup::show_error_popup(error_title, &error_msg);

            // App ko yahin par panic kar do!
            panic!("\n\n=== [CRITICAL ERROR] ===\n{}\n========================\n", error_msg);
        }
    }
}
