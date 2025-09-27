# Rush

`rush` (**Ru**st + ha**sh**) is a simple CLI tool to **hash and compare datasets** using Merkle trees written in Rust.  
It can hash individual files, build full dataset Merkle trees, and compare two trees to spot differences.


> **Note**  
> This is a **toy project**, built for fun and as a way to learn more about Rust.  
> It’s not meant for production use, but feel free to explore, tinker, or extend it.

## Features

- 🔑 Hash a single file (`rush hash`)
- 🌳 Build a Merkle tree from a dataset (`rush build`)
- 🔍 Compare two datasets (`rush diff`) – *in progress*
- 🔌 Extensible design: new hashers (BLAKE3, SHA-256, …) can be added easily

## Installation
```bash
git clone https://github.com/Nelsi11120/rush.git
cd rush
cargo build --release
```
The binary will be in ```target/release/rush```

## Usage

### Build a Merkle tree

```bash
rush build ./my_dataset --num-workers 4
```

### Hash a single file
```bash
rush hash ./file.txt
```

### Compare two datasets (WIP)

```bash
rush diff ./dataset_v1 ./dataset_v2
```

## TODO / Progress

#### Core Features
- [x] File hashing (`rush hash`)
- [x] Build Merkle tree (`rush build`)
- [ ] Diff two datasets (`rush diff`)
- [ ] Support for BLAKE3 and SHA-256
- [ ] Python bindings (PyPI wheel via pyo3/maturin)

#### Nice to have
- [ ] Benchmark hashing throughput
- [ ] Progress bar for hashing
- [ ] Shell completions (bash/zsh/fish)
- [ ] Add --quiet and --verbose modes
- [ ] Option to ignore hidden files or patterns 
- [ ] Distribute binaries: 
  - [ ] Homebrew tap (`brew install rush`)
  - [ ] Debian package + APT repo (`apt-get install rush`)


# rush
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.89%2B-orange.svg)](https://www.rust-lang.org)

`rush` (**Ru**st + ha**sh**) is a simple CLI tool to **hash and compare datasets** using Merkle trees, written in Rust.  
It can hash individual files, build full dataset Merkle trees, and compare two trees to spot differences.
