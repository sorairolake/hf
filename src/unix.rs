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

    if let Err(err) = fs::metadata(path) {
        Err(err)
    } else {
        Ok(path
            .file_name()
            .and_then(OsStr::to_str)
            .map_or(bool::default(), |name| name.starts_with('.')))
    }
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
