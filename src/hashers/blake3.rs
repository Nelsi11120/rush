use anyhow::{Ok, Result, bail};
use blake3::Hasher;
use rs_merkle::Hasher as Mh;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::hashers::utils::{Digest, DigestCompatibleHasher};
#[derive(Clone)]
pub struct Blake3Algorithm {}

impl Mh for Blake3Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

impl DigestCompatibleHasher for Blake3Algorithm {
    fn to_digest(hash: Self::Hash) -> Digest {
        Digest::D32(hash)
    }

    fn from_digest(digest: &Digest) -> Result<Self::Hash> {
        match digest {
            Digest::D32(arr) => Ok(*arr),
            _ => bail!("Expected 32-byte digest for Blake3"),
        }
    }

    fn zero_digest() -> Digest {
        Digest::D32([0u8; 32])
    }
}

pub fn blake3_hash_file(path: &Path, bytes_to_hash: u64, buffer_size: usize) -> Result<Digest> {
    let file = File::open(path)?;
    let mut hasher = Hasher::new();

    if bytes_to_hash > 0 {
        // Hash only up to bytes_to_hash
        // Read at most bytes_to_hash bytes
        let taker = file.take(bytes_to_hash);
        let mut reader = BufReader::with_capacity(buffer_size, taker);
        std::io::copy(&mut reader, &mut hasher)?
    } else {
        let mut reader = BufReader::with_capacity(buffer_size, file);
        std::io::copy(&mut reader, &mut hasher)?
    };

    Ok(Digest::D32(hasher.finalize().into()))
}
