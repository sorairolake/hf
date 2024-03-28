// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    ffi::OsStr,
    fs,
    io::{self, Error, ErrorKind},
    path::Path,
};

pub fn is_hidden(path: &Path) -> io::Result<bool> {
    let file_name = path
        .file_name()
        .ok_or_else(|| Error::from(ErrorKind::InvalidInput))?;
    let is_hidden = file_name.to_string_lossy().starts_with('.');
    Ok(is_hidden)
}

pub fn hide(path: &Path) -> io::Result<()> {
    let file_name = path
        .file_name()
        .map(OsStr::to_string_lossy)
        .filter(|n| !n.starts_with('.'))
        .ok_or_else(|| Error::from(ErrorKind::InvalidInput))?;
    let dest_file_name = String::from('.') + &file_name;
    fs::rename(path, path.with_file_name(dest_file_name))
}

pub fn show(path: &Path) -> io::Result<()> {
    let file_name = path
        .file_name()
        .map(OsStr::to_string_lossy)
        .filter(|n| n.starts_with('.'))
        .ok_or_else(|| Error::from(ErrorKind::InvalidInput))?;
    let dest_file_name = file_name.trim_start_matches('.');
    fs::rename(path, path.with_file_name(dest_file_name))
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn is_hidden() {
        assert!(super::is_hidden(Path::new(".file")).unwrap());
        assert!(super::is_hidden(Path::new("path/to/.file")).unwrap());
    }

    #[test]
    fn is_hidden_when_non_hidden_file() {
        assert!(!super::is_hidden(Path::new("file")).unwrap());
        assert!(!super::is_hidden(Path::new("path/to/file")).unwrap());
    }

    #[test]
    fn hide() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir = temp_dir.path();
        let file_path = temp_dir.join("file");
        let hidden_file_path = temp_dir.join(".file");
        assert!(!file_path.exists());
        assert!(!hidden_file_path.exists());

        File::create(&file_path).unwrap();
        assert!(file_path.exists());
        assert!(!hidden_file_path.exists());

        super::hide(&file_path).unwrap();
        assert!(!file_path.exists());
        assert!(hidden_file_path.exists());
    }

    #[test]
    fn show() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir = temp_dir.path();
        let hidden_file_path = temp_dir.join(".file");
        let file_path = temp_dir.join("file");
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
