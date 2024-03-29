// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `hf` crate is a cross-platform library for manipulating hidden files or
//! hidden directories.
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
//! let file_path = temp_dir.join("file");
//! let hidden_file_path = temp_dir.join(".file");
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
//! let file_path = temp_dir.path().join("file");
//!
//! File::create(&file_path).unwrap();
//! assert!(!hf::is_hidden(&file_path).unwrap());
//!
//! hf::hide(&file_path).unwrap();
//! assert!(hf::is_hidden(&file_path).unwrap());
//!
//! hf::show(&file_path).unwrap();
//! assert!(!hf::is_hidden(file_path).unwrap());
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/hf/0.3.0/")]
#![cfg_attr(doc_cfg, feature(doc_auto_cfg, doc_cfg))]
// Lint levels of rustc.
#![deny(missing_debug_implementations, missing_docs)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

mod ops;
mod platform;

pub use crate::ops::{hide, is_hidden, show};
#[cfg(unix)]
pub use crate::platform::unix;
