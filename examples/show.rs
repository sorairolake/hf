// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of making a hidden file or directory visible.

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// Hidden file or directory to show.
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    hf::show(&opt.file).with_context(|| format!("could not show {}", opt.file.display()))
}
