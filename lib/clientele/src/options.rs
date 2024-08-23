// This is free and unencumbered software released into the public domain.

extern crate std;

use clap::{ArgAction, Args};

#[derive(Debug, Args)]
pub struct StandardOptions {
    /// Enable debugging output
    #[clap(short = 'd', long, value_parser, global = true)]
    pub debug: bool,

    /// Show license information
    #[clap(long, value_parser)]
    pub license: bool,

    /// Enable verbose output
    #[clap(short = 'v', long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Print version information
    #[clap(short = 'V', long, value_parser)]
    pub version: bool,
}
