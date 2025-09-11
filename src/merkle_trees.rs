use std::{path::Path, sync::mpsc::channel, thread};

use crossbeam_channel::bounded;

use crate::{HashMethod, md5_hasher::md5_hash_file};

pub fn build_merkle_tree(paths: &Path, method: &HashMethod, bytes_to_hash: u64) {
    let mut hash_method;
    match method {
        HashMethod::Md5 => {
            hash_method = md5_hash_file;
        }
        _ => unimplemented!(),
    }

    let workers = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);

    // let (s, r) = bounded(workers);
}
