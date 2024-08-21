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

mod feature;
pub use feature::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
