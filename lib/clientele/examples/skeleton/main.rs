// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

use clientele::{
    crates::clap::{Parser, Subcommand},
    SysexitsError,
};

/// Skeleton command-line interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "Skeleton")]
#[command(arg_required_else_help = true)]
struct Options {
    /// Enable debugging output
    #[clap(short = 'd', long, value_parser, global = true)]
    debug: bool,

    /// Show license information
    #[clap(long, value_parser)]
    license: bool,

    /// Enable verbose output
    #[clap(short = 'v', long, value_parser, global = true)]
    verbose: bool,

    /// Print version information
    #[clap(short = 'V', long, value_parser)]
    version: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show the current configuration
    Config {},
}

pub fn main() -> Result<(), SysexitsError> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    if options.version {
        println!("skeleton {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if options.license {
        println!("This is free and unencumbered software released into the public domain.");
        return Ok(());
    }

    let subcommand = &options.command;
    match subcommand.as_ref().expect("subcommand is required") {
        Commands::Config {} => {
            println!("This is the implementation of the `config` subcommand.");
            Ok(())
        }
    }
}
