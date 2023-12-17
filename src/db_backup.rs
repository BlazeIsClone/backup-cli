use crate::remote_tmpfs::remote_tmpfs_create;
use crate::sftp_write_local::sftp_write_local;
use crate::tcp_session::tcp_session;
use dialoguer::Input;

pub fn init(
    remote_hostname: &str,
    remote_username: &str,
    local_dir: &str,
    private_key_path: &str,
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

    let remote_file_id = remote_tmpfs_create();
    let mysql_output = format!("/tmp/backup-cli-{}-database.sql", &remote_file_id);

    let mysqldump_cmd = format!(
        "mysqldump --quick --single-transaction --host={} --port={} --user={} --password={} --databases {} > {}",
        &mysql_host, &mysql_port, &mysql_user, &mysql_password, &mysql_databases, &mysql_output
    );

    match tcp_session(remote_hostname, remote_username, private_key_path) {
        Ok(session) => {
            println!("SSH session established successfully.");

            session.channel_session()?.exec(&mysqldump_cmd)?;

            let _ = sftp_write_local(&session, &mysql_output, &local_dir, "database.sql");

            println!("Removing temp file: {}", &mysql_output);
            let rm_cmd = format!("rm -f /tmp/backup-cli-{}-database.sql", &remote_file_id);

            session.channel_session()?.exec(&rm_cmd)?;
        }

        Err(err) => eprintln!("SSH session error: {}", err),
    }

    Ok(())
}
