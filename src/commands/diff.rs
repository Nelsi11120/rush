use crate::hashers::utils::Digest;
use crate::{utils::Node, utils::node_from_file};
use anyhow::{Result, bail};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::path::Path;

pub(crate) fn invoke(lhs: &Path, rhs: &Path) -> Result<()> {
    if let Some(d) = diff(lhs, rhs)? {
        // print in a simple, deterministic order
        for k in d.added {
            println!("Only in {}: {}", rhs.display(), k);
        }
        for k in d.removed {
            println!("Only in {}: {}", lhs.display(), k);
        }
        for k in d.changed {
            println!("Present in both but content differs: {}", k);
        }
    }
    // If None => identical => print nothing (like GNU diff)
    Ok(())
}

#[derive(Debug, Default)]
pub struct Diff {
    pub added: Vec<String>,   // present only in rhs
    pub removed: Vec<String>, // present only in lhs
    pub changed: Vec<String>, // present in both but different
}

impl Diff {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
}

fn node_from_path(path: &Path) -> Result<Node> {
    let node_path = path.join("merkle.json");
    let node = node_from_file(&node_path)?;
    Ok(node)
}

fn map_children(node: &Node) -> BTreeMap<&str, &Digest> {
    let mut children = BTreeMap::new();

    for c in &node.children {
        children.insert(c.name.as_str(), &c.hash);
    }

    children
}

pub fn diff(path_l: &Path, path_r: &Path) -> Result<Option<Diff>> {
    // Prepare paths by redirecting to hidden Merkle Tree folder
    let path_l = path_l.join(".rush");
    let path_r = path_r.join(".rush");
    let mut out = Diff::default();

    // Recursive call
    diff_rec(&path_l, &path_r, Path::new(""), &mut out)?;

    if out.is_empty() {
        Ok(None)
    } else {
        Ok(Some(out))
    }
}

fn diff_rec(lhs_path: &Path, rhs_path: &Path, rel: &Path, out: &mut Diff) -> Result<()> {
    let lhs_node = node_from_path(lhs_path)?;
    let rhs_node = node_from_path(rhs_path)?;

    // Sanity checks. Fails if meta data is not equal
    // 1. We check that we use the same hashing method
    if lhs_node.hash_method != rhs_node.hash_method {
        bail!(
            "Hash methods differ: {} vs {}",
            lhs_node.hash_method,
            rhs_node.hash_method
        )
    }

    // 2. We check that we use the same number of bytes to hash
    if lhs_node.bytes_to_hash != rhs_node.bytes_to_hash {
        bail!(
            "Bytes to hash differ at {}: {} vs {}",
            rel.display(),
            lhs_node.bytes_to_hash,
            rhs_node.bytes_to_hash
        )
    }

    // We can now make the comparisons
    if lhs_node.root_hash == rhs_node.root_hash {
        return Ok(());
    }

    let left_children = map_children(&lhs_node);
    let right_children = map_children(&rhs_node);

    // Removed (only in left)
    for name in left_children.keys() {
        if !right_children.contains_key(name) {
            out.removed.push(path_join(rel, name));
        }
    }
    // Added (only in right)
    for name in right_children.keys() {
        if !left_children.contains_key(name) {
            out.added.push(path_join(rel, name));
        }
    }
    // Changed (present in both but different)
    for (name, lhash) in &left_children {
        if let Some(rhash) = right_children.get(name) {
            if lhash == rhash {
                continue;
            }
            // children are different
            let lsub = lhs_path.join(name);
            let rsub = rhs_path.join(name);

            // need to check if the children is a dir or not
            if is_rush_dir(&lsub) && is_rush_dir(&rsub) {
                diff_rec(&lsub, &rsub, &rel.join(name), out)?;
            } else {
                // treat as a leaf change
                out.changed.push(path_join(rel, name));
            }
        }
    }
    Ok(())
}

fn path_join(rel: &Path, name: &str) -> String {
    if rel.as_os_str().is_empty() {
        // root level children
        name.to_string()
    } else {
        let mut s = String::with_capacity(rel.as_os_str().len() + 1 + name.len());
        write!(&mut s, "{}/{}", rel.display(), name).ok();
        s
    }
}

fn is_rush_dir(path: &Path) -> bool {
    fs::metadata(path.join("merkle.json"))
        .map(|m| m.is_file())
        .unwrap_or(false)
}
