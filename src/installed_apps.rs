use winreg::RegKey;
use sha2::{Sha256, Digest};
use std::fs;
use std::path::{Path, PathBuf};

fn find_executable(path: &str) -> Option<PathBuf> {
    let dir_path = Path::new(path);
    if dir_path.is_dir() {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "exe" {
                        return Some(entry.path());
                    }
                }
            }
        }
    }
    None
}

fn hash_executable(exe_path: &Path) -> String {
    match fs::read(exe_path) {
        Ok(content) => {
            let mut hasher = Sha256::new();
            hasher.update(content);
            format!("{:x}", hasher.finalize())
        }
        Err(_) => "Hash Error".to_string(),
    }
}

pub fn gather_installed_apps() -> String {
    let mut report = String::new();
    report.push_str("<h2>Installed Applications</h2>\n<table border='1'>\n");
    report.push_str("<tr><th>Application Name</th><th>Install Date</th><th>Executable Hash</th></tr>\n");

    let hklm = RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let uninstall_path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";

    if let Ok(key) = hklm.open_subkey(uninstall_path) {
        for subkey_name in key.enum_keys().filter_map(Result::ok) {
            if let Ok(subkey) = key.open_subkey(&subkey_name) {
                let display_name: Result<String, _> = subkey.get_value("DisplayName");
                let install_date: Result<String, _> = subkey.get_value("InstallDate");
                let install_location: Result<String, _> = subkey.get_value("InstallLocation");

                let exe_hash = if let Ok(location) = install_location {
                    if let Some(exe_path) = find_executable(&location) {
                        hash_executable(&exe_path)
                    } else {
                        "No Executable Found".to_string()
                    }
                } else {
                    "Unknown Path".to_string()
                };

                if let Ok(name) = display_name {
                    let date = install_date.unwrap_or_else(|_| "Unknown".to_string());
                    report.push_str(&format!(
                        "<tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                        name, date, exe_hash
                    ));
                }
            }
        }
    }
    report.push_str("</table>\n");
    report
}
