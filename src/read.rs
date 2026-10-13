use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[cfg(feature = "progress")]
use indicatif::ProgressBar;

/// Wrap `reader` in the decoder for its detected format, or return it as-is.
///
/// The format is detected from the leading magic bytes, so the file name is
/// irrelevant and anything unrecognized is treated as plain text.
///
/// # Errors
///
/// Returns an error if a decoder fails to initialize, or if compressed data is
/// detected for a codec that is not compiled in.
fn decode(reader: Box<dyn io::Read>) -> io::Result<Box<dyn BufRead>> {
    #[cfg(any(feature = "zstd", feature = "bzip2", feature = "flate2"))]
    {
        let mut reader = BufReader::new(reader);
        let mime = infer::get(reader.fill_buf()?).map(|t| t.mime_type());

        #[cfg(feature = "zstd")]
        if mime == Some("application/zstd") {
            let mut decoder = zstd::stream::read::Decoder::new(reader)?;
            decoder.window_log_max(31)?;
            return Ok(Box::new(BufReader::new(decoder)));
        }

        #[cfg(feature = "bzip2")]
        if mime == Some("application/x-bzip2") {
            return Ok(Box::new(BufReader::new(bzip2::read::MultiBzDecoder::new(
                reader,
            ))));
        }

        #[cfg(feature = "flate2")]
        if mime == Some("application/gzip") {
            return Ok(Box::new(BufReader::new(flate2::read::MultiGzDecoder::new(
                reader,
            ))));
        }

        #[cfg(not(feature = "zstd"))]
        if mime == Some("application/zstd") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "zstd data detected, but support is not compiled in (enable the `zstd` feature)",
            ));
        }

        #[cfg(not(feature = "bzip2"))]
        if mime == Some("application/x-bzip2") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "bzip2 data detected, but support is not compiled in (enable the `bzip2` feature)",
            ));
        }

        #[cfg(not(feature = "flate2"))]
        if mime == Some("application/gzip") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "gzip data detected, but support is not compiled in (enable the `flate2` feature)",
            ));
        }

        Ok(Box::new(reader))
    }

    #[cfg(not(any(feature = "zstd", feature = "bzip2", feature = "flate2")))]
    {
        Ok(Box::new(BufReader::new(reader)))
    }
}

/// Open `input_path` as a buffered reader, decoding by content.
///
/// If `pb` is given, reads are reported to it.
///
/// # Examples
///
/// ```
/// use std::io::Read;
///
/// let path = std::env::temp_dir().join("fileutils-reader-example.jsonl");
/// std::fs::write(&path, b"{\"x\":1}\n").unwrap();
///
/// let mut text = String::new();
/// fileutils::read::setup_reader(&path, None)
///     .unwrap()
///     .read_to_string(&mut text)
///     .unwrap();
/// assert_eq!(text, "{\"x\":1}\n");
/// ```
///
/// # Errors
///
/// Returns an error if the file cannot be opened, a decoder fails to
/// initialize, or compressed data is detected for a codec that is not compiled
/// in.
#[cfg(feature = "progress")]
pub fn setup_reader<P: AsRef<Path>>(
    input_path: P,
    pb: Option<&ProgressBar>,
) -> io::Result<Box<dyn BufRead>> {
    let file = File::open(input_path.as_ref())?;
    let reader: Box<dyn io::Read> = match pb {
        Some(p) => Box::new(p.wrap_read(file)),
        None => Box::new(file),
    };
    decode(reader)
}

/// Open `input_path` as a buffered reader, decoding by content.
///
/// # Examples
///
/// ```
/// use std::io::Read;
///
/// let path = std::env::temp_dir().join("fileutils-reader-example.jsonl");
/// std::fs::write(&path, b"{\"x\":1}\n").unwrap();
///
/// let mut text = String::new();
/// fileutils::read::setup_reader(&path)
///     .unwrap()
///     .read_to_string(&mut text)
///     .unwrap();
/// assert_eq!(text, "{\"x\":1}\n");
/// ```
///
/// # Errors
///
/// Returns an error if the file cannot be opened, a decoder fails to
/// initialize, or compressed data is detected for a codec that is not compiled
/// in.
#[cfg(not(feature = "progress"))]
pub fn setup_reader<P: AsRef<Path>>(input_path: P) -> io::Result<Box<dyn BufRead>> {
    decode(Box::new(File::open(input_path.as_ref())?))
}
