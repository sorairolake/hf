// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

use crate::utils::command;

#[test]
fn without_subcommand() {
    command::command()
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "requires a subcommand but one was not provided",
        ));
}

#[test]
fn log_level_without_subcommand() {
    command::command()
        .arg("--log-level")
        .arg("WARN")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "requires a subcommand but one was not provided",
        ));
}
