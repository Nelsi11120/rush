use anyhow::Result;
use md5::{Digest, Md5};
use rs_merkle::Hasher;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Clone)]
pub struct Md5Algorithm;

impl Hasher for Md5Algorithm {
    type Hash = [u8; 16];

    fn hash(data: &[u8]) -> Self::Hash {
        let mut hasher = Md5::new();
        hasher.update(data);
        <[u8; 16]>::from(hasher.finalize())
    }
}

pub fn md5_hash_file(path: &Path, bytes_to_hash: u64) -> Result<[u8; 16]> {
    let mut file = File::open(path)?;
    let mut hasher = Md5::new();

    let hash: [u8; 16] = if bytes_to_hash == 0 {
        // Hash the whole file leveraging Read/Write traits
        std::io::copy(&mut file, &mut hasher)?;
        hasher.finalize().into()
    } else {
        // Hash only up to bytes_to_hash
        // Default buffer size is 8192 bytes (8KB)
        let reader = BufReader::new(file);
        // Read at most bytes_to_hash bytes
        let mut handle = reader.take(bytes_to_hash);
        std::io::copy(&mut handle, &mut hasher)?;
        hasher.finalize().into()
    };

    Ok(hash)
}
