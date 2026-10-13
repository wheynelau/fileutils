# fileutils

Personal file utilities for reading and writing `.jsonl`, `.zst`, `.gz` and `.bz2` files.

## Features

- `zstd` (default) — zstd read/write support via [`zstd`](https://crates.io/crates/zstd)
- `progress` (default) — progress bar integration via [`indicatif`](https://crates.io/crates/indicatif)
- `flate2` — gzip `.gz` read/write support via [`flate2`](https://crates.io/crates/flate2)
- `bzip2` — `.bz2` read support via [`bzip2`](https://crates.io/crates/bzip2)

## Usage

```toml
[dependencies]
fileutils = "0.3"
```

Disable default features if you don't need compression or progress:

```toml
[dependencies]
fileutils = { version = "0.3", default-features = false, features = ["zstd"] }
```

## License

MIT
