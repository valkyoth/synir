#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![no_std]

/// Portable foundation; syntax APIs are planned in subsequent milestones.
pub use synir_core as core;

#[cfg(feature = "proc-macro")]
/// Compiler boundary scaffold; no import or expansion API is implemented yet.
pub use synir_host as native;
