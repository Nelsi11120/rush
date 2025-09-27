mod cli;
mod commands;
mod hashers;
mod utils;
use anyhow::Result;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(err) = rush() {
        eprintln!("error:{:#}", err);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn rush() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        // Rush commands
        cli::Command::Build {
            path,
            method,
            bytes_to_hash,
            buffer_size,
            num_workers,
        } => commands::build::invoke(&path, &method, bytes_to_hash, buffer_size, num_workers)?,

        cli::Command::Diff { path1, path2 } => commands::diff::invoke(&path1, &path2)?,

        cli::Command::Hash {
            path,
            method,
            bytes_to_hash,
            buffer_size,
        } => commands::hash::invoke(&path, &method, bytes_to_hash, buffer_size)?,
    }

    Ok(())
}
