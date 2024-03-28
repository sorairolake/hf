// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    ffi::CString,
    fs,
    io::{self, Error},
    os::windows::fs::MetadataExt,
    path::Path,
};

use windows::{core::PCSTR, Win32::Storage::FileSystem};

fn get_file_attributes(path: &Path) -> io::Result<FileSystem::FILE_FLAGS_AND_ATTRIBUTES> {
    let attributes = fs::metadata(path)?.file_attributes();
    let attributes = FileSystem::FILE_FLAGS_AND_ATTRIBUTES(attributes);
    Ok(attributes)
}

pub fn is_hidden(path: &Path) -> io::Result<bool> {
    let attributes = get_file_attributes(path)?;
    let is_hidden = (attributes & FileSystem::FILE_ATTRIBUTE_HIDDEN).0 > 0;
    Ok(is_hidden)
}

pub fn hide(path: &Path) -> io::Result<()> {
    let attributes = get_file_attributes(path)? | FileSystem::FILE_ATTRIBUTE_HIDDEN;
    let path = path.to_string_lossy();
    let path = CString::new(path.as_bytes()).map_err(Error::from)?;
    let path = path.as_bytes_with_nul();
    let path = PCSTR::from_raw(path.as_ptr());
    unsafe { FileSystem::SetFileAttributesA(path, attributes) }.map_err(Error::from)
}

pub fn show(path: &Path) -> io::Result<()> {
    let attributes = get_file_attributes(path)? & !FileSystem::FILE_ATTRIBUTE_HIDDEN;
    let path = path.to_string_lossy();
    let path = CString::new(path.as_bytes()).map_err(Error::from)?;
    let path = path.as_bytes_with_nul();
    let path = PCSTR::from_raw(path.as_ptr());
    unsafe { FileSystem::SetFileAttributesA(path, attributes) }.map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, process::Command};

    #[test]
    fn is_hidden() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file");
        assert!(!file_path.exists());

        File::create(&file_path).unwrap();

        Command::new("attrib")
            .arg("+h")
            .arg(&file_path)
            .status()
            .unwrap();
        assert!(super::is_hidden(&file_path).unwrap());
    }

    #[test]
    fn is_hidden_when_non_hidden_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file");
        assert!(!file_path.exists());

        File::create(&file_path).unwrap();

        assert!(!super::is_hidden(&file_path).unwrap());
    }

    #[test]
    fn is_hidden_when_file_does_not_exist() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file");
        assert!(!file_path.exists());

        assert!(super::is_hidden(&file_path).is_err());
    }

    #[test]
    fn hide() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file");
        assert!(!file_path.exists());

        File::create(&file_path).unwrap();
        assert!(!super::is_hidden(&file_path).unwrap());

        super::hide(&file_path).unwrap();
        assert!(super::is_hidden(&file_path).unwrap());
    }

    #[test]
    fn show() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file");
        assert!(!file_path.exists());

        File::create(&file_path).unwrap();
        assert!(!super::is_hidden(&file_path).unwrap());

        Command::new("attrib")
            .arg("+h")
            .arg(&file_path)
            .status()
            .unwrap();
        assert!(super::is_hidden(&file_path).unwrap());

        super::show(&file_path).unwrap();
        assert!(!super::is_hidden(&file_path).unwrap());
    }
}
