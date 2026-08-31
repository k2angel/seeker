use md5;
use sha2::{Digest, Sha256};

pub fn md5sum(data: &[u8]) -> String {
    format!("{:x}", md5::compute(data))
}

pub fn sha256sum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    hex::encode(hasher.finalize())
}
