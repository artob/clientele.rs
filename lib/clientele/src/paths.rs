// This is free and unencumbered software released into the public domain.

use crate::envs;
use camino::Utf8PathBuf;

pub fn home() -> Option<Utf8PathBuf> {
    envs::home().map(Utf8PathBuf::from)
}

pub fn tmpdir() -> Option<Utf8PathBuf> {
    envs::tmpdir().map(Utf8PathBuf::from)
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_data_home() -> Option<Utf8PathBuf> {
    envs::xdg_data_home().map(Utf8PathBuf::from)
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_config_home() -> Option<Utf8PathBuf> {
    envs::xdg_config_home().map(Utf8PathBuf::from)
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_state_home() -> Option<Utf8PathBuf> {
    envs::xdg_state_home().map(Utf8PathBuf::from)
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_cache_home() -> Option<Utf8PathBuf> {
    envs::xdg_cache_home().map(Utf8PathBuf::from)
}
