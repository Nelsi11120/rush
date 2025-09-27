use std::path::Path;

use anyhow::{Ok, Result};

use crate::hashers::utils::{HashMethod, hash_file};

pub(crate) fn invoke(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
    buffer_size: usize,
) -> Result<()> {
    let hash_root = {
        if path.is_file() {
            hash_file(path, method, bytes_to_hash, buffer_size)?
        } else {
            anyhow::bail!("Path is not a file: {}", path.display());
        }
    };
    print!("{}", hex::encode(hash_root));
    Ok(())
}
