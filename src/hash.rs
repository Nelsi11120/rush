use crate::{HashMethod, md5_hasher::md5_hash_file};
use anyhow::Result;
use std::path::Path;

pub fn hash_file(path: &Path, method: &HashMethod, bytes_to_hash: u64) -> Result<[u8; 16]> {
    match method {
        HashMethod::Md5 => {
            let hash = md5_hash_file(path, bytes_to_hash)?;
            Ok(hash)
        }
    }
}
