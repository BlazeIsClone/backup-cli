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
    let mysql_db: String = Input::new()
        .with_prompt("MySQL database")
        .interact()
        .unwrap();

    let mysql_db_username: String = Input::new()
        .with_prompt("MySQL database username")
        .interact()
        .unwrap();

    let mysql_db_password: String = Input::new()
        .with_prompt("MySQL database password")
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

    let mysql_host = "localhost";
    let mysql_port = 3306;
    let mysql_user = &mysql_db_username;
    let mysql_password = &mysql_db_password;
    let mysql_databases = &mysql_db;
    let mysql_output = "/var/tmp/mysql.sql";

    let mysqldump_cmd = format!(
        "mysqldump --quick --single-transaction --host={} --port={} --user={} --password={} --databases {} > {}",
        &mysql_host, &mysql_port, &mysql_user, &mysql_password, &mysql_databases, &mysql_output
        );

    channel.exec(&mysqldump_cmd)?;

    // Read and print the command's output
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    println!("{}", output);

    let _ = sftp_write_local(&session, &mysql_output, &local_dir, "database.sql");

    Ok(())
}
