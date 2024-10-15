// This is free and unencumbered software released into the public domain.

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_data_home() -> Option<String> {
    var("XDG_DATA_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_config_home() -> Option<String> {
    var("XDG_CONFIG_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_state_home() -> Option<String> {
    var("XDG_STATE_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_cache_home() -> Option<String> {
    var("XDG_CACHE_HOME")
}

#[cfg(feature = "std")]
fn var(key: impl AsRef<std::ffi::OsStr>) -> Option<String> {
    use std::env::VarError::*;
    match std::env::var(key) {
        Err(NotPresent | NotUnicode(_)) => None,
        Ok(value) if value.trim().is_empty() => None,
        Ok(value) => Some(String::from(value.trim())),
    }
}
