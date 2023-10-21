use ssh2::Session;
use std::error::Error;
use std::net::TcpStream;
use std::path::Path;

pub fn tcp_session(
    remote_hostname: &str,
    remote_username: &str,
    private_key_path: &str,
) -> Result<Session, Box<dyn Error>> {
    // SSH connection information
    let port = 22;
    let current_username = whoami::username();
    // Connect to the SSH server
    let tcp = TcpStream::connect((remote_hostname, port))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // Authenticate with the SSH server using the private key
    session.userauth_pubkey_file(
        &remote_username,
        None,
        Path::new(&format!(
            "/home/{}/.ssh/{}",
            &current_username, &private_key_path
        )),
        None,
    )?;

    Ok(session)
}
