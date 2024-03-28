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

use std::fs::File;

use predicates::prelude::predicate;

fn command() -> assert_cmd::Command {
    let mut command = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.current_dir("tests");
    command
}

#[test]
fn generate_completion_conflicts_with_subcommands() {
    command()
        .arg("--generate-completion")
        .arg("bash")
        .arg("encode")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the subcommand 'encode' cannot be used with '--generate-completion <SHELL>'",
        ));
    command()
        .arg("--generate-completion")
        .arg("bash")
        .arg("decode")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the subcommand 'decode' cannot be used with '--generate-completion <SHELL>'",
        ));
}

#[test]
fn generate_completion() {
    command()
        .arg("--generate-completion")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("elvish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("nushell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("powershell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::ne(""));
}

#[test]
fn generate_completion_with_invalid_shell() {
    command()
        .arg("--generate-completion")
        .arg("a")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--generate-completion <SHELL>'",
        ));
}

#[test]
fn long_version() {
    command()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/long-version.md"
        )));
}

#[test]
fn after_long_help() {
    command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/after-long-help.md"
        )));
}

#[test]
fn basic_hide() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("file");

    File::create(&file_path).unwrap();

    command()
        .arg("hide")
        .arg("-n")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            file_path.file_name().unwrap().to_string_lossy(),
        ));

    command()
        .arg("hide")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success();

    #[cfg(unix)]
    assert!(temp_dir.join(".file").exists());
    #[cfg(windows)]
    assert!(hf::is_hidden(file_path).unwrap());
}

#[test]
fn long_version_for_hide_command() {
    command()
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
    command()
        .arg("hide")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/hide-after-long-help.md"
        )));
}

#[test]
fn basic_show() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join(if cfg!(unix) { ".file" } else { "file" });

    File::create(&file_path).unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path)
        .status()
        .unwrap();

    command()
        .arg("show")
        .arg("-n")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            file_path.file_name().unwrap().to_string_lossy(),
        ));

    command()
        .arg("show")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success();

    #[cfg(unix)]
    assert!(temp_dir.join("file").exists());
    #[cfg(windows)]
    assert!(!hf::is_hidden(file_path).unwrap());
}

#[test]
fn long_version_for_show_command() {
    command()
        .arg("show")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/long-version.md"
        )));
}

#[test]
fn after_long_help_for_show_command() {
    command()
        .arg("show")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/show-after-long-help.md"
        )));
}
