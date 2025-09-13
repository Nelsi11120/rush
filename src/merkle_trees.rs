use std::{
    cmp, fs,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    thread,
};

use rayon::prelude::*;

use crate::{HashMethod, md5_hasher::md5_hash_file};

pub fn build_merkle_tree(
    path: &Path,
    method: &HashMethod,
    bytes_to_hash: u64,
) -> std::io::Result<[u8; 16]> {
    let mut hash_method;

    match method {
        HashMethod::Md5 => {
            hash_method = md5_hash_file;
        }
        _ => unimplemented!(),
    }

    let mut entries: Vec<PathBuf> =
        fs::read_dir(path)?.filter_map(|e| e.ok().map(|e| e.path())).collect();

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    // Recurse in Parallel
    let children = entries
        .par_iter()
        .map(|p| {
            let mut hash = [0u8; 16];
            if p.is_file() {
                hash = hash_method(&p, bytes_to_hash)?;
            } else {
                hash = build_merkle_tree(p, &method, bytes_to_hash)?;
            }
            Ok::<[u8; 16], std::io::Error>(hash)
        })
        .collect::<std::io::Result<Vec<[u8; 16]>>>();

    let children = children?;

    for child in children {
        println!("{}", hex::encode(child));
    }
    Ok([0u8; 16])
}
