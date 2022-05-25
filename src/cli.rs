//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::env;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

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
    /// Generate shell completion and print it.
    pub fn print_completion(gen: impl Generator) {
        clap_complete::generate(
            gen,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

impl Default for Opt {
    fn default() -> Self {
        let mut args = Self::parse();

        let command_name = env::args()
            .next()
            .map(PathBuf::from)
            .as_ref()
            .and_then(|name| Path::file_stem(name))
            .and_then(OsStr::to_str)
            .map_or_else(|| "hf".to_string(), str::to_string);

        if !args.force {
            args.dry_run = true;
        }

        if command_name == "unhf" {
            args.visible = true;
            args.hidden = bool::default();
        } else if !args.visible {
            args.hidden = true;
        }

        args
    }
}
