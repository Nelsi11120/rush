use std::{
    io::{BufReader, Read},
    path::Path,
};

use anyhow::{Result, bail};
use openssl::hash::{Hasher, MessageDigest};
use std::fs::File;

use crate::hashers::utils::{Digest, DigestCompatibleHasher};
pub use rs_merkle::algorithms::Sha256 as Sha256Algorithm;

impl DigestCompatibleHasher for Sha256Algorithm {
    fn to_digest(hash: Self::Hash) -> Digest {
        Digest::D32(hash)
    }
    fn from_digest(digest: &Digest) -> Result<Self::Hash> {
        match digest {
            Digest::D32(arr) => Ok(*arr),
            _ => bail!("Expected 32-byte digest for Sha256"),
        }
    }

    fn zero_digest() -> Digest {
        Digest::D32([0u8; 32])
    }
}

pub fn sha256_hash_file(path: &Path, bytes_to_hash: u64, buffer_size: usize) -> Result<Digest> {
    let file = File::open(path)?;
    let mut hasher = Hasher::new(MessageDigest::sha256())?;
    if bytes_to_hash > 0 {
        let taker = file.take(bytes_to_hash);
        let mut reader = BufReader::with_capacity(buffer_size, taker);
        std::io::copy(&mut reader, &mut hasher)?;
    } else {
        let mut reader = BufReader::with_capacity(buffer_size, file);
        std::io::copy(&mut reader, &mut hasher)?;
    }

    let digest = hasher.finish()?;
    Ok(Digest::D32((*digest).try_into()?))
}
