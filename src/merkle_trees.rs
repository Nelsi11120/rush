use anyhow::{Result, bail};
use rayon::prelude::*;
use rs_merkle::MerkleTree;

use std::fs::File;
use std::io::BufWriter;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{HashMethod, md5_hasher::Md5Algorithm};
use crate::{utils::Leaf, utils::Node, utils::rel_path_str};

fn setup_build(root: &Path) -> anyhow::Result<PathBuf> {
    let rush_root = root.join(".rush");
    fs::create_dir_all(&rush_root)?;
    Ok(rush_root)
}

fn store_node_to_disk(
    node: &Node,
    dataset_root: &Path,
    path: &Path,
    rush_root: &Path,
) -> Result<()> {
    let rel = path.strip_prefix(dataset_root)?;
    let target_dir = rush_root.join(rel);

    fs::create_dir_all(&target_dir)?;

    let file_path = target_dir.join("merkle.json");
    let file = BufWriter::new(File::create(&file_path)?);
    serde_json::to_writer(file, node)?;
    Ok(())
}

pub fn build_merkle_tree(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
    store: bool,
) -> Result<[u8; 16]> {
    // Get the rush root path
    let rush_root = if store {
        setup_build(path)?
    } else {
        path.into()
    };
    build_merkle_tree_rec(path, path, &rush_root, method, bytes_to_hash, store)
}

fn build_merkle_tree_rec(
    dataset_root: &Path,
    path: &Path,
    rush_root: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
    store: bool,
) -> Result<[u8; 16]> {
    let hash_method = method.hash_method();

    let mut entries: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.file_name().is_none_or(|p| p != ".rush"))
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
                hash = build_merkle_tree_rec(
                    dataset_root,
                    p,
                    rush_root,
                    method,
                    bytes_to_hash,
                    store,
                )?;
            }

            // convert &PathBuf to String
            let name = match p.file_name() {
                Some(os) => os.to_string_lossy().into_owned(),
                None => bail!("missing file name for: {}", p.display()),
            };

            Ok(Leaf {
                name,
                content_hash: hash,
            })
        })
        .collect::<Result<Vec<Leaf>>>()?;

    let children_hash: Vec<[u8; 16]> = children.iter().map(|c| c.content_hash).collect();
    let merkle_tree = MerkleTree::<Md5Algorithm>::from_leaves(&children_hash);
    let root = merkle_tree.root().unwrap_or([0u8; 16]);

    let node = Node {
        name: rel_path_str(dataset_root, path),
        hash_method: method.as_str().to_string(),
        root_hash: root,
        children,
        bytes_to_hash,
    };

    if store {
        let _ = store_node_to_disk(&node, dataset_root, path, rush_root);
    }

    Ok(root)
}
