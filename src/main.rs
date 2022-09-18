//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

// Lint levels of rustc.
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

mod cli;
mod core;
#[cfg(unix)]
#[path = "unix.rs"]
mod os;
#[cfg(windows)]
#[path = "windows.rs"]
mod os;

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    match core::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            if let Some(err) = err.downcast_ref::<io::Error>() {
                match err.kind() {
                    io::ErrorKind::NotFound => return sysexits::ExitCode::NoInput.into(),
                    io::ErrorKind::PermissionDenied => return sysexits::ExitCode::NoPerm.into(),
                    _ => (),
                }
            }
            ExitCode::FAILURE
        }
    }
}
