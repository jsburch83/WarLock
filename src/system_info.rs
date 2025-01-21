use sysinfo::System;
use chrono::Local;
use winreg::enums::*;
use winreg::RegKey;

pub fn gather_system_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Gather CPU and memory info
    let cpu_count = sys.cpus().len();
    let cpu_name = sys.cpus().first().map_or("Unknown".to_string(), |cpu| cpu.brand().to_string());
    let total_memory = if sys.total_memory() > 1024 * 1024 {
        format!("{:.2} GB", sys.total_memory() as f64 / (1024.0 * 1024.0))
    } else {
        format!("{} MB", sys.total_memory() / 1024)
    };
    let used_memory = if sys.used_memory() > 1024 * 1024 {
        format!("{:.2} GB", sys.used_memory() as f64 / (1024.0 * 1024.0))
    } else {
        format!("{} MB", sys.used_memory() / 1024)
    };

    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Retrieve OS Name and Build Number from Windows Registry
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let os_info_path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";

    let (os_name, build_number) = if let Ok(key) = hklm.open_subkey(os_info_path) {
        let product_name: String = key.get_value("ProductName").unwrap_or_else(|_| "Unknown".to_string());
        let build: String = key.get_value("CurrentBuildNumber").unwrap_or_else(|_| "Unknown".to_string());
        (product_name, build)
    } else {
        ("Unknown OS".to_string(), "Unknown Build".to_string())
    };

    let mut report = String::new();
    report.push_str(&format!("<h1>System Information Report</h1>\n"));
    report.push_str(&format!("<p>Report Date: {}</p>\n", current_time));
    report.push_str(&format!("<p>OS Name: {}</p>\n", os_name));
    report.push_str(&format!("<p>Build Number: {}</p>\n", build_number));
    report.push_str(&format!("<p>CPU Count: {}</p>\n", cpu_count));
    report.push_str(&format!("<p>CPU Name: {}</p>\n", cpu_name));
    report.push_str(&format!("<p>Total Memory: {}</p>\n", total_memory));
    report.push_str(&format!("<p>Used Memory: {}</p>\n", used_memory));

    report
}
