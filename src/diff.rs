use crate::{utils::Node, utils::node_from_file};
use anyhow::{Ok, Result, bail};
use std::path::Path;

fn node_from_path(path: &Path) -> Result<Node> {
    let node_path = path.join("merkle.json");
    let node = node_from_file(&node_path)?;
    Ok(node)
}

fn diff_children() -> Result<()> {
    unimplemented!()
}

pub fn diff(path1: &Path, path2: &Path) -> Result<()> {
    // Prepare paths by redirecting to hidden Merkle Tree folder
    let path1 = path1.join(".rush");
    let path2 = path2.join(".rush");

    let root_node1 = node_from_path(&path1)?;
    let root_node2 = node_from_path(&path2)?;

    if root_node1.root_hash == root_node2.root_hash {
        return Ok(());
    }

    // The hashes differs. We need to check what's diffrent

    // Sanity checks. Fails if meta data is not equal
    // 1. We check that we use the same hashing method
    if root_node1.hash_method != root_node2.hash_method {
        bail!(
            "Hash methods differ: {} vs {}",
            root_node1.hash_method,
            root_node2.hash_method
        )
    }
    // 2. We check that we use the same number of bytes to hash
    if root_node1.bytes_to_hash != root_node2.bytes_to_hash {
        bail!(
            "Bytes to hash differ: {} vs {}",
            root_node1.bytes_to_hash,
            root_node2.bytes_to_hash
        )
    }
    // Sanity Checks passses so there must be differences in the children
    diff_children()
}
