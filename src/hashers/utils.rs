use crate::hashers::{blake3::blake3_hash_file, md5::md5_hash_file};
use anyhow::{Ok, Result};
use clap::ValueEnum;
use hex::{FromHex, decode_to_slice};
use rs_merkle::Hasher;
use std::{fmt, path::Path};

#[derive(Default, Clone, ValueEnum, Debug)]
pub enum HashMethod {
    #[clap(alias = "md5")]
    #[default]
    Md5,
    Blake3,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Digest {
    D16([u8; 16]),
    D32([u8; 32]),
}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        match self {
            Digest::D16(b) => b,
            Digest::D32(b) => b,
        }
    }
}

impl FromHex for Digest {
    type Error = anyhow::Error;
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let hex_bytes = hex.as_ref();
        // Divide by 2 since each byte is 2 hex chars
        match hex_bytes.len() / 2 {
            // https://docs.rs/hex/latest/src/hex/lib.rs.html#206
            16 => {
                let mut out = [0u8; 16];
                decode_to_slice(hex, &mut out as &mut [u8])?;
                Ok(Digest::D16(out))
            }
            32 => {
                let mut out = [0u8; 32];
                decode_to_slice(hex, &mut out as &mut [u8])?;
                Ok(Digest::D32(out))
            }
            _ => Err(anyhow::anyhow!(
                "Invalid hex string length: {} bytes",
                hex_bytes.len() / 2
            )),
        }
    }
}

impl HashMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HashMethod::Md5 => "md5",
            HashMethod::Blake3 => "blake3",
        }
    }
}

impl fmt::Display for HashMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            HashMethod::Md5 => "md5",
            HashMethod::Blake3 => "blake3",
        })
    }
}

pub fn hash_file(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
    buffer_size: usize,
) -> Result<Digest> {
    match method {
        HashMethod::Md5 => Ok(md5_hash_file(path, bytes_to_hash, buffer_size)?),
        HashMethod::Blake3 => Ok(blake3_hash_file(path, bytes_to_hash, buffer_size)?),
    }
}

pub trait DigestCompatibleHasher: Hasher {
    fn to_digest(hash: Self::Hash) -> Digest;
    fn from_digest(digest: &Digest) -> Result<Self::Hash>;
    fn zero_digest() -> Digest;
}
