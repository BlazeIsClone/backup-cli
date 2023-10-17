use crate::sftp_write_local::sftp_write_local;
use crate::tcp_session::tcp_session;
use dialoguer::Input;
use std::io::Read;

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

    let session = tcp_session(remote_hostname, remote_username)?;

    let mut channel = session.channel_session()?;

    channel.exec(&mysqldump_cmd)?;

    // Read and print the command's output
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    println!("{}", output);

    let _ = sftp_write_local(&session, &mysql_output, &local_dir, "database.sql");

    Ok(())
}
