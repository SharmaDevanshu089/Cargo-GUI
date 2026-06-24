use std::process::Command;

/// **Dhoom-Dhadaka Popup**: Yeh function har platform par native error popup dikhayega!
/// Bina kisi heavy crate ya external library ke, seedhe system commands use karke.
/// Windows par powershell, Mac par AppleScript, aur Linux par zenity/kdialog.
pub fn show_error_popup(title: &str, message: &str) {
    // ==========================================
    // 🪟 Case 1: Windows
    // ==========================================
    #[cfg(target_os = "windows")]
    {
        // Windows par PowerShell ka use karke PresentationFramework se MessageBox kholte hain
        let script = format!(
            "Add-Type -AssemblyName PresentationFramework; [System.Windows.MessageBox]::Show('{}', '{}', 'OK', 'Error')",
            message.replace("'", "''"), // single quotes ko double-single quotes me escape karo
            title.replace("'", "''")
        );
        let _ = Command::new("powershell")
            .args(&["-Command", &script])
            .status();
    }

    // ==========================================
    // 🍎 Case 2: macOS
    // ==========================================
    #[cfg(target_os = "macos")]
    {
        // macOS par AppleScript (osascript) ka use karke native dialog box dikhate hain
        let script = format!(
            "display dialog \"{}\" with title \"{}\" buttons {{\"OK\"}} default button \"OK\" with icon stop",
            message.replace("\"", "\\\""), // double quotes ko escape karo
            title.replace("\"", "\\\"")
        );
        let _ = Command::new("osascript").args(&["-e", &script]).status();
    }

    // ==========================================
    // 🐧 Case 3: Linux
    // ==========================================
    #[cfg(target_os = "linux")]
    {
        // Linux par hum check karte hain agar 'zenity' ya 'kdialog' installed hai
        if Command::new("zenity").arg("--version").status().is_ok() {
            let _ = Command::new("zenity")
                .args(&[
                    "--error",
                    &format!("--title={}", title),
                    &format!("--text={}", message),
                ])
                .status();
        } else if Command::new("kdialog").arg("--version").status().is_ok() {
            let _ = Command::new("kdialog")
                .args(&["--error", message, "--title", title])
                .status();
        } else {
            // Agar Linux user ne zenity/kdialog bhi install nahi kiya, toh terminal hi sahara hai!
            eprintln!(
                "\n=== [CRITICAL ERROR] ===\nTitle: {}\nMessage: {}\n========================\n",
                title, message
            );
        }
    }
}
