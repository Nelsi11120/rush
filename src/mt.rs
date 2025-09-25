use anyhow::{Ok, Result};
use rs_merkle::MerkleTree;

use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::HashMethod;
use crate::hash::hash_file;
use crate::md5_hasher::Md5Algorithm;
use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn collect_files<I>(walk: I) -> Result<Vec<PathBuf>>
where
    I: IntoIterator<Item = DirEntry>,
{
    let mut file_names = Vec::new();
    for entry in walk {
        if entry.file_type().is_file() {
            file_names.push(entry.path().to_path_buf());
        }
    }
    Ok(file_names)
}

fn build_merkle_tree<I>(walk: I, hashes: &[[u8; 16]]) -> Result<[u8; 16]>
where
    I: IntoIterator<Item = DirEntry>,
{
    let mut file_count = 0;
    for entry in walk {
        if entry.file_type().is_file() {
            let _hash = hashes[file_count];
            file_count += 1;
        }
    }
    Ok([0u8; 16])
}

fn get_walker(path: &Path) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
}

pub fn build(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
    buffer_size: usize,
    num_workers: usize,
    store: bool,
) -> Result<[u8; 16]> {
    // Create deterministic walk
    let walker = get_walker(path);
    // First DFS pass: Collect files
    let file_names = collect_files(walker)?;

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
    let walker = get_walker(path);
    let root = build_merkle_tree(walker, &hashes)?;

    // Since we have all the vectors
    let merkle_tree = MerkleTree::<Md5Algorithm>::from_leaves(&hashes);
    let root = merkle_tree.root().unwrap_or([0u8; 16]);

    Ok(root)
}
