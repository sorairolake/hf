//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::process::ExitCode;

use anyhow::Result;

use crate::cli::Opt;

/// Runs the program and returns the exit status.
#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<ExitCode> {
    let args = Opt::default();

    if let Some(shell) = args.generate_completion {
        Opt::print_completion(shell);

        return Ok(ExitCode::SUCCESS);
    }

    Ok(ExitCode::SUCCESS)
}
