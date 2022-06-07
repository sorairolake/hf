//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::fs;
use std::io;
use std::os::windows::fs::MetadataExt;
use std::path::Path;

use windows::Win32::Storage::FileSystem::{
    SetFileAttributesA, FILE_ATTRIBUTE_HIDDEN, FILE_FLAGS_AND_ATTRIBUTES,
};

fn get_file_attributes(path: impl AsRef<Path>) -> io::Result<FILE_FLAGS_AND_ATTRIBUTES> {
    let attributes = fs::metadata(path.as_ref())?.file_attributes();

    Ok(FILE_FLAGS_AND_ATTRIBUTES(attributes))
}

/// Returns `true` if the file is a hidden file.
pub fn is_hidden(path: impl AsRef<Path>) -> io::Result<bool> {
    let attributes = get_file_attributes(path)?;

    Ok((attributes & FILE_ATTRIBUTE_HIDDEN).0 > 0)
}

/// Hide a file or directory.
pub fn hide(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path
        .as_ref()
        .to_str()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;

    let attributes = get_file_attributes(path)? | FILE_ATTRIBUTE_HIDDEN;

    unsafe { SetFileAttributesA(path, attributes) }
        .ok()
        .map_err(io::Error::from)
}

/// Show a hidden file or hidden directory.
pub fn show(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path
        .as_ref()
        .to_str()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;

    let attributes = get_file_attributes(path)? & !FILE_ATTRIBUTE_HIDDEN;

    unsafe { SetFileAttributesA(path, attributes) }
        .ok()
        .map_err(io::Error::from)
}
