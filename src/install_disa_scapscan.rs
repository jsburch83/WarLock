use std::process::Command;
use std::path::Path;

pub fn check_and_install_scap() {
    let scap_tool_path = r"C:\Program Files\SCAP Scanner\scapscanner.exe"; // Path where SCAP scanner is usually installed
    let scap_installer_dir = "src/scc-5.10.1_Windows"; // Directory containing the SCAP installation files
    let scap_installer_path = format!("{}/SCC_5.10.1_Windows_Setup.exe", scap_installer_dir);

    if Path::new(scap_tool_path).exists() {
        println!("SCAP Scanner is already installed.");
    } else {
        println!("SCAP Scanner not found. Installing from {}", scap_installer_dir);

        if Path::new(&scap_installer_path).exists() {
            let output = Command::new(&scap_installer_path)
                .arg("/VERYSILENT")
                .arg("/SUPPRESSMSGBOXES")
                .arg("/SP-")
                .arg("/DIR=C:\\SCAP_Tool")
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("SCAP Scanner installed successfully.");
                    } else {
                        eprintln!("SCAP Scanner installation failed: {:?}", output);
                    }
                }
                Err(e) => {
                    eprintln!("Error running the installer: {}", e);
                }
            }
        } else {
            eprintln!("SCAP installation files not found in {}", scap_installer_dir);
        }
    }
}
