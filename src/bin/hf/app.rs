// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::Context;
use clap::{CommandFactory, Parser, error::ErrorKind};
use log::{info, warn};
use simplelog::{ColorChoice, Config, SimpleLogger, TermLogger, TerminalMode};

use crate::cli::{Command, Opt};

/// Runs the program and returns the result.
#[allow(clippy::too_many_lines)]
pub fn run() -> anyhow::Result<()> {
    let opt = Opt::parse();

    if let Some(shell) = opt.generate_completion {
        Opt::print_completion(shell);
        return Ok(());
    }

    let log_level = opt.log_level.into();
    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .or_else(|_| SimpleLogger::init(log_level, Config::default()))?;

    if let Some(command) = opt.command {
        match command {
            Command::Hide(arg) => {
                let files = arg
                    .input
                    .into_iter()
                    .map(|f| {
                        #[cfg(unix)]
                        std::fs::metadata(&f)
                            .with_context(|| format!("{} does not exist", f.display()))?;
                        let is_hidden = hf::is_hidden(&f).with_context(|| {
                            format!("could not read information from {}", f.display())
                        });
                        match is_hidden {
                            Ok(false) => Ok((f, true)),
                            Ok(true) => Ok((f, false)),
                            Err(err) => Err(err),
                        }
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?;

                match (arg.dry_run, arg.force) {
                    (true, _) => {
                        for file in files {
                            if file.1 {
                                println!("{}", file.0.display());
                            } else {
                                warn!("{} is ignored", file.0.display());
                            }
                        }
                    }
                    (_, true) => {
                        for file in files {
                            if file.1 {
                                hf::hide(&file.0).with_context(|| {
                                    format!("could not hide {}", file.0.display())
                                })?;
                                info!("{} has been hidden", file.0.display());
                            } else {
                                warn!("{} is already hidden", file.0.display());
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Command::Show(arg) => {
                let files = arg
                    .input
                    .into_iter()
                    .map(|f| {
                        #[cfg(unix)]
                        std::fs::metadata(&f)
                            .with_context(|| format!("{} does not exist", f.display()))?;
                        let is_hidden = hf::is_hidden(&f).with_context(|| {
                            format!("could not read information from {}", f.display())
                        });
                        match is_hidden {
                            Ok(true) => Ok((f, true)),
                            Ok(false) => Ok((f, false)),
                            Err(err) => Err(err),
                        }
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?;

                match (arg.dry_run, arg.force) {
                    (true, _) => {
                        for file in files {
                            if file.1 {
                                println!("{}", file.0.display());
                            } else {
                                warn!("{} is ignored", file.0.display());
                            }
                        }
                    }
                    (_, true) => {
                        for file in files {
                            if file.1 {
                                hf::show(&file.0).with_context(|| {
                                    format!("could not show {}", file.0.display())
                                })?;
                                info!("{} has been shown", file.0.display());
                            } else {
                                warn!("{} is already shown", file.0.display());
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    } else {
        Opt::command()
            .error(ErrorKind::MissingSubcommand, "missing subcommand")
            .exit()
    }
    Ok(())
}
