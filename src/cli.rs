//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::env;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

use clap::{AppSettings, CommandFactory, Parser, ValueHint};
use clap_complete::{Generator, Shell};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[clap(version, about)]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
pub struct Opt {
    /// Make files or directories invisible.
    ///
    /// This is the default behavior.
    #[clap(short('H'), long, conflicts_with("show"))]
    pub hide: bool,

    /// Make hidden files or directories visible.
    #[clap(short, long)]
    pub show: bool,

    /// Make actual changes to files or directories.
    #[clap(short, long, conflicts_with("dry-run"))]
    pub force: bool,

    /// Only show what would be done.
    ///
    /// This is the default behavior.
    #[clap(short('n'), long)]
    pub dry_run: bool,

    /// Files or directories to make changes.
    #[clap(
        value_parser,
        value_name("FILE"),
        value_hint(ValueHint::FilePath),
        required_unless_present("generate-completion")
    )]
    pub input: Vec<PathBuf>,

    /// Suppress log messages.
    #[clap(short, long, conflicts_with("verbose"))]
    pub quiet: bool,

    /// Verbose mode.
    ///
    /// Can be specified multiple times to increase the log level.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,

    /// Generate shell completion.
    ///
    /// The completion is output to stdout.
    #[clap(long, value_enum, value_name("SHELL"))]
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

    /// Processes relations between arguments.
    pub fn process_relations(mut self) -> Self {
        let command_name = env::args()
            .next()
            .map(PathBuf::from)
            .as_ref()
            .and_then(|name| Path::file_stem(name))
            .and_then(OsStr::to_str)
            .map_or_else(|| "hf".to_string(), str::to_string);

        if !self.force {
            self.dry_run = true;
        }

        if command_name == "unhf" {
            self.show = true;
            self.hide = bool::default();
        } else if !self.show {
            self.hide = true;
        }
        self
    }
}
