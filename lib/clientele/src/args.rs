// This is free and unencumbered software released into the public domain.

extern crate std;

use std::{ffi::OsString, vec::Vec};

pub fn args_os() -> Result<Vec<OsString>, std::io::Error> {
    #[cfg(not(feature = "wild"))]
    let args = std::env::args_os();
    #[cfg(feature = "wild")]
    let args = wild::args_os();

    #[cfg(not(feature = "argfile"))]
    return Ok(args.collect());
    #[cfg(feature = "argfile")]
    return argfile::expand_args_from(args, argfile::parse_fromfile, argfile::PREFIX);
}
