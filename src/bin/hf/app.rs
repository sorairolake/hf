// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::Context;
use clap::Parser;
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

    TermLogger::init(
        opt.log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .or_else(|_| SimpleLogger::init(opt.log_level, Config::default()))?;

    if let Some(command) = opt.command {
        match command {
            Command::Hide(arg) => {
                let files = arg
                    .input
                    .into_iter()
                    .map(|f| {
                        #[cfg(unix)]
                        std::fs::metadata(&f).with_context(|| format!("{f:?} does not exist"))?;
                        let is_hidden = hf::is_hidden(&f)
                            .with_context(|| format!("could not read information from {f:?}"));
                        match is_hidden {
                            Ok(false) => Ok((f, true)),
                            Ok(true) => Ok((f, false)),
                            Err(err) => Err(err),
                        }
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?;

                if arg.dry_run {
                    for file in files {
                        if file.1 {
                            println!("{:?}", file.0);
                        } else {
                            warn!("{:?} is ignored", file.0);
                        }
                    }
                    return Ok(());
                }

                if arg.force {
                    for file in files {
                        if file.1 {
                            hf::hide(&file.0)
                                .with_context(|| format!("could not hide {:?}", file.0))?;
                            info!("{:?} has been hidden", file.0);
                        } else {
                            warn!("{:?} is already hidden", file.0);
                        }
                    }
                    return Ok(());
                }
                unreachable!();
            }
            Command::Show(arg) => {
                let files = arg
                    .input
                    .into_iter()
                    .map(|f| {
                        #[cfg(unix)]
                        std::fs::metadata(&f).with_context(|| format!("{f:?} does not exist"))?;
                        let is_hidden = hf::is_hidden(&f)
                            .with_context(|| format!("could not read information from {f:?}"));
                        match is_hidden {
                            Ok(true) => Ok((f, true)),
                            Ok(false) => Ok((f, false)),
                            Err(err) => Err(err),
                        }
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?;

                if arg.dry_run {
                    for file in files {
                        if file.1 {
                            println!("{:?}", file.0);
                        } else {
                            warn!("{:?} is ignored", file.0);
                        }
                    }
                    return Ok(());
                }

                if arg.force {
                    for file in files {
                        if file.1 {
                            hf::show(&file.0)
                                .with_context(|| format!("could not show {:?}", file.0))?;
                            info!("{:?} has been shown", file.0);
                        } else {
                            warn!("{:?} is already shown", file.0);
                        }
                    }
                    return Ok(());
                }
                unreachable!();
            }
        }
    } else {
        unreachable!();
    }
}
