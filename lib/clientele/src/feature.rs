// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "argfile")]
    "argfile",
    #[cfg(feature = "clap")]
    "clap",
    #[cfg(feature = "dirs")]
    "dirs",
    #[cfg(feature = "dotenv")]
    "dotenv",
    #[cfg(feature = "error-stack")]
    "error-stack",
    #[cfg(feature = "tracing")]
    "tracing",
    #[cfg(feature = "wild")]
    "wild",
];
