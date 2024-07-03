// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

mod utils;

use std::fs::File;

use predicates::prelude::predicate;

#[test]
fn basic_hide() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    utils::command::command()
        .arg("hide")
        .arg("-n")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("{}", file_path.display())));

    utils::command::command()
        .arg("hide")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{} has been hidden",
            file_path.display()
        )));

    #[cfg(unix)]
    assert!(temp_dir.join(".foo.txt").exists());
    #[cfg(windows)]
    assert!(hf::is_hidden(file_path).unwrap());
}

#[test]
fn hide_with_multiple_files() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = (temp_dir.join("foo.txt"), temp_dir.join("bar.txt"));

    File::create(&file_path.0).unwrap();
    File::create(&file_path.1).unwrap();

    utils::command::command()
        .arg("hide")
        .arg("-n")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{}",
            file_path.0.display()
        )))
        .stdout(predicate::str::contains(format!(
            "{}",
            file_path.1.display()
        )));

    utils::command::command()
        .arg("hide")
        .arg("-f")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{} has been hidden",
            file_path.0.display()
        )))
        .stdout(predicate::str::contains(format!(
            "{} has been hidden",
            file_path.1.display()
        )));
}

#[test]
fn hide_when_hidden_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join(if cfg!(unix) { ".foo.txt" } else { "foo.txt" });

    File::create(&file_path).unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path)
        .status()
        .unwrap();

    utils::command::command()
        .arg("hide")
        .arg("-n")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{} is ignored",
            file_path.display()
        )));

    utils::command::command()
        .arg("hide")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{} is already hidden",
            file_path.display()
        )));

    #[cfg(unix)]
    assert!(temp_dir.join(".foo.txt").exists());
    #[cfg(windows)]
    assert!(hf::is_hidden(file_path).unwrap());
}

#[test]
fn hide_when_file_does_not_exist() {
    {
        let command = utils::command::command()
            .arg("hide")
            .arg("-n")
            .arg("non_existent.txt")
            .assert()
            .failure()
            .code(66);
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                "could not read information from non_existent.txt",
            ));
        } else {
            command.stderr(predicate::str::contains("non_existent.txt does not exist"));
        }
    }

    {
        let command = utils::command::command()
            .arg("hide")
            .arg("-f")
            .arg("non_existent.txt")
            .assert()
            .failure()
            .code(66);
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                "could not read information from non_existent.txt",
            ));
        } else {
            command.stderr(predicate::str::contains("non_existent.txt does not exist"));
        }
    }
}

#[test]
fn hide_with_force_and_dry_run() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    utils::command::command()
        .arg("hide")
        .arg("-f")
        .arg("-n")
        .arg(&file_path)
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the argument '--force' cannot be used with '--dry-run'",
        ));
}

#[test]
fn hide_with_off_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    utils::command::command()
        .arg("hide")
        .arg("--log-level")
        .arg("OFF")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn hide_with_error_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    utils::command::command()
        .arg("hide")
        .arg("--log-level")
        .arg("ERROR")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn hide_with_warn_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join(if cfg!(unix) { ".foo.txt" } else { "foo.txt" });

    File::create(&file_path).unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path)
        .status()
        .unwrap();

    utils::command::command()
        .arg("hide")
        .arg("--log-level")
        .arg("WARN")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{} is already hidden",
            file_path.display()
        )));
}

#[test]
fn hide_with_info_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = (
        temp_dir.join("foo.txt"),
        temp_dir.join(if cfg!(unix) { ".bar.txt" } else { "bar.txt" }),
    );

    File::create(&file_path.0).unwrap();
    File::create(&file_path.1).unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path.1)
        .status()
        .unwrap();

    utils::command::command()
        .arg("hide")
        .arg("--log-level")
        .arg("INFO")
        .arg("-f")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{} has been hidden",
            file_path.0.display()
        )))
        .stdout(predicate::str::contains(format!(
            "{} is already hidden",
            file_path.1.display()
        )));
}

#[test]
fn hide_with_debug_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = (
        temp_dir.join("foo.txt"),
        temp_dir.join(if cfg!(unix) { ".bar.txt" } else { "bar.txt" }),
    );

    File::create(&file_path.0).unwrap();
    File::create(&file_path.1).unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path.1)
        .status()
        .unwrap();

    utils::command::command()
        .arg("hide")
        .arg("--log-level")
        .arg("DEBUG")
        .arg("-f")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{} has been hidden",
            file_path.0.display()
        )))
        .stdout(predicate::str::contains(format!(
            "{} is already hidden",
            file_path.1.display()
        )));
}

#[test]
fn hide_with_trace_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = (
        temp_dir.join("foo.txt"),
        temp_dir.join(if cfg!(unix) { ".bar.txt" } else { "bar.txt" }),
    );

    File::create(&file_path.0).unwrap();
    File::create(&file_path.1).unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path.1)
        .status()
        .unwrap();

    utils::command::command()
        .arg("hide")
        .arg("--log-level")
        .arg("TRACE")
        .arg("-f")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{} has been hidden",
            file_path.0.display()
        )))
        .stdout(predicate::str::contains(format!(
            "{} is already hidden",
            file_path.1.display()
        )));
}

#[test]
fn hide_with_invalid_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    utils::command::command()
        .arg("hide")
        .arg("--log-level")
        .arg("a")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--log-level <LEVEL>'",
        ));
}

#[test]
fn long_version_for_hide_command() {
    utils::command::command()
        .arg("hide")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/long-version.md"
        )));
}

#[test]
fn after_long_help_for_hide_command() {
    utils::command::command()
        .arg("hide")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/hide-after-long-help.md"
        )));
}
