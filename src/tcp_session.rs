use ssh2::Session;
use std::error::Error;
use std::net::TcpStream;
use std::path::Path;

pub fn tcp_session(
    remote_hostname: &str,
    remote_username: &str,
) -> Result<Session, Box<dyn Error>> {
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
    Ok(session)
}
