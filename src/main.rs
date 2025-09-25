#![allow(unused)]
use std::{fmt, path::PathBuf};
mod diff;
mod hash;
mod md5_hasher;
mod merkle_trees;
mod utils;
use crate::{
    diff::diff, hash::hash_file, md5_hasher::md5_hash_file, merkle_trees::build_merkle_tree,
};
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;
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
    /// Build a Merkle tree from path
    Build {
        /// Root path of the folder to build a merkle tree for
        #[arg(value_name = "PATH", value_hint = ValueHint::DirPath, required=true)]
        path: PathBuf,
        /// The hashing function we want to use to hash
        #[arg(long, default_value_t = HashMethod::Md5)]
        method: HashMethod,
        /// The number of bytes to hash, if 0 is provided then it hashes the
        /// full content of the file
        #[arg(short, long, default_value_t = 0)]
        bytes_to_hash: u64,
        /// Buffer size for read and hash operations.
        #[arg(long = "bs", default_value_t = 8192)]
        buffer_size: usize,
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
        /// Buffer size for read and hash operations.
        /// TODO: clamp bs
        #[arg(long = "bs", default_value_t = 8192)]
        buffer_size: usize,
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

    pub fn hash_method(&self) -> fn(&Path, u64, usize) -> Result<[u8; 16]> {
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

fn main() -> ExitCode {
    if let Err(err) = rush() {
        eprintln!("error:{:#}", err);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn rush() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Build {
            path,
            method,
            bytes_to_hash,
            buffer_size,
        } => {
            if path.is_dir() {
                let hash_root =
                    build_merkle_tree(path, method, *bytes_to_hash, *buffer_size, true)?;
                println!("{}", hex::encode(hash_root));
            } else {
                anyhow::bail!("incorrect path: {}\n should be a directory", path.display());
            }
        }
        Command::Diff { path1, path2 } => {
            if let Some(d) = diff(path1, path2)? {
                // print in a simple, deterministic order
                for k in d.added {
                    println!("Only in right: {}", k);
                }
                for k in d.removed {
                    println!("Only in left: {}", k);
                }
                for k in d.changed {
                    println!("Files differ: {}", k);
                }
            }
            // If None => identical => print nothing (like GNU diff)
        }
        Command::Hash {
            path,
            method,
            bytes_to_hash,
            buffer_size,
        } => {
            let hash: [u8; 16];
            if path.is_file() {
                let file_len = std::fs::metadata(path)?.len();
                let runs = 3;
                let mut times = Vec::with_capacity(runs);
                let mut hash = [0u8; 16];
                for run in 1..=runs {
                    let t0 = Instant::now();
                    hash = hash_file(path, method, *bytes_to_hash, *buffer_size)?;
                    let dt = t0.elapsed().as_secs_f64();
                    times.push(dt);
                    let mbps = (file_len as f64) / 1_000_000.0 / dt;
                    let mibps = (file_len as f64) / (1024.0 * 1024.0) / dt;
                    eprintln!("run {run}: {dt:.3}s  {mbps:.1} MB/s ({mibps:.1} MiB/s)");
                }

                // stats
                let mut sorted = times.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let best = sorted[0];
                let mean = times.iter().sum::<f64>() / times.len() as f64;
                let median = if sorted.len() % 2 == 1 {
                    sorted[sorted.len() / 2]
                } else {
                    (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) * 0.5
                };

                // throughputs for stats
                let mb = file_len as f64 / 1_000_000.0;
                let mib = file_len as f64 / (1024.0 * 1024.0);
                eprintln!(
                    "best:   {best:.3}s  {}/s ({:.1} MiB/s)",
                    format!("{:.1} MB", mb / best),
                    mib / best
                );
                eprintln!(
                    "median: {median:.3}s  {}/s ({:.1} MiB/s)",
                    format!("{:.1} MB", mb / median),
                    mib / median
                );
                eprintln!(
                    "mean:   {mean:.3}s  {}/s ({:.1} MiB/s)",
                    format!("{:.1} MB", mb / mean),
                    mib / mean
                );
            } else if path.is_dir() {
                hash = build_merkle_tree(path, method, *bytes_to_hash, *buffer_size, false)?;
            } else {
                anyhow::bail!("path not accessible: {}", path.display());
            }
        }
    }

    Ok(())
}
