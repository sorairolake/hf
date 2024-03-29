// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Provides functionality for Unix platforms.

use std::{
    ffi::OsStr,
    fs,
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
};

pub(crate) fn is_hidden(path: &Path) -> io::Result<bool> {
    let file_name = path
        .file_name()
        .ok_or_else(|| Error::from(ErrorKind::InvalidInput))?;
    let is_hidden = file_name.to_string_lossy().starts_with('.');
    Ok(is_hidden)
}

pub(crate) fn hide(path: &Path) -> io::Result<()> {
    let dest_path = hidden_file_name(path).ok_or_else(|| Error::from(ErrorKind::InvalidInput))?;
    fs::rename(path, dest_path)
}

pub(crate) fn show(path: &Path) -> io::Result<()> {
    let dest_path = normal_file_name(path).ok_or_else(|| Error::from(ErrorKind::InvalidInput))?;
    fs::rename(path, dest_path)
}

/// Returns the path after making `path` invisible.
///
/// Returns [`None`] if `path` terminates in `..` or the file name starts with
/// `.`.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// #
/// assert_eq!(
///     hf::unix::hidden_file_name("file").unwrap(),
///     Path::new(".file")
/// );
/// ```
pub fn hidden_file_name(path: impl AsRef<Path>) -> Option<PathBuf> {
    let path = path.as_ref();
    let file_name = path
        .file_name()
        .map(OsStr::to_string_lossy)
        .filter(|n| !n.starts_with('.'))?;
    let dest_path = path.with_file_name(String::from('.') + &file_name);
    Some(dest_path)
}

/// Returns the path after making `path` visible.
///
/// Returns [`None`] if `path` terminates in `..` or the file name does not
/// start with `.`.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// #
/// assert_eq!(
///     hf::unix::normal_file_name(".file").unwrap(),
///     Path::new("file")
/// );
/// ```
pub fn normal_file_name(path: impl AsRef<Path>) -> Option<PathBuf> {
    let path = path.as_ref();
    let file_name = path
        .file_name()
        .map(OsStr::to_string_lossy)
        .filter(|n| n.starts_with('.'))?;
    let dest_path = path.with_file_name(file_name.trim_start_matches('.'));
    Some(dest_path)
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn is_hidden() {
        assert!(super::is_hidden(Path::new(".file")).unwrap());
        assert!(super::is_hidden(Path::new("path/to/.file")).unwrap());
    }

    #[test]
    fn is_hidden_when_non_hidden_file() {
        assert!(!super::is_hidden(Path::new("file")).unwrap());
        assert!(!super::is_hidden(Path::new("path/to/file")).unwrap());
    }

    #[test]
    fn hide() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir = temp_dir.path();
        let file_path = temp_dir.join("file");
        let hidden_file_path = temp_dir.join(".file");
        assert!(!file_path.exists());
        assert!(!hidden_file_path.exists());

        File::create(&file_path).unwrap();
        assert!(file_path.exists());
        assert!(!hidden_file_path.exists());

        super::hide(&file_path).unwrap();
        assert!(!file_path.exists());
        assert!(hidden_file_path.exists());
    }

    #[test]
    fn show() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir = temp_dir.path();
        let hidden_file_path = temp_dir.join(".file");
        let file_path = temp_dir.join("file");
        assert!(!hidden_file_path.exists());
        assert!(!file_path.exists());

        File::create(&hidden_file_path).unwrap();
        assert!(hidden_file_path.exists());
        assert!(!file_path.exists());

        super::show(&hidden_file_path).unwrap();
        assert!(!hidden_file_path.exists());
        assert!(file_path.exists());
    }

    #[test]
    fn hidden_file_name() {
        assert_eq!(super::hidden_file_name("file").unwrap(), Path::new(".file"));
        assert_eq!(
            super::hidden_file_name("path/to/file").unwrap(),
            Path::new("path/to/.file")
        );
    }

    #[test]
    fn normal_file_name() {
        assert_eq!(super::normal_file_name(".file").unwrap(), Path::new("file"));
        assert_eq!(
            super::normal_file_name("path/to/.file").unwrap(),
            Path::new("path/to/file")
        );
    }
}
