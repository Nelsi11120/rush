use anyhow::Ok;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Leaf {
    pub name: String,
    #[serde(with = "hex::serde")]
    pub content_hash: [u8; 16],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub name: String,
    pub hash_method: String,
    #[serde(with = "hex::serde")]
    pub root_hash: [u8; 16],
    pub children: Vec<Leaf>,
    pub bytes_to_hash: u64,
}

pub fn node_from_file(path: &Path) -> Result<Node> {
    // Open the file in read-only mode with buffer.
    let reader = BufReader::new(File::open(path)?);

    // Deserialize JSON contents of the files as 'Node'
    let node = serde_json::from_reader(reader)?;
    Ok(node)
}
