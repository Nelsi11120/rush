use anyhow::Result;
use rayon::prelude::*;
use rs_merkle::MerkleTree;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{HashMethod, md5_hasher::Md5Algorithm, md5_hasher::md5_hash_file};

#[derive(Serialize, Deserialize)]
struct Leaf {
    name: OsString,
    content_hash: [u8; 16],
}

#[derive(Serialize, Deserialize)]
struct Node {
    name: OsString,
    hash_method: &'static str,
    root_hash: [u8; 16],
    children: Vec<Leaf>,
    bytes_to_hash: u64,
}

fn write_node() {
    unimplemented!()
}

pub fn build_merkle_tree(path: &Path, method: &HashMethod, bytes_to_hash: u64) -> Result<[u8; 16]> {
    let hash_method = match method {
        HashMethod::Md5 => md5_hash_file,
    };

    let mut entries: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    // Recurse in Parallel
    let children: Vec<Leaf> = entries
        .par_iter()
        .map(|p| -> Result<Leaf> {
            let mut hash = [0u8; 16];
            if p.is_file() {
                hash = hash_method(p, bytes_to_hash)?;
            } else if p.is_dir() {
                hash = build_merkle_tree(p, method, bytes_to_hash)?;
            }
            Ok(Leaf {
                name: p.as_os_str().to_os_string(),
                content_hash: hash,
            })
        })
        .collect::<Result<Vec<Leaf>>>()?;

    let children_hash: Vec<[u8; 16]> = children.iter().map(|c| c.content_hash).collect();
    let merkle_tree = MerkleTree::<Md5Algorithm>::from_leaves(&children_hash);
    let root = merkle_tree.root().unwrap_or([0u8; 16]);

    Ok(root)
}
