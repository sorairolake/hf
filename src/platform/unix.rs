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
///     hf::unix::hidden_file_name("foo/bar.txt").unwrap(),
///     Path::new("foo/.bar.txt")
/// );
///
/// assert!(hf::unix::hidden_file_name(".foo.txt").is_none());
/// assert!(hf::unix::hidden_file_name("foo.txt/..").is_none());
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
///     hf::unix::normal_file_name("foo/.bar.txt").unwrap(),
///     Path::new("foo/bar.txt")
/// );
///
/// assert!(hf::unix::normal_file_name("foo.txt").is_none());
/// assert!(hf::unix::normal_file_name(".foo.txt/..").is_none());
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
        assert!(super::is_hidden(Path::new(".foo.txt")).unwrap());
        assert!(super::is_hidden(Path::new("..foo.txt")).unwrap());
        assert!(super::is_hidden(Path::new("foo/.bar.txt")).unwrap());
        assert!(super::is_hidden(Path::new(".foo/.bar.txt")).unwrap());
    }

    #[test]
    fn is_hidden_when_non_hidden_file() {
        assert!(!super::is_hidden(Path::new("foo.txt")).unwrap());
        assert!(!super::is_hidden(Path::new("foo/bar.txt")).unwrap());
        assert!(!super::is_hidden(Path::new(".foo/bar.txt")).unwrap());
    }

    #[test]
    fn is_hidden_with_invalid_path() {
        assert_eq!(
            super::is_hidden(Path::new(".foo.txt/.."))
                .unwrap_err()
                .kind(),
            ErrorKind::InvalidInput
        );
        assert_eq!(
            super::is_hidden(Path::new("foo.txt/.."))
                .unwrap_err()
                .kind(),
            ErrorKind::InvalidInput
        );
        assert_eq!(
            super::is_hidden(Path::new("/")).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }

    #[test]
    fn hide() {
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let temp_dir = temp_dir.path();
            let file_path = temp_dir.join("foo.txt");
            let hidden_file_path = temp_dir.join(".foo.txt");
            assert!(!file_path.exists());
            assert!(!hidden_file_path.exists());

            File::create(&file_path).unwrap();
            assert!(file_path.exists());
            assert!(!hidden_file_path.exists());

            super::hide(&file_path).unwrap();
            assert!(!file_path.exists());
            assert!(hidden_file_path.exists());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let temp_dir = temp_dir.path();
            let parent_dir = temp_dir.join("foo");
            let file_path = parent_dir.join("bar.txt");
            let hidden_file_path = parent_dir.join(".bar.txt");
            fs::create_dir(parent_dir).unwrap();
            assert!(!file_path.exists());
            assert!(!hidden_file_path.exists());

            File::create(&file_path).unwrap();
            assert!(file_path.exists());
            assert!(!hidden_file_path.exists());

            super::hide(&file_path).unwrap();
            assert!(!file_path.exists());
            assert!(hidden_file_path.exists());
        }
    }

    #[test]
    fn hide_when_hidden_file() {
        {
            let hidden_file_path = Path::new(".foo.txt");
            assert_eq!(
                super::hide(hidden_file_path).unwrap_err().kind(),
                ErrorKind::InvalidInput
            );
        }
        {
            let hidden_file_path = Path::new("..foo.txt");
            assert_eq!(
                super::hide(hidden_file_path).unwrap_err().kind(),
                ErrorKind::InvalidInput
            );
        }
        {
            let hidden_file_path = Path::new("foo/.bar.txt");
            assert_eq!(
                super::hide(hidden_file_path).unwrap_err().kind(),
                ErrorKind::InvalidInput
            );
        }
    }

    #[test]
    fn hide_with_invalid_path() {
        let file_path = Path::new("foo.txt/..");
        assert_eq!(
            super::hide(file_path).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }

    #[test]
    fn hide_when_file_does_not_exist() {
        let file_path = Path::new("foo.txt");
        assert_eq!(
            super::hide(file_path).unwrap_err().kind(),
            ErrorKind::NotFound
        );
    }

    #[test]
    fn show() {
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let temp_dir = temp_dir.path();
            let hidden_file_path = temp_dir.join(".foo.txt");
            let file_path = temp_dir.join("foo.txt");
            assert!(!hidden_file_path.exists());
            assert!(!file_path.exists());

            File::create(&hidden_file_path).unwrap();
            assert!(hidden_file_path.exists());
            assert!(!file_path.exists());

            super::show(&hidden_file_path).unwrap();
            assert!(!hidden_file_path.exists());
            assert!(file_path.exists());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let temp_dir = temp_dir.path();
            let hidden_file_path = temp_dir.join("..foo.txt");
            let file_path = temp_dir.join("foo.txt");
            assert!(!hidden_file_path.exists());
            assert!(!file_path.exists());

            File::create(&hidden_file_path).unwrap();
            assert!(hidden_file_path.exists());
            assert!(!file_path.exists());

            super::show(&hidden_file_path).unwrap();
            assert!(!hidden_file_path.exists());
            assert!(file_path.exists());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let temp_dir = temp_dir.path();
            let parent_dir = temp_dir.join("foo");
            let hidden_file_path = parent_dir.join(".bar.txt");
            let file_path = parent_dir.join("bar.txt");
            fs::create_dir(parent_dir).unwrap();
            assert!(!hidden_file_path.exists());
            assert!(!file_path.exists());

            File::create(&hidden_file_path).unwrap();
            assert!(hidden_file_path.exists());
            assert!(!file_path.exists());

            super::show(&hidden_file_path).unwrap();
            assert!(!hidden_file_path.exists());
            assert!(file_path.exists());
        }
    }

    #[test]
    fn show_when_non_hidden_file() {
        {
            let file_path = Path::new("foo.txt");
            assert_eq!(
                super::show(file_path).unwrap_err().kind(),
                ErrorKind::InvalidInput
            );
        }
        {
            let file_path = Path::new("foo/bar.txt");
            assert_eq!(
                super::show(file_path).unwrap_err().kind(),
                ErrorKind::InvalidInput
            );
        }
    }

    #[test]
    fn show_with_invalid_path() {
        let hidden_file_path = Path::new(".foo.txt/..");
        assert_eq!(
            super::show(hidden_file_path).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }

    #[test]
    fn show_when_file_does_not_exist() {
        let hidden_file_path = Path::new(".foo.txt");
        assert_eq!(
            super::show(hidden_file_path).unwrap_err().kind(),
            ErrorKind::NotFound
        );
    }

    #[test]
    fn hidden_file_name() {
        assert_eq!(
            super::hidden_file_name("foo.txt").unwrap(),
            Path::new(".foo.txt")
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
        assert!(super::hidden_file_name("foo/.bar.txt").is_none(),);
        assert!(super::hidden_file_name(".foo/.bar.txt").is_none());
    }

    #[test]
    fn hidden_file_name_with_invalid_path() {
        assert!(super::hidden_file_name("foo.txt/..").is_none());
        assert!(super::hidden_file_name("/").is_none(),);
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
        assert!(super::normal_file_name("foo/bar.txt").is_none(),);
        assert!(super::normal_file_name(".foo/bar.txt").is_none());
    }

    #[test]
    fn normal_file_name_with_invalid_path() {
        assert!(super::normal_file_name(".foo.txt/..").is_none());
        assert!(super::normal_file_name("/").is_none(),);
    }
}
