<!--
SPDX-FileCopyrightText: 2022 Shun Sakai

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# hf

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![MSRV][msrv-badge]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**hf** is a cross-platform hidden file library and utility.

## Installation

### From source

```sh
cargo install hf
```

### From binaries

The [release page] contains pre-built binaries for Linux, macOS and Windows.

### How to build

Please see [BUILD.adoc].

## Usage

### Make files invisible

Don't actually hide anything, just show what would be done:

```sh
hf hide -n data.txt
```

Actually hide files:

```sh
hf hide -f data.txt
```

### Make hidden files visible

Don't actually show anything, just show what would be done:

```sh
hf show -n .data.txt
```

Actually show hidden files:

```sh
hf show -f .data.txt
```

### Generate shell completion

`--generate-completion` option generates shell completions to standard output.

The following shells are supported:

- `bash`
- `elvish`
- `fish`
- `nushell`
- `powershell`
- `zsh`

Example:

```sh
hf --generate-completion bash > hf.bash
```

## Use as a library

This crate is also available as a library.

Add this to your `Cargo.toml` to use it as a library:

```toml
[dependencies]
hf = { version = "0.3.6", default-features = false }
```

By default, the dependencies required to build the application are also built.
If you disable the `default` feature, only the dependencies required to build
the library will be built.

### Documentation

See the [documentation][docs-url] for more details.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.74.0.

## Command-line options

Please see the following:

- [`hf(1)`]
- [`hf-hide(1)`]
- [`hf-show(1)`]
- [`hf-help(1)`]

## Source code

The upstream repository is available at
<https://github.com/sorairolake/hf.git>.

The source code is also available at:

- <https://gitlab.com/sorairolake/hf.git>
- <https://codeberg.org/sorairolake/hf.git>

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## License

Copyright (C) 2022-2024 Shun Sakai (see [AUTHORS.adoc])

1.  This program is distributed under the terms of either the _Apache License
    2.0_ or the _MIT License_.
2.  Some files are distributed under the terms of the _Creative Commons
    Attribution 4.0 International Public License_.

This project is compliant with version 3.2 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/hf/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/hf/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/hf?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/hf
[msrv-badge]: https://img.shields.io/crates/msrv/hf?style=for-the-badge&logo=rust
[docs-badge]: https://img.shields.io/docsrs/hf?style=for-the-badge&logo=docsdotrs&label=Docs.rs
[docs-url]: https://docs.rs/hf
[license-badge]: https://img.shields.io/crates/l/hf?style=for-the-badge
[release page]: https://github.com/sorairolake/hf/releases
[BUILD.adoc]: BUILD.adoc
[`hf(1)`]: https://sorairolake.github.io/hf/book/man/man1/hf.1.html
[`hf-hide(1)`]: https://sorairolake.github.io/hf/book/man/man1/hf-hide.1.html
[`hf-show(1)`]: https://sorairolake.github.io/hf/book/man/man1/hf-show.1.html
[`hf-help(1)`]: https://sorairolake.github.io/hf/book/man/man1/hf-help.1.html
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec/
