//! Revision: 48
mod installed_apps;
mod network_connections;
mod system_info;
mod install_disa_scapscan;
mod scap_scan_cli;

use log::{info, LevelFilter};
use env_logger;
use std::fs::File;
use std::io::Write;

fn main() {
    // Initialize the logger
    env_logger::builder().filter_level(LevelFilter::Info).init();
    info!("Starting System Info Logger...");

    install_disa_scapscan::check_and_install_scap();
    scap_scan_cli::configure_scap_scan("C:\\SCAP_Tool\\input", "C:\\SCAP_Tool\\output");
    let scap_report = scap_scan_cli::run_scap_scan_with_all_content();
    

    let system_report = system_info::gather_system_info();
    let installed_apps_report = installed_apps::gather_installed_apps();
    let network_report = network_connections::gather_network_connections();

    let mut report = String::new();
    report.push_str(&system_report);
    report.push_str(&installed_apps_report);
    report.push_str(&network_report);
    report.push_str(&scap_report);

    // Write the report to an HTML file
    let mut file = File::create("system_report.html").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");

    info!("System report generated: system_report.html");
}
