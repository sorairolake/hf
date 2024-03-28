// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{io, path::Path};

use crate::platform::imp;

/// Returns `true` if the path is a hidden file or a hidden directory.
///
/// # Errors
///
/// ## On Unix
///
/// Returns [`Err`] if `path` terminates in `..`.
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
/// assert!(hf::is_hidden(".file").unwrap());
/// assert!(!hf::is_hidden("file").unwrap());
/// # }
/// ```
///
/// ## On Windows
///
/// ```
/// # #[cfg(windows)]
/// # {
/// # use std::{fs::File, process::Command};
/// #
/// let temp_dir = tempfile::tempdir().unwrap();
/// let file_path = temp_dir.path().join("file");
///
/// File::create(&file_path).unwrap();
///
/// Command::new("attrib")
///     .arg("+h")
///     .arg(&file_path)
///     .status()
///     .unwrap();
/// assert!(hf::is_hidden(&file_path).unwrap());
///
/// Command::new("attrib")
///     .arg("-h")
///     .arg(&file_path)
///     .status()
///     .unwrap();
/// assert!(!hf::is_hidden(file_path).unwrap());
/// # }
/// ```
pub fn is_hidden(path: impl AsRef<Path>) -> io::Result<bool> {
    imp::is_hidden(path.as_ref())
}

/// Hides a file or a directory.
///
/// # Errors
///
/// ## On Unix
///
/// Returns [`Err`] if any of the following are true:
///
/// - `path` terminates in `..`.
/// - The file name starts with `.`.
/// - [`std::fs::rename`] returns an error.
///
/// ## On Windows
///
/// Returns [`Err`] if any of the following are true:
///
/// - Metadata about a file could not be obtained.
/// - `path` contains the null character.
/// - The [`SetFileAttributesA`] function fails.
///
/// # Examples
///
/// ## On Unix
///
/// ```
/// # #[cfg(unix)]
/// # {
/// # use std::fs::File;
/// #
/// let temp_dir = tempfile::tempdir().unwrap();
/// let temp_dir = temp_dir.path();
/// let file_path = temp_dir.join("file");
/// let hidden_file_path = temp_dir.join(".file");
///
/// File::create(&file_path).unwrap();
/// assert!(file_path.exists());
/// assert!(!hidden_file_path.exists());
///
/// hf::hide(&file_path).unwrap();
/// assert!(!file_path.exists());
/// assert!(hidden_file_path.exists());
/// # }
/// ```
///
/// ## On Windows
///
/// ```
/// # #[cfg(windows)]
/// # {
/// # use std::fs::File;
/// #
/// let temp_dir = tempfile::tempdir().unwrap();
/// let file_path = temp_dir.path().join("file");
///
/// File::create(&file_path).unwrap();
/// assert!(!hf::is_hidden(&file_path).unwrap());
///
/// hf::hide(&file_path).unwrap();
/// assert!(hf::is_hidden(file_path).unwrap());
/// # }
/// ```
///
/// [`SetFileAttributesA`]: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileattributesa
pub fn hide(path: impl AsRef<Path>) -> io::Result<()> {
    imp::hide(path.as_ref())
}

/// Shows a hidden file or a hidden directory.
///
/// # Errors
///
/// ## On Unix
///
/// Returns [`Err`] if any of the following are true:
///
/// - `path` terminates in `..`.
/// - The file name does not start with `.`.
/// - [`std::fs::rename`] returns an error.
///
/// ## On Windows
///
/// Returns [`Err`] if any of the following are true:
///
/// - Metadata about a file could not be obtained.
/// - `path` contains the null character.
/// - The [`SetFileAttributesA`] function fails.
///
/// # Examples
///
/// ## On Unix
///
/// ```
/// # #[cfg(unix)]
/// # {
/// # use std::fs::File;
/// #
/// let temp_dir = tempfile::tempdir().unwrap();
/// let temp_dir = temp_dir.path();
/// let hidden_file_path = temp_dir.join(".file");
/// let file_path = temp_dir.join("file");
///
/// File::create(&hidden_file_path).unwrap();
/// assert!(hidden_file_path.exists());
/// assert!(!file_path.exists());
///
/// hf::show(&hidden_file_path).unwrap();
/// assert!(!hidden_file_path.exists());
/// assert!(file_path.exists());
/// # }
/// ```
///
/// ## On Windows
///
/// ```
/// # #[cfg(windows)]
/// # {
/// # use std::{fs::File, process::Command};
/// #
/// let temp_dir = tempfile::tempdir().unwrap();
/// let file_path = temp_dir.path().join("file");
///
/// File::create(&file_path).unwrap();
///
/// Command::new("attrib")
///     .arg("+h")
///     .arg(&file_path)
///     .status()
///     .unwrap();
/// assert!(hf::is_hidden(&file_path).unwrap());
///
/// hf::show(&file_path).unwrap();
/// assert!(!hf::is_hidden(file_path).unwrap());
/// # }
/// ```
///
/// [`SetFileAttributesA`]: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileattributesa
pub fn show(path: impl AsRef<Path>) -> io::Result<()> {
    imp::show(path.as_ref())
}
