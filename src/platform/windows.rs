// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Provides functionality for Windows.

use std::{
    fs,
    io::{self, Error},
    os::windows::fs::MetadataExt,
    path::Path,
};

use windows::{core::HSTRING, Win32::Storage::FileSystem};

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
    let path = HSTRING::from(path);
    unsafe { FileSystem::SetFileAttributesW(&path, attributes) }.map_err(Error::from)
}

pub fn show(path: &Path) -> io::Result<()> {
    let attributes = get_file_attributes(path)? & !FileSystem::FILE_ATTRIBUTE_HIDDEN;
    let path = HSTRING::from(path);
    unsafe { FileSystem::SetFileAttributesW(&path, attributes) }.map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::ErrorKind, process::Command};

    use super::*;

    #[test]
    fn is_hidden() {
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("foo.txt");
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();

            Command::new("attrib")
                .arg("+h")
                .arg(&file_path)
                .status()
                .unwrap();
            assert!(super::is_hidden(&file_path).unwrap());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("\u{30D5}\u{30A1}\u{30A4}\u{30EB}.txt");
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();

            Command::new("attrib")
                .arg("+h")
                .arg(&file_path)
                .status()
                .unwrap();
            assert!(super::is_hidden(&file_path).unwrap());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("foo/bar.txt");
            fs::create_dir(file_path.parent().unwrap()).unwrap();
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();

            Command::new("attrib")
                .arg("+h")
                .arg(&file_path)
                .status()
                .unwrap();
            assert!(super::is_hidden(&file_path).unwrap());
        }
    }

    #[test]
    fn is_hidden_when_non_hidden_file() {
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("foo.txt");
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();

            assert!(!super::is_hidden(&file_path).unwrap());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("\u{30D5}\u{30A1}\u{30A4}\u{30EB}.txt");
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();

            assert!(!super::is_hidden(&file_path).unwrap());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("foo/bar.txt");
            fs::create_dir(file_path.parent().unwrap()).unwrap();
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();

            assert!(!super::is_hidden(&file_path).unwrap());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("foo/bar.txt");
            fs::create_dir(file_path.parent().unwrap()).unwrap();
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();

            Command::new("attrib")
                .arg("+h")
                .arg(file_path.parent().unwrap())
                .status()
                .unwrap();
            assert!(!super::is_hidden(&file_path).unwrap());
        }
    }

    #[test]
    fn is_hidden_when_file_does_not_exist() {
        {
            let file_path = Path::new("foo.txt");
            assert_eq!(
                super::is_hidden(&file_path).unwrap_err().kind(),
                ErrorKind::NotFound
            );
        }
        {
            let file_path = Path::new("foo/bar.txt");
            assert_eq!(
                super::is_hidden(&file_path).unwrap_err().kind(),
                ErrorKind::NotFound
            );
        }
    }

    #[test]
    fn hide() {
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("foo.txt");
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();
            assert!(!super::is_hidden(&file_path).unwrap());

            super::hide(&file_path).unwrap();
            assert!(super::is_hidden(&file_path).unwrap());
        }
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("\u{30D5}\u{30A1}\u{30A4}\u{30EB}.txt");
            assert!(!file_path.exists());

            File::create(&file_path).unwrap();
            assert!(!super::is_hidden(&file_path).unwrap());

            super::hide(&file_path).unwrap();
            assert!(super::is_hidden(&file_path).unwrap());
        }
    }

    #[test]
    fn hide_when_file_does_not_exist() {
        let file_path = Path::new("foo.txt");
        assert_eq!(
            super::hide(&file_path).unwrap_err().kind(),
            ErrorKind::NotFound
        );
    }

    #[test]
    fn show() {
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("foo.txt");
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
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("\u{30D5}\u{30A1}\u{30A4}\u{30EB}.txt");
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

    #[test]
    fn show_when_file_does_not_exist() {
        let file_path = Path::new("foo.txt");
        assert_eq!(
            super::show(&file_path).unwrap_err().kind(),
            ErrorKind::NotFound
        );
    }
}
