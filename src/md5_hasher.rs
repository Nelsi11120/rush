use anyhow::Result;
use openssl::hash::{Hasher, MessageDigest};
use rs_merkle::Hasher as Mh;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Clone)]
pub struct Md5Algorithm {}

impl Mh for Md5Algorithm {
    type Hash = [u8; 16];

    fn hash(data: &[u8]) -> Self::Hash {
        let mut hasher =
            Hasher::new(MessageDigest::md5()).expect("OpenSSL should be available at runtime");
        hasher.update(data);
        let digest = hasher.finish().expect("Md5 update failed");
        (*digest).try_into().expect("MD5 should be 16 bytes")
    }
}

pub fn md5_hash_file(path: &Path, bytes_to_hash: u64, buffer_size: usize) -> Result<[u8; 16]> {
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new(MessageDigest::md5())?;

    if bytes_to_hash > 0 {
        // Hash only up to bytes_to_hash
        // Read at most bytes_to_hash bytes
        let mut taker = file.take(bytes_to_hash);
        let mut reader = BufReader::with_capacity(buffer_size, taker);
        std::io::copy(&mut reader, &mut hasher)?
    } else {
        let mut reader = BufReader::with_capacity(buffer_size, file);
        std::io::copy(&mut reader, &mut hasher)?
    };

    let digest = hasher.finish()?;
    Ok((*digest).try_into()?)
}
