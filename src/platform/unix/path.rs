// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Provides functionality for Unix platforms.

use std::{
    ffi::OsStr,
    fs,
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
};

/// Returns the path after making `path` invisible.
///
/// Returns [`None`] if any of the following are true:
///
/// - `path` terminates in `..`.
/// - `path` is not valid UTF-8.
/// - The file name starts with `.`.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// assert_eq!(
///     hf::unix::hidden_file_name("foo.txt").as_deref(),
///     Some(Path::new(".foo.txt"))
/// );
/// assert_eq!(
///     hf::unix::hidden_file_name("foo/bar.txt").as_deref(),
///     Some(Path::new("foo/.bar.txt"))
/// );
///
/// assert_eq!(hf::unix::hidden_file_name(".foo.txt"), None);
/// assert_eq!(hf::unix::hidden_file_name("foo/.bar.txt"), None);
/// assert_eq!(hf::unix::hidden_file_name("foo.txt/.."), None);
/// ```
pub fn hidden_file_name(path: impl AsRef<Path>) -> Option<PathBuf> {
    let inner = |path: &Path| -> Option<PathBuf> {
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .filter(|n| !n.starts_with('.'))?;
        let file_name = String::from('.') + file_name;
        Some(path.with_file_name(file_name))
    };
    inner(path.as_ref())
}

/// Returns the path after making `path` visible.
///
/// Returns [`None`] if any of the following are true:
///
/// - `path` terminates in `..`.
/// - `path` is not valid UTF-8.
/// - The file name does not start with `.`.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// assert_eq!(
///     hf::unix::normal_file_name(".foo.txt").as_deref(),
///     Some(Path::new("foo.txt"))
/// );
/// assert_eq!(
///     hf::unix::normal_file_name("foo/.bar.txt").as_deref(),
///     Some(Path::new("foo/bar.txt"))
/// );
///
/// assert_eq!(hf::unix::normal_file_name("foo.txt"), None);
/// assert_eq!(hf::unix::normal_file_name("foo/bar.txt"), None);
/// assert_eq!(hf::unix::normal_file_name(".foo.txt/.."), None);
/// ```
pub fn normal_file_name(path: impl AsRef<Path>) -> Option<PathBuf> {
    let inner = |path: &Path| -> Option<PathBuf> {
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .filter(|n| n.starts_with('.'))?;
        let file_name = file_name.trim_start_matches('.');
        Some(path.with_file_name(file_name))
    };
    inner(path.as_ref())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::unix::ffi::OsStrExt};

    use super::*;

    #[test]
    fn hidden_file_name() {
        assert_eq!(
            super::hidden_file_name("foo.txt").unwrap(),
            Path::new(".foo.txt")
        );
        assert_eq!(
            super::hidden_file_name("ファイル.txt").unwrap(),
            Path::new(".ファイル.txt")
        );
        assert_eq!(
            super::hidden_file_name("🦀.txt").unwrap(),
            Path::new(".🦀.txt")
        );
        assert_eq!(
            super::hidden_file_name("foo/bar.txt").unwrap(),
            Path::new("foo/.bar.txt")
        );
        assert_eq!(
            super::hidden_file_name(".foo/bar.txt").unwrap(),
            Path::new(".foo/.bar.txt")
        );
    }

    #[test]
    fn hidden_file_name_when_hidden_file() {
        assert!(super::hidden_file_name(".foo.txt").is_none());
        assert!(super::hidden_file_name("..foo.txt").is_none());
        assert!(super::hidden_file_name("foo/.bar.txt").is_none());
        assert!(super::hidden_file_name(".foo/.bar.txt").is_none());
    }

    #[test]
    fn hidden_file_name_with_invalid_path() {
        assert!(super::hidden_file_name("foo.txt/..").is_none());
        assert!(super::hidden_file_name("/").is_none());
    }

    #[test]
    fn hidden_file_name_with_non_utf8_path() {
        assert!(super::hidden_file_name(OsStr::from_bytes(&[0x00, 0x9F, 0x92, 0x96])).is_none());
    }

    #[test]
    fn normal_file_name() {
        assert_eq!(
            super::normal_file_name(".foo.txt").unwrap(),
            Path::new("foo.txt")
        );
        assert_eq!(
            super::normal_file_name("..foo.txt").unwrap(),
            Path::new("foo.txt")
        );
        assert_eq!(
            super::normal_file_name(".ファイル.txt").unwrap(),
            Path::new("ファイル.txt")
        );
        assert_eq!(
            super::normal_file_name(".🦀.txt").unwrap(),
            Path::new("🦀.txt")
        );
        assert_eq!(
            super::normal_file_name("foo/.bar.txt").unwrap(),
            Path::new("foo/bar.txt")
        );
        assert_eq!(
            super::normal_file_name(".foo/.bar.txt").unwrap(),
            Path::new(".foo/bar.txt")
        );
    }

    #[test]
    fn normal_file_name_when_non_hidden_file() {
        assert!(super::normal_file_name("foo.txt").is_none());
        assert!(super::normal_file_name("foo/bar.txt").is_none());
        assert!(super::normal_file_name(".foo/bar.txt").is_none());
    }

    #[test]
    fn normal_file_name_with_invalid_path() {
        assert!(super::normal_file_name(".foo.txt/..").is_none());
        assert!(super::normal_file_name("/").is_none());
    }

    #[test]
    fn normal_file_name_with_non_utf8_path() {
        assert!(
            super::normal_file_name(OsStr::from_bytes(&[0x2E, 0x00, 0x9F, 0x92, 0x96])).is_none()
        );
    }
}
