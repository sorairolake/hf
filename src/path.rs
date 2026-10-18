// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Extensions to [`Path`].

use std::{io, path::Path};

use crate::platform::imp;

pub trait PathExt {
/// Returns [`true`] if the path is a hidden file or directory.
///
/// # Platform-specific behavior
///
/// - On Unix, returns [`true`] if the file name starts with `.`.
/// - On Windows, returns [`true`] if the file has the hidden file attribute.
///
/// # Errors
///
/// ## On Unix
///
/// Returns [`Err`] if any of the following are true:
///
/// - `self` terminates in `..`.
/// - `self` is not valid UTF-8.
///
/// ## On Windows
///
/// Returns [`Err`] if metadata about a file could not be obtained.
///
/// # Examples
///
/// ## On Unix
///
/// ```
/// # #[cfg(unix)]
/// # {
/// assert_eq!(hf::is_hidden(".foo.txt").unwrap(), true);
/// assert_eq!(hf::is_hidden("foo.txt").unwrap(), false);
///
/// assert!(hf::is_hidden(".foo.txt/..").is_err());
/// # }
/// ```
///
/// ## On Windows
///
/// ```
/// # #[cfg(windows)]
/// # {
/// use std::{
///     fs::{self, File},
///     process::Command,
/// };
///
/// let temp_dir = tempfile::tempdir().unwrap();
/// let file_path = temp_dir.path().join("foo.txt");
/// assert!(!file_path.exists());
///
/// File::create(&file_path).unwrap();
/// assert!(file_path.exists());
///
/// // Set the hidden file attribute.
/// Command::new("attrib")
///     .arg("+h")
///     .arg(&file_path)
///     .status()
///     .unwrap();
/// assert_eq!(hf::is_hidden(&file_path).unwrap(), true);
///
/// // Clear the hidden file attribute.
/// Command::new("attrib")
///     .arg("-h")
///     .arg(&file_path)
///     .status()
///     .unwrap();
/// assert_eq!(hf::is_hidden(&file_path).unwrap(), false);
///
/// fs::remove_file(&file_path).unwrap();
/// assert!(hf::is_hidden(file_path).is_err());
/// # }
/// ```
fn is_hidden(&self) -> io::Result<bool>;
}

impl PathExt for Path {
fn is_hidden(&self) -> io::Result<bool> {
    ops::is_hidden(self)
}
}
