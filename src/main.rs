mod db_backup;
mod fs_backup;
mod remote_tmpfs;
mod sftp_write_local;
mod tcp_session;

use dialoguer::{Input, MultiSelect, Select};
use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_username = whoami::username();
    let private_key_dir = format!("/home/{}/.ssh", &current_username);
    let private_key_files = scan_private_key_files(&private_key_dir)?;
    let private_key_path = select_private_key(&private_key_files)?;

    let multiselected = &["Database", "File System"];

    let defaults = &[false, false];

    let selections = MultiSelect::new()
        .with_prompt("Pick Backup Options (use spacebar to select)")
        .items(&multiselected[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    let remote_hostname: String = Input::new()
        .with_prompt("Enter hostname")
        .interact()
        .unwrap();

    let remote_username: String = Input::new()
        .with_prompt("Enter username")
        .interact()
        .unwrap();

    let local_dir: String = Input::new()
        .with_prompt("Where do you want to store")
        .with_initial_text("./")
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        for selection in selections {
            if multiselected[selection] == "Database" {
                let _ = db_backup::init(
                    &remote_hostname,
                    &remote_username,
                    &local_dir,
                    &private_key_path,
                );
            }

            if multiselected[selection] == "File System" {
                let _ = fs_backup::init(
                    &remote_hostname,
                    &remote_username,
                    &local_dir,
                    &private_key_path,
                );
            }
        }
    }

    Ok(())
}

fn scan_private_key_files(private_key_dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut private_key_files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(private_key_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    private_key_files.push(name.to_owned());
                }
            }
        }
    }

    Ok(private_key_files)
}

fn select_private_key(private_key_files: &Vec<String>) -> Result<String, Box<dyn Error>> {
    if private_key_files.is_empty() {
        return Err("No private key files found in the .ssh directory".into());
    }

    let selection = Select::new()
        .with_prompt("Select the private key to use:")
        .items(&private_key_files)
        .default(0)
        .interact()?;

    let selected_private_key = &private_key_files[selection];

    Ok(selected_private_key.to_string())
}
