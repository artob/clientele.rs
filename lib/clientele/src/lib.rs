// This is free and unencumbered software released into the public domain.

//! This crate provides CLI support utilities.
//!
//! ```edition2021
//! # use clientele::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

#[doc(hidden)]
pub mod prelude;

pub use argfile::*;

pub use dotenvy::dotenv;

mod feature;
pub use feature::*;

mod sysexits;
pub use sysexits::*;

pub use wild::*;

#[doc(hidden)]
pub mod crates {
    #[cfg(feature = "argfile")]
    pub use argfile;
    #[cfg(feature = "clap")]
    pub use clap;
    #[cfg(feature = "dirs")]
    pub use dirs;
    #[cfg(feature = "dotenv")]
    pub use dotenvy;
    #[cfg(feature = "error-stack")]
    pub use error_stack;
    #[cfg(feature = "tracing")]
    pub use tracing;
    #[cfg(feature = "wild")]
    pub use wild;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
