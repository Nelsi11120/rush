use anyhow::Result;
use clap::ValueEnum;
use std::{fmt, path::Path};

use crate::hashers::md5::md5_hash_file;

#[derive(Default, Clone, ValueEnum, Debug)]
pub enum HashMethod {
    #[clap(alias = "md5")]
    #[default]
    Md5,
}

impl HashMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HashMethod::Md5 => "md5",
        }
    }
}

impl fmt::Display for HashMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            HashMethod::Md5 => "md5",
        })
    }
}

pub fn hash_file(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
    buffer_size: usize,
) -> Result<[u8; 16]> {
    match method {
        HashMethod::Md5 => {
            let hash = md5_hash_file(path, bytes_to_hash, buffer_size)?;
            Ok(hash)
        }
    }
}
