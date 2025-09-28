use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

use crate::hashers::utils::HashMethod;

/// Simple tool to hash and compare your data
#[derive(Parser, Debug)]
#[command(name="rush", version = "1.0", about, long_about = None)]
pub(crate) struct Cli {
    /// The entry point command like build, compare...
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Build a Merkle tree from path
    Build {
        /// Root path of the folder to build a merkle tree for
        #[arg(value_name = "PATH", value_hint = ValueHint::DirPath, required=true)]
        path: PathBuf,
        /// The hashing function we want to use to hash
        #[arg(short, long, default_value_t = HashMethod::Md5)]
        method: HashMethod,
        /// The number of bytes to hash, if 0 is provided then it hashes the
        /// full content of the file
        #[arg(long = "bh", default_value_t = 0)]
        bytes_to_hash: u64,
        /// Buffer size for read and hash operations.
        #[arg(short, long = "bs", default_value_t = 8192)]
        buffer_size: usize,
        /// Number of worker threads (default: 4)
        #[arg(long, short = 'w', default_value_t = 4)]
        num_workers: usize,
    },
    /// Compare the two Merkle trees from folder path
    Diff {
        /// Path to the first folder
        #[arg(value_hint = ValueHint::DirPath)]
        path1: PathBuf,
        /// Path to the second folder
        #[arg(value_hint = ValueHint::DirPath)]
        path2: PathBuf,
    },
    /// Hash a single file
    Hash {
        /// Path to the file
        #[arg(value_hint = ValueHint::FilePath)]
        path: PathBuf,
        /// The hashing function we want to use to hash
        #[arg(short, long, default_value_t = HashMethod::Md5)]
        method: HashMethod,
        /// The number of bytes to hash, if 0 is provided then it hashes the
        /// full content of the file
        #[arg(long = "bh", default_value_t = 0)]
        bytes_to_hash: u64,
        /// Buffer size for read and hash operations.
        #[arg(short, long = "bs", default_value_t = 8192)]
        buffer_size: usize,
    },
}
