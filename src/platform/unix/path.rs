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
