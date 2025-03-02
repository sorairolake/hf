// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    io::{self, Write},
    path::PathBuf,
};

use clap::{ArgGroup, Args, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::Generator;
use simplelog::LevelFilter;

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    '\n',
    "Copyright (C) 2022 Shun Sakai\n",
    '\n',
    "This program is distributed under the terms of either the Apache License 2.0 or\n",
    "the MIT License.\n",
    '\n',
    "This is free software: you are free to change and redistribute it. There is NO\n",
    "WARRANTY, to the extent permitted by law.\n",
    '\n',
    "Report bugs to <https://github.com/sorairolake/hf/issues>."
);

const AFTER_LONG_HELP: &str = "See `hf(1)` for more details.";

const HIDE_AFTER_LONG_HELP: &str = "See `hf-hide(1)` for more details.";

const SHOW_AFTER_LONG_HELP: &str = "See `hf-show(1)` for more details.";

#[derive(Debug, Parser)]
#[command(
    version,
    long_version(LONG_VERSION),
    about,
    max_term_width(100),
    propagate_version(true),
    after_long_help(AFTER_LONG_HELP),
    arg_required_else_help(true),
    args_conflicts_with_subcommands(true)
)]
pub struct Opt {
    /// The minimum log level to print.
    #[arg(
        long,
        value_enum,
        default_value_t,
        global(true),
        value_name("LEVEL"),
        ignore_case(true)
    )]
    pub log_level: LogLevel,

    /// Generate shell completion.
    ///
    /// The completion is output to standard output.
    #[arg(long, value_enum, value_name("SHELL"))]
    pub generate_completion: Option<Shell>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Make files and directories invisible.
    #[command(after_long_help(HIDE_AFTER_LONG_HELP))]
    Hide(Hide),

    /// Make hidden files and directories visible.
    #[command(after_long_help(SHOW_AFTER_LONG_HELP))]
    Show(Show),
}

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("mode").required(true)))]
pub struct Hide {
    /// Actually hide files and directories.
    #[arg(short, long, group("mode"))]
    pub force: bool,

    /// Don't actually hide anything, just show what would be done.
    #[arg(short('n'), long, group("mode"))]
    pub dry_run: bool,

    /// Files and directories to hide.
    #[arg(value_name("FILE"), value_hint(ValueHint::FilePath))]
    pub input: Vec<PathBuf>,
}

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("mode").required(true)))]
pub struct Show {
    /// Actually show hidden files and directories.
    #[arg(short, long, group("mode"))]
    pub force: bool,

    /// Don't actually show anything, just show what would be done.
    #[arg(short('n'), long, group("mode"))]
    pub dry_run: bool,

    /// Hidden files and directories to show.
    #[arg(value_name("FILE"), value_hint(ValueHint::FilePath))]
    pub input: Vec<PathBuf>,
}

impl Opt {
    /// Generates shell completion and print it.
    pub fn print_completion(generator: impl Generator) {
        clap_complete::generate(
            generator,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[allow(clippy::doc_markdown)]
#[value(rename_all = "lower")]
pub enum Shell {
    /// Bash.
    Bash,

    /// Elvish.
    Elvish,

    /// fish.
    Fish,

    /// Nushell.
    Nushell,

    #[allow(clippy::enum_variant_names)]
    /// PowerShell.
    PowerShell,

    /// Zsh.
    Zsh,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::Shell::Bash.file_name(name),
            Self::Elvish => clap_complete::Shell::Elvish.file_name(name),
            Self::Fish => clap_complete::Shell::Fish.file_name(name),
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
            Self::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            Self::Zsh => clap_complete::Shell::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn Write) {
        match self {
            Self::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Self::Elvish => clap_complete::Shell::Elvish.generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
#[value(rename_all = "UPPER")]
pub enum LogLevel {
    /// Lowest log level.
    Off,

    /// Error log level.
    Error,

    /// Warn log level.
    Warn,

    /// Info log level.
    #[default]
    Info,

    /// Debug log level.
    Debug,

    /// Trace log level.
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        Opt::command().debug_assert();
    }

    #[test]
    fn file_name_shell() {
        assert_eq!(Shell::Bash.file_name("hf"), "hf.bash");
        assert_eq!(Shell::Elvish.file_name("hf"), "hf.elv");
        assert_eq!(Shell::Fish.file_name("hf"), "hf.fish");
        assert_eq!(Shell::Nushell.file_name("hf"), "hf.nu");
        assert_eq!(Shell::PowerShell.file_name("hf"), "_hf.ps1");
        assert_eq!(Shell::Zsh.file_name("hf"), "_hf");
    }

    #[test]
    fn default_log_level() {
        assert_eq!(LogLevel::default(), LogLevel::Info);
    }

    #[test]
    fn from_log_level_to_level_filter() {
        assert_eq!(LevelFilter::from(LogLevel::Off), LevelFilter::Off);
        assert_eq!(LevelFilter::from(LogLevel::Error), LevelFilter::Error);
        assert_eq!(LevelFilter::from(LogLevel::Warn), LevelFilter::Warn);
        assert_eq!(LevelFilter::from(LogLevel::Info), LevelFilter::Info);
        assert_eq!(LevelFilter::from(LogLevel::Debug), LevelFilter::Debug);
        assert_eq!(LevelFilter::from(LogLevel::Trace), LevelFilter::Trace);
    }
}
