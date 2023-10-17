use dialoguer::Input;
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;

use crate::sftp_write_local::sftp_write_local;

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
    session.userauth_pubkey_file(&remote_username, None, Path::new(&private_key_path), None)?;

    // Execute a command on the remote server
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
