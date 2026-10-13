pub mod read;
pub mod write;

#[cfg(feature = "zstd")]
pub mod compression;

#[cfg(feature = "zstd")]
pub use zstd::stream::{read::Decoder, write::Encoder};

#[cfg(feature = "zstd")]
pub use compression::CompressionConfig;
