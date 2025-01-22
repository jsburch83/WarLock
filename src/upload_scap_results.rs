use quick_xml::Reader;
use quick_xml::events::Event;
use std::fs::{File, self};
use std::io::{BufReader, Write, Read};
use std::path::{Path};
use std::time::SystemTime;
use std::fs::metadata;
use reqwest::blocking::Client;
use dotenv::dotenv;
use std::env;
use std::io::{self, stdin};

pub fn upload_non_compliant_results() {
    dotenv().ok();
    let base_path = Path::new(r"C:\\Users\\Administrator\\SCC\\Sessions");
    if !base_path.exists() {
        eprintln!("SCC Sessions directory not found at: {}", base_path.display());
        return;
    }

    let latest_session = fs::read_dir(base_path)
        .expect("Unable to read SCC Sessions directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .max_by_key(|entry| metadata(entry.path()).unwrap().modified().unwrap_or(SystemTime::UNIX_EPOCH));

    if let Some(session) = latest_session {
        let scap_xml_path = session.path().join("Results/SCAP/XML");
        if scap_xml_path.exists() {
            println!("Do you want to upload SCAP results? (yes/no): ");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read input");
            if input.trim().to_lowercase() == "yes" {
                match fs::read_dir(&scap_xml_path) {
                    Ok(entries) => {
                        for entry in entries.filter_map(|e| e.ok()) {
                            if let Some(file_name) = entry.file_name().to_str() {
                                if file_name.contains("XCCDF-Results") {
                                    let file_path = entry.path();
                                    println!("Found results file: {:?}", file_path);
                                    let chunk_files = split_xml_by_tag(&file_path, "RuleResult");
                                    for chunk in chunk_files {
                                        upload_file_to_api(&chunk);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading XML directory: {}", e),
                }
            } else {
                println!("SCAP results upload skipped.");
            }
        } else {
            eprintln!("SCAP XML results directory not found.");
        }
    } else {
        eprintln!("No session directories found.");
    }
}

fn split_xml_by_tag(file_path: &Path, tag: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut xml_reader = Reader::from_reader(reader);

    let mut chunks = Vec::new();
    let mut buffer = Vec::new();
    let mut inside_tag = false;
    let mut part = 1;

    loop {
        match xml_reader.read_event(&mut buffer) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == tag.as_bytes() => {
                inside_tag = true;
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == tag.as_bytes() => {
                inside_tag = false;
                let chunk_filename = format!("{}_part{}.xml", file_path.display(), part);
                let mut output_file = File::create(&chunk_filename).expect("Unable to create file");
                output_file.write_all(&buffer).expect("Unable to write to file");
                chunks.push(chunk_filename);
                buffer.clear();
                part += 1;
            }
            Ok(Event::Eof) => break,
            Ok(_) => {
                if inside_tag {
                    buffer.extend_from_slice(xml_reader.buffer());
                }
            }
            Err(e) => eprintln!("Error parsing XML: {:?}", e),
        }
        buffer.clear();
    }

    chunks
}

fn upload_file_to_api(file_path: &Path) {
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found");
    let client = Client::new();
    let url = "https://api.openai.com/v1/files";

    if let Ok(mut file) = File::open(file_path) {
        let mut content = Vec::new();
        if file.read_to_end(&mut content).is_ok() {
            let response = client.post(url)
                .bearer_auth(api_key)
                .body(content)
                .send();
            
            match response {
                Ok(resp) => println!("File successfully uploaded to API. Status: {}", resp.status()),
                Err(e) => eprintln!("Failed to upload file to API: {}", e),
            }
        } else {
            eprintln!("Failed to read the file content.");
        }
    } else {
        eprintln!("Unable to open the SCAP results file.");
    }
}
