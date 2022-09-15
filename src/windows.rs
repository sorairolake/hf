//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::ffi::CString;
use std::fs;
use std::io;
use std::os::windows::fs::MetadataExt;
use std::path::Path;

use windows::{core::PCSTR, Win32::Storage::FileSystem};

fn get_file_attributes(
    path: impl AsRef<Path>,
) -> io::Result<FileSystem::FILE_FLAGS_AND_ATTRIBUTES> {
    let attributes = fs::metadata(path.as_ref())?.file_attributes();
    Ok(FileSystem::FILE_FLAGS_AND_ATTRIBUTES(attributes))
}

/// Returns `true` if the file is a hidden file.
pub fn is_hidden(path: impl AsRef<Path>) -> io::Result<bool> {
    let attributes = get_file_attributes(path)?;
    Ok((attributes & FileSystem::FILE_ATTRIBUTE_HIDDEN).0 > 0)
}

/// Hide a file or directory.
pub fn hide(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_str().unwrap_or_default();
    let attributes = get_file_attributes(path)? | FileSystem::FILE_ATTRIBUTE_HIDDEN;
    let path = CString::new(path).map_err(io::Error::from)?;
    let bytes = path.as_bytes_with_nul();
    let path = PCSTR::from_raw(bytes.as_ptr());
    unsafe { FileSystem::SetFileAttributesA(path, attributes) }
        .ok()
        .map_err(io::Error::from)
}

/// Show a hidden file or hidden directory.
pub fn show(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_str().unwrap_or_default();
    let attributes = get_file_attributes(path)? & !FileSystem::FILE_ATTRIBUTE_HIDDEN;
    let path = CString::new(path).map_err(io::Error::from)?;
    let bytes = path.as_bytes_with_nul();
    let path = PCSTR::from_raw(bytes.as_ptr());
    unsafe { FileSystem::SetFileAttributesA(path, attributes) }
        .ok()
        .map_err(io::Error::from)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::process::Command;

    use super::*;

    #[test]
    fn test_is_hidden() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join("file");
        File::create(&file_path).unwrap();
        Command::new("attrib")
            .arg("+h")
            .arg(&file_path)
            .status()
            .unwrap();
        assert!(is_hidden(file_path).unwrap());
    }

    #[test]
    fn test_is_not_hidden() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join("file");
        File::create(&file_path).unwrap();
        Command::new("attrib")
            .arg("-h")
            .arg(&file_path)
            .status()
            .unwrap();
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
        Command::new("attrib")
            .arg("-h")
            .arg(&file_path)
            .status()
            .unwrap();
        hide(&file_path).unwrap();
        assert!(is_hidden(file_path).unwrap());
    }

    #[test]
    fn test_show() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join("file");
        File::create(&file_path).unwrap();
        Command::new("attrib")
            .arg("+h")
            .arg(&file_path)
            .status()
            .unwrap();
        show(&file_path).unwrap();
        assert!(!is_hidden(file_path).unwrap());
    }
}
