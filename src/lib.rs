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
//! ## On Unix
//!
//! ```
//! # #[cfg(unix)]
//! # {
//! use std::fs::File;
//!
//! let temp_dir = tempfile::tempdir().unwrap();
//! let temp_dir = temp_dir.path();
//! let file_path = temp_dir.join("foo.txt");
//! let hidden_file_path = hf::unix::hidden_file_name(&file_path).unwrap();
//! assert_eq!(hidden_file_path, temp_dir.join(".foo.txt"));
//! assert!(!file_path.exists());
//! assert!(!hidden_file_path.exists());
//!
//! File::create(&file_path).unwrap();
//! assert!(file_path.exists());
//! assert!(!hidden_file_path.exists());
//!
//! hf::hide(&file_path).unwrap();
//! assert!(!file_path.exists());
//! assert!(hidden_file_path.exists());
//!
//! hf::show(&hidden_file_path).unwrap();
//! assert!(file_path.exists());
//! assert!(!hidden_file_path.exists());
//! # }
//! ```
//!
//! ## On Windows
//!
//! ```
//! # #[cfg(windows)]
//! # {
//! use std::fs::File;
//!
//! let temp_dir = tempfile::tempdir().unwrap();
//! let file_path = temp_dir.path().join("foo.txt");
//! assert!(!file_path.exists());
//!
//! File::create(&file_path).unwrap();
//! assert!(file_path.exists());
//! assert_eq!(hf::is_hidden(&file_path).unwrap(), false);
//!
//! hf::hide(&file_path).unwrap();
//! assert!(file_path.exists());
//! assert_eq!(hf::is_hidden(&file_path).unwrap(), true);
//!
//! hf::show(&file_path).unwrap();
//! assert!(file_path.exists());
//! assert_eq!(hf::is_hidden(file_path).unwrap(), false);
//! # }
//! ```
//!
//! [hidden files and directories]: https://en.wikipedia.org/wiki/Hidden_file_and_hidden_directory

#![doc(html_root_url = "https://docs.rs/hf/0.3.10/")]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
// Lint levels of rustc.
#![deny(missing_docs)]

mod ops;
mod platform;

pub use crate::ops::{hide, is_hidden, show};
#[cfg(unix)]
pub use crate::platform::unix;
