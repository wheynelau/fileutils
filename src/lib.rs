pub mod read;
pub mod write;

#[cfg(feature = "compression")]
pub mod compression;

#[cfg(feature = "compression")]
pub use zstd::stream::{read::Decoder, write::Encoder};

#[cfg(feature = "compression")]
pub use compression::CompressionConfig;
