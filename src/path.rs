// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Extensions to [`Path`].

use std::{io, path::Path};
use sealed::sealed;
use crate::ops;

#[sealed]
pub trait PathExt {
/// Returns [`true`] if the path is a hidden file or directory.
///
/// # Platform-specific behavior
///
/// - On Unix, returns [`true`] if the file name starts with `.`.
/// - On Windows, returns [`true`] if the file has the hidden file attribute.
///
/// This is an alias to [`hf::is_hidden`].
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
/// use hf::PathExt;
/// use std::path::Path;
///
/// assert_eq!(Path::new(".foo.txt").is_hidden().unwrap(), true);
/// assert_eq!(Path::new("foo.txt").is_hidden().unwrap(), false);
///
/// assert!(Path::new(".foo.txt/..").is_hidden().is_err());
/// # }
/// ```
///
/// ## On Windows
///
/// ```
/// # #[cfg(windows)]
/// # {
/// use hf::PathExt;
/// use std::{
///     fs::{self, File},
///     process::Command,path::Path
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
/// assert_eq!(file_path.is_hidden().unwrap(), true);
///
/// // Clear the hidden file attribute.
/// Command::new("attrib")
///     .arg("-h")
///     .arg(&file_path)
///     .status()
///     .unwrap();
/// assert_eq!(file_path.is_hidden().unwrap(), false);
///
/// fs::remove_file(file_path).unwrap();
/// assert!(file_path.is_hidden().is_err());
/// # }
/// ```
fn is_hidden(&self) -> io::Result<bool>;
}

#[sealed]
impl PathExt for Path {
fn is_hidden(&self) -> io::Result<bool> {
    ops::is_hidden(self)
}
}
