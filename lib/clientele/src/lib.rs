// This is free and unencumbered software released into the public domain.

//! This crate provides CLI support utilities.
//!
//! ```edition2021
//! # use clientele::*;
//! ```

//#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

#[doc(hidden)]
mod prelude;

#[cfg(feature = "std")]
mod args;
#[cfg(feature = "std")]
pub use args::*;

#[cfg(feature = "camino")]
pub use camino::{Utf8Path, Utf8PathBuf};

#[cfg(feature = "dotenv")]
pub use dotenvy::dotenv;

#[cfg(feature = "std")]
mod envs;

mod feature;
pub use feature::*;

#[cfg(all(feature = "std", feature = "clap"))]
mod options;
pub use options::*;

#[cfg(all(feature = "std", feature = "camino"))]
mod paths;

mod sysexits;
pub use sysexits::*;

#[doc(hidden)]
pub mod crates {
    #[cfg(feature = "argfile")]
    pub use argfile;
    #[cfg(feature = "camino")]
    pub use camino;
    #[cfg(feature = "clap")]
    pub use clap;
    #[cfg(feature = "dirs")]
    pub use dirs;
    pub use dogma;
    #[cfg(feature = "dotenv")]
    pub use dotenvy;
    #[cfg(feature = "parse-duration")]
    pub use duration_str;
    #[cfg(feature = "error-stack")]
    pub use error_stack;
    #[cfg(feature = "tracing")]
    pub use tracing;
    #[cfg(feature = "parse-byteunit")]
    pub use ubyte;
    #[cfg(feature = "wild")]
    pub use wild;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
