//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Returns `true` if the file is a hidden file.
pub fn is_hidden(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref();
    fs::metadata(path).map(|_| {
        path.file_name()
            .and_then(OsStr::to_str)
            .map_or(bool::default(), |name| name.starts_with('.'))
    })
}

/// Hide a file or directory.
pub fn hide(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    let dest_basename = path
        .file_name()
        .and_then(OsStr::to_str)
        .map(|name| '.'.to_string() + name)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
    fs::rename(path, path.with_file_name(dest_basename))
}

/// Show a hidden file or hidden directory.
pub fn show(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    let dest_basename = path
        .file_name()
        .and_then(OsStr::to_str)
        .and_then(|name| name.split_once('.'))
        .map(|(_, name)| name)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
    fs::rename(path, path.with_file_name(dest_basename))
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_is_hidden() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join(".file");
        File::create(&file_path).unwrap();
        assert!(is_hidden(file_path).unwrap());
    }

    #[test]
    fn test_is_not_hidden() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join("file");
        File::create(&file_path).unwrap();
        assert!(!is_hidden(file_path).unwrap());
    }

    #[test]
    fn test_is_hidden_when_file_does_not_exist() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join("file");
        assert!(is_hidden(file_path).is_err());
    }

    #[test]
    fn test_hide() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join("file");
        File::create(&file_path).unwrap();
        hide(&file_path).unwrap();
        assert!(!file_path.exists());
        assert!(tempdir.path().join(".file").exists());
    }

    #[test]
    fn test_show() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join(".file");
        File::create(&file_path).unwrap();
        show(&file_path).unwrap();
        assert!(!file_path.exists());
        assert!(tempdir.path().join("file").exists());
    }
}
