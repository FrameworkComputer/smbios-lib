//! SMBIOS Library
//!
//! Implements the DMTF [System Management BIOS (SMBIOS) Reference Specification 3.5.0](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.5.0.pdf).
//!
//! This library focuses on the tasks involved with reading and interpreting
//! BIOS data.

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]

#![no_std]
extern crate no_std_compat as std;

mod core;
#[cfg(feature = "std")]
mod file_io;
#[cfg(feature = "std")]
mod macos;
mod structs;
#[cfg(feature = "std")]
mod unix;
#[cfg(feature = "std")]
mod windows;

pub use structs::*;

pub use crate::core::*;
#[cfg(feature = "std")]
pub use file_io::*;

#[cfg(target_family = "windows")]
pub use windows::{load_windows_smbios_data, raw_smbios_from_device, table_load_from_device};

#[cfg(feature = "std")]
pub use windows::WinSMBiosData;

#[cfg(feature = "std")]
//#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
pub use unix::*;

#[cfg(feature = "std")]
//#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use macos::*;
