use anyhow::{Result, bail};
use openssl::hash::{Hasher, MessageDigest};
use rs_merkle::Hasher as Mh;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::hashers::utils::{Digest, DigestCompatibleHasher};

#[derive(Clone)]
pub struct Md5Algorithm {}

impl Mh for Md5Algorithm {
    type Hash = [u8; 16];

    fn hash(data: &[u8]) -> Self::Hash {
        // TODO: error handling
        let mut hasher =
            Hasher::new(MessageDigest::md5()).expect("OpenSSL should be available at runtime");
        hasher.update(data).expect("Md5 Update failed");
        let digest = hasher.finish().expect("Md5 finish failed");
        (*digest).try_into().expect("MD5 should be 16 bytes")
    }
}

impl DigestCompatibleHasher for Md5Algorithm {
    fn to_digest(hash: Self::Hash) -> Digest {
        Digest::D16(hash)
    }

    fn from_digest(digest: &Digest) -> Result<Self::Hash> {
        match digest {
            Digest::D16(arr) => Ok(*arr),
            _ => bail!("Expected 16-byte digest for Blake3"),
        }
    }

    fn zero_digest() -> Digest {
        Digest::D16([0u8; 16])
    }
}

pub fn md5_hash_file(path: &Path, bytes_to_hash: u64, buffer_size: usize) -> Result<Digest> {
    let file = File::open(path)?;
    let mut hasher = Hasher::new(MessageDigest::md5())?;

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

    let digest = hasher.finish()?;
    Ok(Digest::D16((*digest).try_into()?))
}
