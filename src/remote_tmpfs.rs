use rand::distributions::{Alphanumeric, DistString};

pub fn remote_tmpfs_create() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 8)
}
