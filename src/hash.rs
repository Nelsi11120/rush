use std::path::Path;

use crate::{HashMethod, md5_hasher::md5_hash_file};

pub fn hash_file(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
) -> std::io::Result<[u8; 16]> {
    match method {
        HashMethod::Md5 => {
            let hash = md5_hash_file(path, bytes_to_hash)?;
            Ok(hash)
        }
        _ => unimplemented!(),
    }
}
