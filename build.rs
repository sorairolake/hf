//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

// Lint levels of rustc.
#![warn(rust_2018_idioms)]
#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::env;
use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus};

fn generate_man_page(out_dir: impl AsRef<Path>) -> io::Result<ExitStatus> {
    let man_dir = env::current_dir()?.join("doc/man/man1");

    Command::new("asciidoctor")
        .args(["-b", "manpage"])
        .args(["-a", concat!("revnumber=", env!("CARGO_PKG_VERSION"))])
        .args(["-D".as_ref(), out_dir.as_ref()])
        .args([man_dir.join("hf.1.adoc"), man_dir.join("unhf.1.adoc")])
        .status()
}

fn main() {
    println!(
        "cargo:rerun-if-changed={}",
        env::current_dir().unwrap().join("doc/man").display()
    );

    match generate_man_page(env::var_os("OUT_DIR").unwrap()) {
        Ok(exit_status) => {
            if !exit_status.success() {
                println!("cargo:warning=Asciidoctor failed ({exit_status})");
            }
        }
        Err(err) => {
            println!("cargo:warning=Failed to execute Asciidoctor ({err})");
        }
    }
}
