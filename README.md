# fileutils

[![CI](https://github.com/wheynelau/fileutils/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/wheynelau/fileutils/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/fileutils/badge.svg)](https://docs.rs/fileutils)
[![crates.io](https://img.shields.io/crates/v/fileutils.svg)](https://crates.io/crates/fileutils)

Personal file utilities for reading and writing `.jsonl`, `.zst`, `.gz` and `.bz2` files.

Reading detects the format from the file's leading magic bytes, so the file name is irrelevant and anything unrecognized is read as plain text.

## Features

- `zstd` (default) — zstd read/write support via [`zstd`](https://crates.io/crates/zstd)
- `progress` (default) — progress bar integration via [`indicatif`](https://crates.io/crates/indicatif)
- `flate2` — gzip `.gz` read/write support via [`flate2`](https://crates.io/crates/flate2)
- `bzip2` — `.bz2` read support via [`bzip2`](https://crates.io/crates/bzip2)

## Usage

```toml
[dependencies]
fileutils = "0.4"
```

Disable default features if you don't need compression or progress:

```toml
[dependencies]
fileutils = { version = "0.4", default-features = false, features = ["zstd"] }
```

## Re-exports

With the `zstd` feature enabled, the types for configuring it are re-exported, so `zstd` does not need to be a direct dependency:

- `fileutils::CompressionConfig` — zstd level and worker count
- `fileutils::Encoder`, `fileutils::Decoder` — zstd stream types

```rust
use fileutils::{CompressionConfig, write::setup_writer};

let config = CompressionConfig::new(5, 8);
let mut writer = setup_writer("out.zst", Some(&config))?;
```

## License

MIT
