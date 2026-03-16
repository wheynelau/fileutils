# fileutils

Personal file utilities for reading and writing `.jsonl` and `.zst` files.

## Features

- `compression` (default) — zstd read/write support via [`zstd`](https://crates.io/crates/zstd)
- `progress` (default) — progress bar integration via [`indicatif`](https://crates.io/crates/indicatif)

## Usage

```toml
[dependencies]
fileutils = "0.1"
```

Disable default features if you don't need compression or progress:

```toml
[dependencies]
fileutils = { version = "0.1", default-features = false, features = ["compression"] }
```

## License

MIT
