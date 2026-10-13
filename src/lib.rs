//! Read and write `.jsonl`, `.zst`, `.gz` and `.bz2` files.
//!
//! Reading detects the format from the leading magic bytes; anything
//! unrecognized is read as plain text.
//!
//! # Example
//!
//! ```
//! use std::io::{Read, Write};
//!
//! # #[cfg(all(feature = "zstd", feature = "progress"))]
//! # fn main() -> std::io::Result<()> {
//! let path = std::env::temp_dir().join("fileutils-example.jsonl.zst");
//!
//! let mut writer = fileutils::write::setup_writer(&path, None)?;
//! writer.write_all(b"{\"x\":1}\n")?;
//! drop(writer);
//!
//! let mut text = String::new();
//! fileutils::read::setup_reader(&path, None)?.read_to_string(&mut text)?;
//! assert_eq!(text, "{\"x\":1}\n");
//! # Ok(())
//! # }
//! # #[cfg(not(all(feature = "zstd", feature = "progress")))]
//! # fn main() {}
//! ```
//!
//! Configure the zstd encoder with the re-exported [`CompressionConfig`], then
//! pass it to the writer.

pub mod read;
pub mod write;

#[cfg(feature = "zstd")]
pub mod compression;

#[cfg(feature = "zstd")]
pub use zstd::stream::{read::Decoder, write::Encoder};

#[cfg(feature = "zstd")]
#[doc(inline)]
pub use compression::CompressionConfig;
