use anyhow::{Ok, Result, bail};
use rs_merkle::MerkleTree;

use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::hashers::md5::Md5Algorithm;
use crate::hashers::utils::{HashMethod, hash_file};
use crate::utils::{BuildConfig, Leaf, Node, rel_path_str};
use std::fs;
use std::fs::File;
use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
};

pub(crate) fn invoke(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
    buffer_size: usize,
    num_workers: usize,
) -> Result<()> {
    if path.is_dir() {
        let hash_root = build(path, method, bytes_to_hash, buffer_size, num_workers, true)?;
        println!("{}", hex::encode(hash_root));
    } else {
        anyhow::bail!("incorrect path: {}\n should be a directory", path.display());
    }
    Ok(())
}

fn setup_build(root: &Path) -> anyhow::Result<PathBuf> {
    let rush_root = root.join(".rush");
    fs::create_dir_all(&rush_root)?;
    Ok(rush_root)
}

fn get_deterministic_entries(path: &Path) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.file_name().is_none_or(|p| p != ".rush"))
        .collect();

    entries.sort();

    Ok(entries)
}

fn initialize(path: &Path, file_names: &mut Vec<PathBuf>) -> Result<()> {
    let entries = get_deterministic_entries(path)?;
    for entry in entries {
        if entry.is_file() {
            file_names.push(entry);
        } else if entry.is_dir() {
            initialize(&entry, file_names)?;
        }
    }
    Ok(())
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
    serde_json::to_writer_pretty(file, node)?;
    Ok(())
}

// TODO: find better argument handling
fn build_merkle_tree(
    path: &Path,
    hashes: &Vec<[u8; 16]>,
    file_index: &mut usize,
    cfg: &BuildConfig,
) -> Result<[u8; 16]> {
    let entries = get_deterministic_entries(path)?;
    let mut children = Vec::new();
    let mut merkle_tree = MerkleTree::<Md5Algorithm>::new();
    for entry in entries {
        let hash = {
            if entry.is_file() {
                let hash = hashes[*file_index];
                *file_index += 1;
                hash
            } else if entry.is_dir() {
                build_merkle_tree(&entry, hashes, file_index, cfg)?
            } else {
                bail!("neither file or folder")
            }
        };
        let name = rel_path_str(&cfg.dataset_root, &entry);
        children.push(Leaf { name, hash });
        merkle_tree.insert(hash);
    }
    // Don't forget to commit the changes made by MerkleTree::insert
    merkle_tree.commit();
    let root = merkle_tree.root().unwrap_or([0u8; 16]);

    let node = Node {
        name: rel_path_str(&cfg.dataset_root, path),
        hash_method: cfg.method.to_string(),
        root_hash: root,
        children,
        bytes_to_hash: cfg.bytes_to_hash,
    };

    if cfg.store {
        let _ = store_node_to_disk(&node, &cfg.dataset_root, path, &cfg.rush_root);
    }

    Ok(root)
}

fn build(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
    buffer_size: usize,
    num_workers: usize,
    store: bool,
) -> Result<[u8; 16]> {
    let mut file_names = Vec::new();
    // First DFS pass: Collect files
    initialize(path, &mut file_names)?;

    let nb_files = file_names.len();
    // allocate atomic counter
    let next = AtomicUsize::new(0);
    // Result buffer
    let hashes: Vec<[u8; 16]> = vec![[0u8; 16]; nb_files];

    // spawn threads
    thread::scope(|s| {
        for _ in 0..num_workers {
            s.spawn(|| -> Result<()> {
                loop {
                    // atomic counter
                    let i = next.fetch_add(1, Relaxed);
                    if i >= nb_files {
                        break;
                    }
                    let hash = hash_file(&file_names[i], method, bytes_to_hash, buffer_size)?;
                    // SAFETY: Each thread writes to a unique index, thanks to the atomic counter.
                    // No races and we can safely deref the raw pointer.
                    unsafe {
                        let ptr = hashes.as_ptr().add(i);
                        std::ptr::write(ptr as *mut [u8; 16], hash);
                    }
                }
                Ok(())
            });
        }
    });

    // assert_eq!(hashes.len(), file_names.len());

    // All the files are now hashed, we can build the merkle tree
    // Get the rush root path
    let rush_root = if store {
        setup_build(path)?
    } else {
        path.into()
    };
    let mut file_index = 0;
    let cfg = BuildConfig {
        dataset_root: path.to_path_buf(),
        rush_root,
        method: method.as_str(),
        bytes_to_hash,
        store,
    };
    let root = build_merkle_tree(path, &hashes, &mut file_index, &cfg)?;

    Ok(root)
}
