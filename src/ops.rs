// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{io, path::Path};

use crate::platform::imp;

/// Returns `true` if the path is a hidden file or a hidden directory.
///
/// # Platform-specific behavior
///
/// - On Unix, returns `true` if the file name starts with `.`.
/// - On Windows, returns `true` if the file has the hidden file attribute.
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
/// assert!(hf::is_hidden(".foo.txt").unwrap());
/// assert!(!hf::is_hidden("foo.txt").unwrap());
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
/// # use std::{
/// #     fs::{self, File},
/// #     process::Command,
/// # };
/// #
/// let temp_dir = tempfile::tempdir().unwrap();
/// let file_path = temp_dir.path().join("foo.txt");
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
/// assert!(!hf::is_hidden(&file_path).unwrap());
///
/// fs::remove_file(&file_path).unwrap();
/// assert!(hf::is_hidden(file_path).is_err());
/// # }
/// ```
pub fn is_hidden(path: impl AsRef<Path>) -> io::Result<bool> {
    imp::is_hidden(path.as_ref())
}

/// Hides a file or a directory.
///
/// # Platform-specific behavior
///
/// - On Unix, this function renames the file to start with `.`.
/// - On Windows, this function sets the hidden file attribute to the file.
///
/// # Errors
///
/// ## On Unix
///
/// Returns [`Err`] if any of the following are true:
///
/// - The file name starts with `.`.
/// - `path` terminates in `..`.
/// - [`std::fs::rename`] returns an error.
///
/// ## On Windows
///
/// Returns [`Err`] if any of the following are true:
///
/// - Metadata about a file could not be obtained.
/// - The [`SetFileAttributesW`] function fails.
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
/// let file_path = temp_dir.join("foo.txt");
/// let hidden_file_path = hf::unix::hidden_file_name(&file_path).unwrap();
///
/// File::create(&file_path).unwrap();
/// assert!(file_path.exists());
/// assert!(!hidden_file_path.exists());
///
/// hf::hide(&file_path).unwrap();
/// assert!(!file_path.exists());
/// assert!(hidden_file_path.exists());
///
/// assert!(hf::hide(".bar.txt").is_err());
/// assert!(hf::hide("bar.txt/..").is_err());
/// assert!(hf::hide("bar.txt").is_err());
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
/// let file_path = temp_dir.path().join("foo.txt");
///
/// File::create(&file_path).unwrap();
/// assert!(!hf::is_hidden(&file_path).unwrap());
///
/// hf::hide(&file_path).unwrap();
/// assert!(hf::is_hidden(file_path).unwrap());
///
/// assert!(hf::hide("bar.txt").is_err());
/// # }
/// ```
///
/// [`SetFileAttributesW`]: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileattributesw
pub fn hide(path: impl AsRef<Path>) -> io::Result<()> {
    imp::hide(path.as_ref())
}

/// Shows a hidden file or a hidden directory.
///
/// # Platform-specific behavior
///
/// - On Unix, this function renames the file to start with a character other
///   than `.`.
/// - On Windows, this function clears the hidden file attribute to the file.
///
/// # Errors
///
/// ## On Unix
///
/// Returns [`Err`] if any of the following are true:
///
/// - The file name does not start with `.`.
/// - `path` terminates in `..`.
/// - [`std::fs::rename`] returns an error.
///
/// ## On Windows
///
/// Returns [`Err`] if any of the following are true:
///
/// - Metadata about a file could not be obtained.
/// - The [`SetFileAttributesW`] function fails.
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
/// let hidden_file_path = temp_dir.join(".foo.txt");
/// let file_path = hf::unix::normal_file_name(&hidden_file_path).unwrap();
///
/// File::create(&hidden_file_path).unwrap();
/// assert!(hidden_file_path.exists());
/// assert!(!file_path.exists());
///
/// hf::show(&hidden_file_path).unwrap();
/// assert!(!hidden_file_path.exists());
/// assert!(file_path.exists());
///
/// assert!(hf::show("bar.txt").is_err());
/// assert!(hf::show(".bar.txt/..").is_err());
/// assert!(hf::show(".bar.txt").is_err());
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
/// let file_path = temp_dir.path().join("foo.txt");
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
///
/// assert!(hf::show("bar.txt").is_err());
/// # }
/// ```
///
/// [`SetFileAttributesW`]: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileattributesw
pub fn show(path: impl AsRef<Path>) -> io::Result<()> {
    imp::show(path.as_ref())
}
