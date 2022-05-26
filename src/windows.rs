//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::fs;
use std::io;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::process::{Command, ExitStatus};

/// Returns `true` if the file is a hidden file.
pub fn is_hidden(path: impl AsRef<Path>) -> io::Result<bool> {
    let attributes = fs::metadata(path.as_ref())?.file_attributes();

    Ok((attributes & 0x2) > 0)
}

/// Hide a file or directory.
pub fn hide(path: impl AsRef<Path>) -> io::Result<()> {
    let status = Command::new("attrib").arg("+h").arg(path).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::from(io::ErrorKind::Other))
    }
}

/// Show a hidden file or hidden directory.
pub fn show(path: impl AsRef<Path>) -> io::Result<()> {
    let status = Command::new("attrib").arg("-h").arg(path).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::from(io::ErrorKind::Other))
    }
}
