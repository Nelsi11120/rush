use crate::{utils::Node, utils::node_from_file};
use anyhow::{Ok, Result, bail};
use std::collections::HashMap;
use std::path::Path;

pub struct Diff {
    pub added: Vec<String>,   // present only in rhs
    pub removed: Vec<String>, // present only in lhs
    pub changed: Vec<String>, // present in both but different
}

fn node_from_path(path: &Path) -> Result<Node> {
    let node_path = path.join("merkle.json");
    let node = node_from_file(&node_path)?;
    Ok(node)
}

fn diff_nodes(lhs: &Node, rhs: &Node) -> Result<Option<Diff>> {
    unimplemented!()
}

fn map_children(node: &Node) -> HashMap<&str, [u8; 16]> {
    let mut node_childrens = HashMap::new();
    node.children.iter().for_each(|c| {
        node_childrens.insert(c.name.as_str(), c.content_hash);
    });

    node_childrens
}

pub fn diff(path_l: &Path, path_r: &Path) -> Result<(Option<Diff>)> {
    // Prepare paths by redirecting to hidden Merkle Tree folder
    let path_l = path_l.join(".rush");
    let path_r = path_r.join(".rush");

    let root_node_l = node_from_path(&path_l)?;
    let root_node_r = node_from_path(&path_r)?;

    if root_node_l.root_hash == root_node_r.root_hash {
        return Ok(None);
    }

    // The hashes differs. We need to check what's diffrent

    // Sanity checks. Fails if meta data is not equal
    // 1. We check that we use the same hashing method
    if root_node_l.hash_method != root_node_r.hash_method {
        bail!(
            "Hash methods differ: {} vs {}",
            root_node_l.hash_method,
            root_node_r.hash_method
        )
    }
    // 2. We check that we use the same number of bytes to hash
    if root_node_l.bytes_to_hash != root_node_r.bytes_to_hash {
        bail!(
            "Bytes to hash differ: {} vs {}",
            root_node_l.bytes_to_hash,
            root_node_r.bytes_to_hash
        )
    }

    let children_l = map_children(&root_node_l);
    let children_r = map_children(&root_node_r);

    for (k, v) in &children_l {
        println!("{k}: {}", hex::encode(v));
    }
    for (k, v) in &children_r {
        println!("{k}: {}", hex::encode(v));
    }
    diff_nodes(&root_node_l, &root_node_r)
}
