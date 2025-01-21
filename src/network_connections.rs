use std::process::Command;

pub fn gather_network_connections() -> String {
    let mut report = String::new();
    report.push_str("<h2>Network Connections</h2>\n<table border='1'>\n");
    report.push_str("<tr><th>Local Address</th><th>Foreign Address</th><th>Status</th></tr>\n");

    let output = Command::new("netstat").arg("-an").output().expect("Failed to execute netstat");
    let netstat_output = String::from_utf8_lossy(&output.stdout);

    for line in netstat_output.lines().skip(4) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts[3] != "CLOSE_WAIT" {
            report.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                parts[1], parts[2], parts[3]
            ));
        }
    }
    report.push_str("</table>\n");
    report
}
