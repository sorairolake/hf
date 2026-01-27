// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of making a file or directory invisible.

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// File or directory to hide.
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    hf::hide(&opt.file).with_context(|| format!("could not hide {}", opt.file.display()))
}
