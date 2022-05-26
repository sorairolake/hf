//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use anyhow::{Context, Result};

use crate::cli::Opt;
use crate::os;

/// Runs the program and returns the result.
pub fn run() -> Result<()> {
    let args = Opt::default();

    if let Some(shell) = args.generate_completion {
        Opt::print_completion(shell);

        return Ok(());
    }

    let filtered_files = args
        .input
        .into_iter()
        .filter_map(|file| {
            let is_hidden_file = os::is_hidden(&file).with_context(|| {
                format!(
                    "Failed to get information about the file: {}",
                    file.display()
                )
            });
            match is_hidden_file {
                Ok(true) if args.hide => None,
                Ok(true) if args.show => Some(Ok(file)),
                Ok(false) if args.hide => Some(Ok(file)),
                Ok(false) if args.show => None,
                Err(err) => Some(Err(err)),
                Ok(_) => unreachable!(),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    if args.dry_run {
        for file in filtered_files {
            println!("{}", file.display());
        }
    } else if args.force {
        if args.hide {
            for file in filtered_files {
                os::hide(&file)
                    .with_context(|| format!("Failed to hide the file: {}", file.display()))?;
            }
        } else if args.show {
            for file in filtered_files {
                os::show(&file)
                    .with_context(|| format!("Failed to show the file: {}", file.display()))?;
            }
        }
    }

    Ok(())
}
