use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

#[cfg(feature = "zstd")]
use crate::compression::CompressionConfig;

/// Wrap `writer` in the encoder matching `path`'s extension.
fn encode(
    path: &Path,
    writer: File,
    #[cfg(feature = "zstd")] config: Option<&CompressionConfig>,
) -> io::Result<Box<dyn Write>> {
    #[cfg(not(any(feature = "zstd", feature = "flate2")))]
    let _ = path;

    #[cfg(any(feature = "zstd", feature = "flate2"))]
    let ext_is = |ext: &str| {
        path.extension()
            .is_some_and(|e| e.eq_ignore_ascii_case(ext))
    };

    #[cfg(feature = "zstd")]
    if ext_is("zst") {
        let cfg = config.cloned().unwrap_or_default();
        return Ok(Box::new(cfg.build_encoder(writer)?.auto_finish()));
    }

    #[cfg(feature = "flate2")]
    if ext_is("gz") {
        return Ok(Box::new(flate2::write::GzEncoder::new(
            writer,
            flate2::Compression::default(),
        )));
    }

    Ok(Box::new(BufWriter::new(writer)))
}

/// # Errors
///
/// Returns an error if the file cannot be created or a configured encoder
/// fails to initialize.
#[cfg(feature = "zstd")]
pub fn setup_writer<P: AsRef<Path>>(
    filename: P,
    config: Option<&CompressionConfig>,
) -> io::Result<Box<dyn Write>> {
    let path = filename.as_ref();
    encode(path, File::create(path)?, config)
}

/// # Errors
///
/// Returns an error if the file cannot be created or a configured encoder
/// fails to initialize.
#[cfg(not(feature = "zstd"))]
pub fn setup_writer<P: AsRef<Path>>(filename: P) -> io::Result<Box<dyn Write>> {
    let path = filename.as_ref();
    encode(path, File::create(path)?)
}
