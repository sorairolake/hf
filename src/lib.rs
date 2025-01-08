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
//! # Examples
//!
//! ```
//! use std::fs::File;
//!
//! let temp_dir = tempfile::tempdir().unwrap();
//! let file_path = temp_dir.path().join("foo.txt");
//! # assert!(!file_path.exists());
//!
//! File::create(&file_path).unwrap();
//! # #[cfg(unix)]
//! # assert!(file_path.exists());
//! assert!(!hf::is_hidden(&file_path).unwrap());
//!
//! hf::hide(&file_path).unwrap();
//! # #[cfg(unix)]
//! # assert!(!file_path.exists());
//! // Change the file name to start with `.`.
//! #[cfg(unix)]
//! let file_path = hf::unix::hidden_file_name(&file_path).unwrap();
//! # #[cfg(unix)]
//! # assert!(file_path.exists());
//! assert!(hf::is_hidden(&file_path).unwrap());
//!
//! hf::show(&file_path).unwrap();
//! # #[cfg(unix)]
//! # assert!(!file_path.exists());
//! // Change the file name to start with a character other than `.`.
//! #[cfg(unix)]
//! let file_path = hf::unix::normal_file_name(&file_path).unwrap();
//! # #[cfg(unix)]
//! # assert!(file_path.exists());
//! assert!(!hf::is_hidden(file_path).unwrap());
//! ```
//!
//! [hidden files and directories]: https://en.wikipedia.org/wiki/Hidden_file_and_hidden_directory

#![doc(html_root_url = "https://docs.rs/hf/0.3.6/")]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
// Lint levels of rustc.
#![deny(missing_docs)]

mod ops;
mod platform;

pub use crate::ops::{hide, is_hidden, show};
#[cfg(unix)]
pub use crate::platform::unix;
