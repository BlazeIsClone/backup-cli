use crate::sftp_write_local::sftp_write_local;
use crate::tcp_session::tcp_session;
use dialoguer::Input;
use std::io::Read;

pub fn init(
    remote_hostname: &str,
    remote_username: &str,
    local_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let filesystem_path: String = Input::new()
        .with_prompt("Filesystem path to backup")
        .with_initial_text("/var/www/")
        .interact()
        .unwrap();

    let session = tcp_session(remote_hostname, remote_username)?;
    let mut channel = session.channel_session()?;

    let source_dir = &filesystem_path;
    let output_zip = "/var/tmp/files.zip";

    let zip_cmd = format!("cd {source_dir} && zip -r {output_zip} .");

    channel.exec(&zip_cmd)?;

    // Read and print the command's output
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    println!("{}", output);

    let _ = sftp_write_local(&session, &output_zip, &local_dir, "files.zip");

    Ok(())
}
