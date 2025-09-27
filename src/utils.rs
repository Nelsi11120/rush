use anyhow::Ok;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Leaf {
    pub name: String,
    #[serde(with = "hex::serde")]
    pub hash: [u8; 16],
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

pub struct BuildConfig {
    pub dataset_root: PathBuf,
    pub rush_root: PathBuf,
    pub method: &'static str,
    pub bytes_to_hash: u64,
    pub store: bool,
}

pub fn rel_path_str(root: &Path, path: &Path) -> String {
    if root == path {
        root.file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default()
    } else {
        let rel = path.strip_prefix(root).unwrap_or(path);
        rel.to_string_lossy().into_owned()
    }
}

pub fn node_from_file(path: &Path) -> Result<Node> {
    // Open the file in read-only mode with buffer.
    let reader = BufReader::new(File::open(path)?);

    // Deserialize JSON contents of the files as 'Node'
    let node = serde_json::from_reader(reader)?;
    Ok(node)
}
