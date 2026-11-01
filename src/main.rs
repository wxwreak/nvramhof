use std::process::Command;

mod hof;

fn main() -> std::io::Result<()> {
    let output = Command::new("efibootmgr")
        .arg("-v")
        .output()
        .expect("Failed to execute efibootmgr");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let current_id = stdout
        .lines()
        .find(|line| line.starts_with("BootCurrent:"))
        .map(|line| line.replace("BootCurrent:", "").trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let mut clean_entries = Vec::new();

    for line in stdout.lines() {
        if let Some((prefix, full_name)) = line.split_once('*')
            && (prefix.starts_with("Boot00") || prefix.starts_with("Boot01"))
        {
            let id_str = prefix.replace("Boot", "").trim().to_string();
            let full_name = full_name.trim();

            let clean_name = if let Some((name, _)) = full_name.split_once("VenHw") {
                name.trim()
            } else if let Some((name, _)) = full_name.split_once("HD(") {
                name.trim()
            } else if let Some((name, _)) = full_name.split_once("BBS(") {
                name.trim()
            } else if let Some((name, _)) = full_name.split_once("PciRoot") {
                name.trim()
            } else {
                full_name
            };

            clean_entries.push(hof::CleanBootEntry {
                id: id_str,
                name: clean_name.to_string(),
            });
        }
    }

    match hof::gen_nfo_f(&clean_entries, &current_id) {
        Ok(_) => println!("Generated NFO file"),
        Err(e) => eprintln!("Unknown error while writing: {}", e),
    }
    Ok(())
}
