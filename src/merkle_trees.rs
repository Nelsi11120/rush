use anyhow::Result;
use rayon::prelude::*;
use rs_merkle::MerkleTree;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fs::File;
use std::io::BufWriter;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{HashMethod, md5_hasher::Md5Algorithm};

#[derive(Serialize, Deserialize)]
struct Leaf {
    name: String,
    #[serde(with = "hex::serde")]
    content_hash: [u8; 16],
}

#[derive(Serialize, Deserialize)]
struct Node {
    name: String,
    hash_method: &'static str,
    #[serde(with = "hex::serde")]
    root_hash: [u8; 16],
    children: Vec<Leaf>,
    bytes_to_hash: u64,
}

fn store_node_to_disk(node: &Node, path: &Path) -> Result<()> {
    // TODO: make mirroring and better path handling
    let file_path = path.join("merkle.json");
    let file = BufWriter::new(File::create(&file_path)?);
    serde_json::to_writer(file, node)?;
    Ok(())
}

pub fn build_merkle_tree(path: &Path, method: &HashMethod, bytes_to_hash: u64) -> Result<[u8; 16]> {
    let hash_method = method.hash_method();

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
                name: p.to_string_lossy().into_owned(),
                content_hash: hash,
            })
        })
        .collect::<Result<Vec<Leaf>>>()?;

    let children_hash: Vec<[u8; 16]> = children.iter().map(|c| c.content_hash).collect();
    let merkle_tree = MerkleTree::<Md5Algorithm>::from_leaves(&children_hash);
    let root = merkle_tree.root().unwrap_or([0u8; 16]);

    let node = Node {
        name: path.to_string_lossy().into_owned(),
        hash_method: method.as_str(),
        root_hash: root,
        children,
        bytes_to_hash,
    };

    let _ = store_node_to_disk(&node, path);

    Ok(root)
}
