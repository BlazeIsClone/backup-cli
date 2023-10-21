use crate::remote_tmpfs::remote_tmpfs_create;
use crate::sftp_write_local::sftp_write_local;
use crate::tcp_session::tcp_session;
use dialoguer::Input;
use std::io::Read;

pub fn init(
    remote_hostname: &str,
    remote_username: &str,
    local_dir: &str,
    private_key_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let filesystem_path: String = Input::new()
        .with_prompt("Filesystem path to backup")
        .with_initial_text("/var/www/")
        .interact()
        .unwrap();

    match tcp_session(remote_hostname, remote_username, private_key_path) {
        Ok(session) => {
            println!("SSH session established successfully.");

            let remote_file_id = remote_tmpfs_create();

            let source_dir = &filesystem_path;
            let output_zip = format!("/var/tmp/backup-cli-{}-files.zip", &remote_file_id);

            let zip_cmd = format!("cd {source_dir} && zip -r {output_zip} .");

            let mut channel = session.channel_session()?;

            channel.exec(&zip_cmd)?;

            // Read and print the command's output
            let mut output = String::new();
            channel.read_to_string(&mut output)?;
            println!("{}", output);

            let _ = sftp_write_local(&session, &output_zip, &local_dir, "files.zip");

            // Remove temp file
            let rm_cmd = format!("rm -f /var/tmp/backup-cli-{}-files.zip", &remote_file_id);
            println!("Removing temp file: {}", &output_zip);
            session.channel_session()?.exec(&rm_cmd)?;
        }
        Err(err) => eprintln!("SSH session error: {}", err),
    }

    Ok(())
}
