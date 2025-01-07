// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "application")]
fn generate_man_page(out_dir: &str) -> std::io::Result<std::process::ExitStatus> {
    use std::{env, process::Command};

    let man_dir = env::current_dir()?.join("docs/man/man1");
    let mut command = Command::new("asciidoctor");
    command
        .args(["-b", "manpage"])
        .args(["-D", out_dir])
        .arg(man_dir.join("*.1.adoc"))
        .status()
}

#[cfg(feature = "application")]
fn main() {
    use std::env;

    println!("cargo:rerun-if-changed=docs/man");

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not defined");
    match generate_man_page(&out_dir) {
        Ok(exit_status) => {
            if !exit_status.success() {
                println!("cargo:warning=Asciidoctor failed: {exit_status}");
            }
        }
        Err(err) => {
            println!("cargo:warning=failed to execute Asciidoctor: {err}");
        }
    }
}

#[cfg(not(feature = "application"))]
fn main() {}
