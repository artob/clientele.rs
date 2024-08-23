// This is free and unencumbered software released into the public domain.

extern crate std;

use clap::{ArgAction, Args};

#[derive(Debug, Args)]
pub struct StandardOptions {
    #[cfg(feature = "color")]
    /// Set the color output mode
    #[clap(long, default_value_t = clap::ColorChoice::Auto, global = true)]
    pub color: clap::ColorChoice,

    /// Enable debugging output
    #[clap(short = 'd', long, value_parser, global = true)]
    pub debug: bool,

    /// Show license information
    #[clap(long, value_parser)]
    pub license: bool,

    /// Enable verbose output (may be repeated for more verbosity)
    #[clap(short = 'v', long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Print version information
    #[clap(short = 'V', long, value_parser)]
    pub version: bool,
}
