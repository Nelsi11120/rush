#![allow(unused)]
use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use jwalk::WalkDir;
use std::fmt;
use std::path::{Path, PathBuf};

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
        #[arg(value_name = "PATH", value_hint = ValueHint::DirPath, num_args = 1..,)]
        paths: Vec<PathBuf>,
        /// The hashing function we want to use to hash
        #[arg(long, default_value_t = HashMethod::Md5)]
        method: HashMethod,
        /// The number of bytes to hash, if 0 is provided then it hashes the full content of the file
        #[arg(short, long, default_value_t = 0)]
        bytes_to_hash: u8,
    },
    /// Compare the two Merkle trees from folder path
    Compare {
        /// Path to the first folder
        #[arg(value_hint = ValueHint::DirPath)]
        path1: PathBuf,
        /// Path to the second folder
        #[arg(value_hint = ValueHint::DirPath)]
        path2: PathBuf,
    },
}
#[derive(Default, Clone, ValueEnum, Debug)]
enum HashMethod {
    #[clap(alias = "md5")]
    #[default]
    Md5,
}

impl fmt::Display for HashMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            HashMethod::Md5 => "md5",
        })
    }
}

// fn walk() -> Result<(), Box<dyn std::error::Error>> {
//     for entry in WalkDir::new("/Users/nelson/Documents/datasets").sort(true) {
//         println!("{}", entry?.path().display());
//     }
//     Ok(())
// }

fn build_merkle_tree(folder: &Path) {
    println!("d{}", folder.display())
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
                build_merkle_tree(path);
            }
        }
        Command::Compare { path1, path2 } => {
            unimplemented!()
        }
    }
}
