//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::io;
use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use clap_complete::{Generator, Shell};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Opt {
    /// Make files or directories invisible.
    ///
    /// This is the default behavior.
    #[clap(short = 'H', long, conflicts_with = "visible")]
    pub hidden: bool,

    /// Make hidden files or directories visible.
    #[clap(short, long)]
    pub visible: bool,

    /// Make actual changes to files or directories.
    #[clap(short, long, conflicts_with = "dry-run")]
    pub force: bool,

    /// Only show what would be done.
    ///
    /// This is the default behavior.
    #[clap(short = 'n', long)]
    pub dry_run: bool,

    /// Files or directories to make changes.
    #[clap(value_name = "FILE")]
    pub input: Vec<PathBuf>,

    /// Generate shell completion.
    ///
    /// The completion is output to stdout.
    #[clap(long, value_name = "SHELL", arg_enum)]
    pub generate_completion: Option<Shell>,
}

impl Opt {
    pub fn print_completion(gen: impl Generator) {
        clap_complete::generate(
            gen,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}
