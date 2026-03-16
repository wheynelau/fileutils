use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

#[cfg(feature = "compression")]
use crate::compression::CompressionConfig;

/// # Errors
///
/// Returns an error if the file cannot be created or the zstd encoder fails
/// to initialize.
#[cfg(feature = "compression")]
pub fn setup_writer<P: AsRef<Path>>(
    filename: P,
    config: Option<&CompressionConfig>,
) -> io::Result<Box<dyn Write>> {
    let path = filename.as_ref();
    let outfile = File::create(path)?;

    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("zst"))
    {
        let cfg = config.cloned().unwrap_or_default();
        let encoder = cfg.build_encoder(outfile)?;
        Ok(Box::new(encoder.auto_finish()))
    } else {
        Ok(Box::new(BufWriter::new(outfile)))
    }
}

/// # Errors
///
/// Returns an error if the file cannot be created.
#[cfg(not(feature = "compression"))]
pub fn setup_writer<P: AsRef<Path>>(filename: P) -> io::Result<Box<dyn Write>> {
    let outfile = File::create(filename.as_ref())?;
    Ok(Box::new(BufWriter::new(outfile)))
}
