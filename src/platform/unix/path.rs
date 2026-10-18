// SPDX-FileCopyrightText: 2026 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Extensions to [`Path`].

use std::{
    ffi::OsStr,
    fs,
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
};

pub trait PathExt {
/// Returns the path after making `self` invisible.
///
/// Returns [`None`] if any of the following are true:
///
/// - `self` terminates in `..`.
/// - `self` is not valid UTF-8.
/// - The file name starts with `.`.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// assert_eq!(
///     Path::new("foo.txt").hidden_file_name().as_deref(),
///     Some(Path::new(".foo.txt"))
/// );
/// assert_eq!(
///     Path::new("foo/bar.txt").hidden_file_name().as_deref(),
///     Some(Path::new("foo/.bar.txt"))
/// );
///
/// assert_eq!(Path::new(".foo.txt").hidden_file_name(), None);
/// assert_eq!(Path::new("foo/.bar.txt").hidden_file_name(), None);
/// assert_eq!(Path::new("foo.txt/..").hidden_file_name(), None);
/// ```
fn hidden_file_name(&self) -> Option<PathBuf>;

/// Returns the path after making `self` visible.
///
/// Returns [`None`] if any of the following are true:
///
/// - `self` terminates in `..`.
/// - `self` is not valid UTF-8.
/// - The file name does not start with `.`.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// assert_eq!(
///     Path::new(".foo.txt").normal_file_name().as_deref(),
///     Some(Path::new("foo.txt"))
/// );
/// assert_eq!(
///     Path::new("foo/.bar.txt").normal_file_name().as_deref(),
///     Some(Path::new("foo/bar.txt"))
/// );
///
/// assert_eq!(Path::new("foo.txt").normal_file_name(), None);
/// assert_eq!(Path::new("foo/bar.txt").normal_file_name(), None);
/// assert_eq!(Path::new(".foo.txt/..").normal_file_name(), None);
/// ```
fn normal_file_name(&self) -> Option<PathBuf>;
}

impl PathExt for Path {
fn hidden_file_name(&self) -> Option<PathBuf> {
    super::hidden_file_name(self)
}

fn normal_file_name(&self) -> Option<PathBuf> {
    super::normal_file_name(self)
}
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::unix::ffi::OsStrExt};

    use super::*;

    #[test]
    fn hidden_file_name() {
        assert_eq!(
            Path::new("foo.txt").hidden_file_name().unwrap(),
            Path::new(".foo.txt")
        );
        assert_eq!(
            Path::new("ファイル.txt").hidden_file_name().unwrap(),
            Path::new(".ファイル.txt")
        );
        assert_eq!(
            Path::new("🦀.txt").hidden_file_name().unwrap(),
            Path::new(".🦀.txt")
        );
        assert_eq!(
            Path::new("foo/bar.txt").hidden_file_name().unwrap(),
            Path::new("foo/.bar.txt")
        );
        assert_eq!(
            Path::new(".foo/bar.txt").hidden_file_name().unwrap(),
            Path::new(".foo/.bar.txt")
        );
    }

    #[test]
    fn hidden_file_name_when_hidden_file() {
        assert!(Path::new(".foo.txt").hidden_file_name().is_none());
        assert!(Path::new("..foo.txt").hidden_file_name().is_none());
        assert!(Path::new("foo/.bar.txt").hidden_file_name().is_none());
        assert!(Path::new(".foo/.bar.txt").hidden_file_name().is_none());
    }

    #[test]
    fn hidden_file_name_with_invalid_path() {
        assert!(Path::new("foo.txt/..").hidden_file_name().is_none());
        assert!(Path::new("/").hidden_file_name().is_none());
    }

    #[test]
    fn hidden_file_name_with_non_utf8_path() {
        assert!(Path::new(OsStr::from_bytes(&[0x00, 0x9F, 0x92, 0x96])).hidden_file_name().is_none());
    }

    #[test]
    fn normal_file_name() {
        assert_eq!(
            Path::new(".foo.txt").normal_file_name().unwrap(),
            Path::new("foo.txt")
        );
        assert_eq!(
            Path::new("..foo.txt").normal_file_name().unwrap(),
            Path::new("foo.txt")
        );
        assert_eq!(
            Path::new(".ファイル.txt").normal_file_name().unwrap(),
            Path::new("ファイル.txt")
        );
        assert_eq!(
            Path::new(".🦀.txt").normal_file_name().unwrap(),
            Path::new("🦀.txt")
        );
        assert_eq!(
            Path::new("foo/.bar.txt").normal_file_name().unwrap(),
            Path::new("foo/bar.txt")
        );
        assert_eq!(
            Path::new(".foo/.bar.txt").normal_file_name().unwrap(),
            Path::new(".foo/bar.txt")
        );
    }

    #[test]
    fn normal_file_name_when_non_hidden_file() {
        assert!(Path::new("foo.txt").normal_file_name().is_none());
        assert!(Path::new("foo/bar.txt").normal_file_name().is_none());
        assert!(Path::new(".foo/bar.txt").normal_file_name().is_none());
    }

    #[test]
    fn normal_file_name_with_invalid_path() {
        assert!(Path::new(".foo.txt/..").normal_file_name().is_none());
        assert!(Path::new("/").normal_file_name().is_none());
    }

    #[test]
    fn normal_file_name_with_non_utf8_path() {
        assert!(
            Path::new(OsStr::from_bytes(&[0x2E, 0x00, 0x9F, 0x92, 0x96])).normal_file_name().is_none()
        );
    }
}
