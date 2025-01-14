// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `hf` crate is a cross-platform library for manipulating [hidden files
//! and directories].
//!
//! This crate supports both Unix and Windows. On Unix, hidden files and
//! directories are files and directories that starts with a dot character
//! (`.`). On Windows, hidden files and directories are files and directories
//! with the hidden file attribute.
//!
//! [hidden files and directories]: https://en.wikipedia.org/wiki/Hidden_file_and_hidden_directory

#![doc(html_root_url = "https://docs.rs/hf/0.3.8/")]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
// Lint levels of rustc.
#![deny(missing_docs)]

mod ops;
mod platform;

pub use crate::ops::{hide, is_hidden, show};
#[cfg(unix)]
pub use crate::platform::unix;
