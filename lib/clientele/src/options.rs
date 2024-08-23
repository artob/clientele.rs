// This is free and unencumbered software released into the public domain.

extern crate std;

use clap::Args;

#[derive(Debug, Args)]
pub struct StandardOptions {
    /// Enable debugging output
    #[clap(short = 'd', long, value_parser, global = true)]
    pub debug: bool,

    /// Show license information
    #[clap(long, value_parser)]
    pub license: bool,

    /// Enable verbose output
    #[clap(short = 'v', long, value_parser, global = true)]
    pub verbose: bool,

    /// Print version information
    #[clap(short = 'V', long, value_parser)]
    pub version: bool,
}
