//! Shared utilities used by both `mull-shell` and its downstream clients
//! (e.g. `mull-pager-render`). This crate sits upstream of `mull-shell`
//! so it must never depend on it.

pub mod clipboard;
pub mod placeholder_images;
pub mod session;
pub mod stderr;
pub mod ui_config;
