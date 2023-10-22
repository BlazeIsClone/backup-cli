# Backup CLI
Straightforward command line interface for downloading backup snapshots of MySQL Databases and Application File Systems written in Rust Programming Language.

### Design
The "Backup CLI" aims to bridge compatibility issues that stem from connecting clients and servers. This project was inspired by a philosophical concern, MySQL clients often face limited support when interfacing with remote MySQL servers that are on different versions. To combat this all the backup tasks are executed using native Unix commands, with backup files transported via SFTP.

### Host Client Requirements
SSH private key should be present within the default directory `/home/{user}/.ssh`.


### Remote Server Requirements
The client's openssh public key should be authorized by the user. Zip package should be installed for archiving files before transporting via network.

### Installation & Usage (Unix-based Systems)

Download one of the pre-built binaries found in https://github.com/BlazeIsClone/backup-cli/releases/

Move the binary to /usr/local/bin (Unix-based Systems):
```bash
sudo mv backup-cli /usr/local/bin/
```
Make the binary executable:
```bash
sudo chmod +x /usr/local/bin/backup-cli
```

Verify installation & usage:
```bash
backup-cli
```

### Building From Source

In case you want to compile this yourself, you need to install at minimum the following dependencies:

* Rust stable and Cargo
* Make, CMake and a C compiler

Download the source code:
```bash
git clone https://github.com/BlazeIsClone/backup-cli.git
```

Build (change working directory to `/path/to/backup-cli`):
```bash
cargo build --release
```

The compiled output should be inside `/path/to/backup-cli/target/release`.
