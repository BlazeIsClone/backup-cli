use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use ssh2::Session;

pub fn sftp_write_local(
    session: &Session,
    remote_file: &str,
    local_dir: &str,
    local_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize an SFTP session
    let sftp = session.sftp()?;

    let remote_path = Path::new(&remote_file);

    // Open a remote file for reading
    let mut remote_file = sftp.open(&remote_path)?;

    // Create a local file for writing
    let mut local_file = File::create(format!("{}/{}", &local_dir, &local_filename))?;

    let file_size = sftp
        .stat(Path::new(&remote_path))?
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
    pb.finish_with_message("Downloaded successfully.");

    Ok(())
}
