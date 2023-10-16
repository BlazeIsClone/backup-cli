use dialoguer::{Input, MultiSelect};
use ssh2::Session;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let multiselected = &["Database", "File System"];

    let defaults = &[false, false];

    let selections = MultiSelect::new()
        .with_prompt("Pick Backup Options (use spacebar to select)")
        .items(&multiselected[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        for selection in selections {
            if multiselected[selection] == "Database" {
                let _ = db_backup();
            }

            if multiselected[selection] == "File System" {
                let _ = fs_backup();
            }
        }
    }

    Ok(())
}

fn fs_backup() -> Result<(), Box<dyn std::error::Error>> {
    let remote_hostname: String = Input::new()
        .with_prompt("Enter hostname")
        .interact()
        .unwrap();

    let remote_user: String = Input::new()
        .with_prompt("Enter username")
        .interact()
        .unwrap();

    let remote_dir: String = Input::new()
        .with_prompt("Filesystem path to backup")
        .with_initial_text("/var/www/")
        .interact()
        .unwrap();

    let local_dir: String = Input::new()
        .with_prompt("Where do you want to store")
        .interact()
        .unwrap();

    // SSH connection information
    let port = 22;
    let current_username = whoami::username();
    let private_key_path = format!("/home/{current_username}/.ssh/id_rsa");

    // Connect to the SSH server
    let tcp = TcpStream::connect((remote_hostname, port))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // Authenticate with the SSH server using the private key
    session.userauth_pubkey_file(&remote_user, None, Path::new(&private_key_path), None)?;

    // Execute a command on the remote server
    let mut channel = session.channel_session()?;

    let source_dir = &remote_dir;
    let output_zip = "/var/tmp/le-files.zip";

    let zip_cmd = format!("cd {source_dir} && zip -r {output_zip} .");

    channel.exec(&zip_cmd)?;

    // Read and print the command's output
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    println!("{}", output);

    // Initialize an SFTP session
    let sftp = session.sftp()?;

    // Open a remote file for reading
    let mut remote_file = sftp.open(Path::new(&output_zip))?;

    // Create a local file for writing
    let mut local_file = File::create(format!("{}/files.zip", &local_dir))?;

    let file_size = sftp
        .stat(Path::new(&output_zip))?
        .size
        .ok_or("Failed to get file size")?;

    // Create a progress bar
    let pb = indicatif::ProgressBar::new(1024);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{wide_bar}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("# "),
    );
    pb.set_length(file_size);

    // Transfer the file from remote to local with progress
    let mut total_bytes = 0;
    let mut buffer = [0; 8192];
    while let Ok(n) = remote_file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        local_file.write_all(&buffer[0..n])?;
        total_bytes += n as u64;
        pb.set_position(total_bytes);
    }

    // Finish the progress bar
    pb.finish_with_message("File downloaded successfully.");

    Ok(())
}

fn db_backup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Implement database here");

    Ok(())
}
