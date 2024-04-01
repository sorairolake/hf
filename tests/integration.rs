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
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    command()
        .arg("hide")
        .arg("-n")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(r#"{file_path:?}"#)));

    command()
        .arg("hide")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{file_path:?} has been hidden"#
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

    command()
        .arg("hide")
        .arg("-n")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(r#"{:?}"#, file_path.0)))
        .stdout(predicate::str::contains(format!(r#"{:?}"#, file_path.1)));

    command()
        .arg("hide")
        .arg("-f")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{:?} has been hidden"#,
            file_path.0
        )))
        .stdout(predicate::str::contains(format!(
            r#"{:?} has been hidden"#,
            file_path.1
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

    command()
        .arg("hide")
        .arg("-n")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{file_path:?} is ignored"#
        )));

    command()
        .arg("hide")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{file_path:?} is already hidden"#
        )));

    #[cfg(unix)]
    assert!(temp_dir.join(".foo.txt").exists());
    #[cfg(windows)]
    assert!(hf::is_hidden(file_path).unwrap());
}

#[test]
fn hide_when_file_does_not_exist() {
    {
        let command = command()
            .arg("hide")
            .arg("-n")
            .arg("non_existent.txt")
            .assert()
            .failure()
            .code(66);
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                r#"could not read information from "non_existent.txt""#,
            ));
        } else {
            command.stderr(predicate::str::contains(
                r#""non_existent.txt" does not exist"#,
            ));
        }
    }

    {
        let command = command()
            .arg("hide")
            .arg("-f")
            .arg("non_existent.txt")
            .assert()
            .failure()
            .code(66);
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                r#"could not read information from "non_existent.txt""#,
            ));
        } else {
            command.stderr(predicate::str::contains(
                r#""non_existent.txt" does not exist"#,
            ));
        }
    }
}

#[test]
fn hide_with_off_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    command()
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

    command()
        .arg("hide")
        .arg("--log-level")
        .arg("WARN")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{file_path:?} is already hidden"#
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

    command()
        .arg("hide")
        .arg("--log-level")
        .arg("INFO")
        .arg("-f")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{:?} has been hidden"#,
            file_path.0
        )))
        .stdout(predicate::str::contains(format!(
            r#"{:?} is already hidden"#,
            file_path.1
        )));
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
    let file_path = temp_dir.join(if cfg!(unix) { ".foo.txt" } else { "foo.txt" });

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
        .stdout(predicate::str::contains(format!(r#"{file_path:?}"#)));

    command()
        .arg("show")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{file_path:?} has been shown"#
        )));

    #[cfg(unix)]
    assert!(temp_dir.join("foo.txt").exists());
    #[cfg(windows)]
    assert!(!hf::is_hidden(file_path).unwrap());
}

#[test]
fn show_with_multiple_files() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = (
        temp_dir.join(if cfg!(unix) { ".foo.txt" } else { "foo.txt" }),
        temp_dir.join(if cfg!(unix) { ".bar.txt" } else { "bar.txt" }),
    );

    File::create(&file_path.0).unwrap();
    File::create(&file_path.1).unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path.0)
        .status()
        .unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path.1)
        .status()
        .unwrap();

    command()
        .arg("show")
        .arg("-n")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(r#"{:?}"#, file_path.0)))
        .stdout(predicate::str::contains(format!(r#"{:?}"#, file_path.1)));

    command()
        .arg("show")
        .arg("-f")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{:?} has been shown"#,
            file_path.0
        )))
        .stdout(predicate::str::contains(format!(
            r#"{:?} has been shown"#,
            file_path.1
        )));
}

#[test]
fn show_when_non_hidden_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    command()
        .arg("show")
        .arg("-n")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{file_path:?} is ignored"#
        )));

    command()
        .arg("show")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{file_path:?} is already shown"#
        )));

    #[cfg(unix)]
    assert!(temp_dir.join("foo.txt").exists());
    #[cfg(windows)]
    assert!(!hf::is_hidden(file_path).unwrap());
}

#[test]
fn show_when_file_does_not_exist() {
    {
        let command = command()
            .arg("show")
            .arg("-n")
            .arg("non_existent.txt")
            .assert()
            .failure()
            .code(66);
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                r#"could not read information from "non_existent.txt""#,
            ));
        } else {
            command.stderr(predicate::str::contains(
                r#""non_existent.txt" does not exist"#,
            ));
        }
    }

    {
        let command = command()
            .arg("show")
            .arg("-f")
            .arg("non_existent.txt")
            .assert()
            .failure()
            .code(66);
        if cfg!(windows) {
            command.stderr(predicate::str::contains(
                r#"could not read information from "non_existent.txt""#,
            ));
        } else {
            command.stderr(predicate::str::contains(
                r#""non_existent.txt" does not exist"#,
            ));
        }
    }
}

#[test]
fn show_with_off_log_level() {
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

    command()
        .arg("show")
        .arg("--log-level")
        .arg("OFF")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn show_with_warn_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = temp_dir.join("foo.txt");

    File::create(&file_path).unwrap();

    command()
        .arg("show")
        .arg("--log-level")
        .arg("WARN")
        .arg("-f")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{file_path:?} is already shown"#
        )));
}

#[test]
fn show_with_info_log_level() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = temp_dir.path();
    let file_path = (
        temp_dir.join(if cfg!(unix) { ".foo.txt" } else { "foo.txt" }),
        temp_dir.join("bar.txt"),
    );

    File::create(&file_path.0).unwrap();
    File::create(&file_path.1).unwrap();
    #[cfg(windows)]
    std::process::Command::new("attrib")
        .arg("+h")
        .arg(&file_path.0)
        .status()
        .unwrap();

    command()
        .arg("show")
        .arg("--log-level")
        .arg("INFO")
        .arg("-f")
        .arg(&file_path.0)
        .arg(&file_path.1)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            r#"{:?} has been shown"#,
            file_path.0
        )))
        .stdout(predicate::str::contains(format!(
            r#"{:?} is already shown"#,
            file_path.1
        )));
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