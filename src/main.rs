mod db_backup;
mod fs_backup;

use dialoguer::{Input, MultiSelect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        for selection in selections {
            if multiselected[selection] == "Database" {
                let _ = db_backup::init(&remote_hostname, &remote_username, &local_dir);
            }

            if multiselected[selection] == "File System" {
                let _ = fs_backup::init(&remote_hostname, &remote_username, &local_dir);
            }
        }
    }

    Ok(())
}
