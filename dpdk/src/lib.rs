#![warn(rust_2018_idioms)]
#![allow(warnings, clippy::all)] // XXX: This package is under construction

mod ffi;

pub mod eal;
pub mod zeroable;

/// Reexport of [arrayvec][arrayvec] crate
///
/// This is reexported so that downstream crates don't have to manually import arrayvec and won't
/// have version conflicts
pub use arrayvec;
