# Backup CLI
Straightforward command line interface for downloading backup snapshots of MySQL Databases and Application File Systems written in Rust Programming Language.

### Design
The "Backup CLI" aims to bridge compatibility issues that stem from connecting clients and servers. This project was inspired by a philosophical concern, MySQL clients often face limited support when interfacing with remote MySQL servers that are on different versions. To combat this all the backup tasks are executed using native Unix commands, with backup files transported via SFTP.

### Host Client Requirements
SSH private key should be present within the default directory `/home/{user}/.ssh`.


### Remote Server Requirements
The client's openssh public key should be authorized by the user. Zip package should be installed for archiving files before transporting via network.

