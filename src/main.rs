use std::{fmt, path::PathBuf};
mod hash;
mod md5_hasher;
mod merkle_trees;
use crate::{hash::hash_file, md5_hasher::md5_hash_file, merkle_trees::build_merkle_tree};
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use std::path::Path;

/// Simple tool to hash and compare your data
#[derive(Parser, Debug)]
#[command(name="rush", version = "1.0", about, long_about = None)]
struct Cli {
    /// The entry point command like build, compare...
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Build a Merkle tree from paths
    Build {
        /// One or more folder paths to build a Merkle tree for
        #[arg(value_name = "PATH", value_hint = ValueHint::DirPath, num_args = 1.., required=true)]
        paths: Vec<PathBuf>,
        /// The hashing function we want to use to hash
        #[arg(long, default_value_t = HashMethod::Md5)]
        method: HashMethod,
        /// The number of bytes to hash, if 0 is provided then it hashes the
        /// full content of the file
        #[arg(short, long, default_value_t = 0)]
        bytes_to_hash: u64,
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
        #[arg(long, default_value_t = HashMethod::Md5)]
        method: HashMethod,
        /// The number of bytes to hash, if 0 is provided then it hashes the
        /// full content of the file
        #[arg(short, long, default_value_t = 0)]
        bytes_to_hash: u64,
    },
}
#[derive(Default, Clone, ValueEnum, Debug)]
enum HashMethod {
    #[clap(alias = "md5")]
    #[default]
    Md5,
}

impl HashMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HashMethod::Md5 => "md5",
        }
    }

    pub fn hash_method(&self) -> fn(&Path, u64) -> Result<[u8; 16]> {
        match self {
            HashMethod::Md5 => md5_hash_file,
        }
    }
}

impl fmt::Display for HashMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            HashMethod::Md5 => "md5",
        })
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Build {
            paths,
            method,
            bytes_to_hash,
        } => {
            for path in paths {
                let hash_root = build_merkle_tree(path, method, *bytes_to_hash).unwrap();
                println!("{}", hex::encode(hash_root));
            }
        }
        Command::Diff { path1, path2 } => {
            unimplemented!()
        }
        Command::Hash {
            path,
            method,
            bytes_to_hash,
        } => {
            let hash = hash_file(path, method, *bytes_to_hash).unwrap();
            println!("Binary hash: {:?}", hash);
            println!("{}", hex::encode(hash));
        }
    }
}
