//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn, SetLoggerError};
use simplelog::LevelFilter;

use crate::cli::Opt;
use crate::os;

fn logger_init(level: LevelFilter) -> Result<(), SetLoggerError> {
    use simplelog::{ColorChoice, Config, SimpleLogger, TermLogger, TerminalMode};

    TermLogger::init(
        level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .or_else(|_| SimpleLogger::init(level, Config::default()))
}

/// Runs the program and returns the result.
pub fn run() -> Result<()> {
    let args = Opt::parse().process_relations();

    let log_level = if args.quiet {
        LevelFilter::Off
    } else {
        match args.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };
    logger_init(log_level)?;

    if let Some(shell) = args.generate_completion {
        Opt::print_completion(shell);
        return Ok(());
    }

    let input_files = args
        .input
        .into_iter()
        .map(|file| {
            let is_hidden_file = os::is_hidden(&file).with_context(|| {
                format!(
                    "Failed to get information about the file: {}",
                    file.display()
                )
            });
            match is_hidden_file {
                Ok(true) if args.hide => Ok((file, false)),
                Ok(true) if args.show => Ok((file, true)),
                Ok(false) if args.hide => Ok((file, true)),
                Ok(false) if args.show => Ok((file, false)),
                Err(err) => Err(err),
                Ok(_) => unreachable!(),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    if args.dry_run {
        for file in input_files {
            if file.1 {
                println!("{}", file.0.display());
            } else {
                warn!("The file is ignored: {}", file.0.display());
            }
        }
    } else if args.force {
        if args.hide {
            for file in input_files {
                if file.1 {
                    os::hide(&file.0).with_context(|| {
                        format!("Failed to hide the file: {}", file.0.display())
                    })?;
                    info!("The file has been hidden: {}", file.0.display());
                } else {
                    warn!("The file is already hidden: {}", file.0.display());
                }
            }
        } else if args.show {
            for file in input_files {
                if file.1 {
                    os::show(&file.0).with_context(|| {
                        format!("Failed to show the file: {}", file.0.display())
                    })?;
                    info!("The file has been shown: {}", file.0.display());
                } else {
                    warn!("The file is already shown: {}", file.0.display());
                }
            }
        }
    }
    Ok(())
}
