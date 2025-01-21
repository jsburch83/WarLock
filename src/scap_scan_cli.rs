use std::process::Command;
use std::path::Path;

/// Runs the SCAP scan using the command-line tool with all content enabled and returns the results as a formatted string.
pub fn run_scap_scan_with_all_content() -> String {
    let scap_tool_path = r"C:\SCAP_Tool\cscc.exe"; // Path to the SCAP CLI tool

    if Path::new(scap_tool_path).exists() {
        println!("Running SCAP scan with all content enabled...");

        let output = Command::new(scap_tool_path)
            .arg("--enableAllRun")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let result = String::from_utf8_lossy(&output.stdout);
                    println!("SCAP scan completed successfully.");
                    return format!(
                        "<h2>SCAP Scan Report</h2>\n<pre>{}</pre>\n",
                        result.replace("\n", "<br>")
                    );
                } else {
                    let error_output = String::from_utf8_lossy(&output.stderr);
                    eprintln!("SCAP scan failed: {}", error_output);
                    return format!(
                        "<h2>SCAP Scan Report</h2>\n<p style='color:red;'>SCAP scan failed: {}</p>\n",
                        error_output.replace("\n", "<br>")
                    );
                }
            }
            Err(e) => {
                let err_msg = format!("Error executing SCAP scan: {}", e);
                eprintln!("{}", err_msg);
                return format!(
                    "<h2>SCAP Scan Report</h2>\n<p style='color:red;'>{}</p>\n",
                    err_msg
                );
            }
        }
    } else {
        let err_msg = format!("SCAP scanner executable not found at {}", scap_tool_path);
        eprintln!("{}", err_msg);
        return format!(
            "<h2>SCAP Scan Report</h2>\n<p style='color:red;'>{}</p>\n",
            err_msg
        );
    }
}

/// Sets the source and destination directories for SCAP scan results
pub fn configure_scap_scan(source_dir: &str, destination_dir: &str) {
    let scap_tool_path = r"C:\SCAP_Tool\cscc.exe";

    if Path::new(scap_tool_path).exists() {
        println!("Configuring SCAP scan...");

        let set_source = Command::new(scap_tool_path)
            .arg("--setOpt")
            .arg("summarySourceDirectory")
            .arg(source_dir)
            .output();

        if let Err(e) = set_source {
            eprintln!("Error setting source directory: {}", e);
            return;
        }

        let set_dest = Command::new(scap_tool_path)
            .arg("--setOpt")
            .arg("summaryDestinationDirectory")
            .arg(destination_dir)
            .output();

        if let Err(e) = set_dest {
            eprintln!("Error setting destination directory: {}", e);
            return;
        }

        println!("SCAP scan directories configured successfully.");
    } else {
        eprintln!("SCAP scanner executable not found at {}", scap_tool_path);
    }
}
