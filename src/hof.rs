// src/hof.rs
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct CleanBootEntry {
    pub id: String,
    pub name: String,
}

pub fn gen_nfo_f(entries: &[CleanBootEntry], current_id: &str) -> std::io::Result<()> {
    let mut nfo = String::new();
    let boot_mode = if Path::new("/sys/firmware/efi").exists() {
        "UEFI"
    } else {
        "Legacy"
    };

    nfo.push_str("Made by github.com/wxwreak\n\n");
    nfo.push_str(
        " ┼──────────────────────────────────────────────────────────────────────────────────┼\n",
    );
    nfo.push_str(
        " │    ███▄    █ ██▒   █▓ ██▀███   ▄▄▄       ███▄ ▄███▓    ██░ ██  ▒█████    █████▒  │\n",
    );
    nfo.push_str(
        " │    ██ ▀█   █▓██░   █▒▓██ ▒ ██▒▒████▄    ▓██▒▀█▀ ██▒   ▓██░ ██▒▒██▒  ██▒▓██   ▒   │\n",
    );
    nfo.push_str(
        " │   ▓██  ▀█ ██▒▓██  █▒░▓██ ░▄█ ▒▒██  ▀█▄  ▓██    ▓██░   ▒██▀▀██░▒██░  ██▒▒████ ░   │\n",
    );
    nfo.push_str(
        " │   ▓██▒  ▐▌██▒ ▒██ █░░▒██▀▀█▄  ░██▄▄▄▄██ ▒██    ▒██    ░▓█ ░██ ▒██   ██░░▓█▒  ░   │\n",
    );
    nfo.push_str(
        " │   ▒██░   ▓██░  ▒▀█░  ░██▓ ▒██▒ ▓█   ▓██▒▒██▒   ░██▒   ░▓█▒░██▓░ ████▓▒░░▒█░      │\n",
    );
    nfo.push_str(
        " │   ░ ▒░   ▒ ▒   ░ ▐░  ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░░ ▒░   ░  ░    ▒ ░░▒░▒░ ▒░▒░▒░  ▒ ░      │\n",
    );
    nfo.push_str(
        " │   ░ ░░   ░ ▒░  ░ ░░    ░▒ ░ ▒░  ▒   ▒▒ ░░  ░      ░    ▒ ░▒░ ░  ░ ▒ ▒░  ░        │\n",
    );
    nfo.push_str(
        " │      ░   ░ ░     ░░    ░░   ░   ░   ▒   ░      ░       ░  ░░ ░░ ░ ░ ▒   ░ ░      │\n",
    );
    nfo.push_str(
        " │            ░      ░     ░           ░  ░       ░       ░  ░  ░    ░ ░            │\n",
    );
    nfo.push_str(
        " ┼──────────────────────────────────────────────────────────────────────────────────┼\n",
    );
    nfo.push_str(
        " ┼──────────────────────────────────────────────────────────────────────────────────┼\n",
    );
    nfo.push_str("                        -- NVRAM Hall Of Fame --\n\n");

    nfo.push_str(" [i] HW Report:\n");
    nfo.push_str("     ┌────────────────────────────────────────────────────────┐\n");
    nfo.push_str(&format!("     │ {:<22} : {} \n", "Boot Mode", boot_mode));
    nfo.push_str(&format!(
        "     │ {:<22} : {:2} \n",
        "Total Registered",
        entries.len()
    ));
    nfo.push_str(&format!(
        "     │ {:<22} : Boot{} \n",
        "Current Active ID", current_id
    ));
    nfo.push_str("     └────────────────────────────────────────────────────────┘\n\n");

    nfo.push_str(" [*] The HoF Leaderboard:\n");
    nfo.push_str("     ┌────────────────────────────────────────────────────────┐\n");

    let filtered_entries: Vec<&CleanBootEntry> = entries
        .iter()
        .filter(|e| {
            let name_lower = e.name.to_lowercase();
            !name_lower.starts_with("uefi:") && name_lower != "uefi os"
        })
        .collect();

    for entry in filtered_entries {
        let (prefix, status) = if entry.id == current_id {
            (">> ", "Act")
        } else {
            ("   ", "Old")
        };

        nfo.push_str(&format!(
            "     │ {}Boot{} ▒ {:.<25} ▒ [ {} ] \n",
            prefix, entry.id, entry.name, status
        ));
    }
    nfo.push_str("     └────────────────────────────────────────────────────────┘\n\n");
    nfo.push_str(
        " ┼──────────────────────────────────────────────────────────────────────────────────┼\n",
    );

    let mut file = File::create("nvram_hof.nfo")?;
    file.write_all(nfo.as_bytes())?;

    Ok(())
}
