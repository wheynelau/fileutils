use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[cfg(feature = "progress")]
use indicatif::ProgressBar;

/// Wrap `reader` in the decoder matching `path`'s extension.
///
/// # Errors
///
/// Returns an error if a decoder fails to initialize or the extension is not
/// supported.
fn decode<P: AsRef<Path>>(path: P, reader: Box<dyn io::Read>) -> io::Result<Box<dyn BufRead>> {
    let path = path.as_ref();
    let ext_is = |ext: &str| {
        path.extension()
            .is_some_and(|e| e.eq_ignore_ascii_case(ext))
    };

    #[cfg(feature = "zstd")]
    if ext_is("zst") {
        let mut decoder = zstd::stream::read::Decoder::new(reader)?;
        decoder.window_log_max(31)?;
        return Ok(Box::new(BufReader::new(decoder)));
    }

    #[cfg(feature = "bzip2")]
    if ext_is("bz2") {
        return Ok(Box::new(BufReader::new(bzip2::read::BzDecoder::new(
            reader,
        ))));
    }

    if ext_is("jsonl") {
        return Ok(Box::new(BufReader::new(reader)));
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "unsupported file type",
    ))
}

/// # Errors
///
/// Returns an error if the file cannot be opened, a decoder fails to
/// initialize, or the file extension is unsupported.
#[cfg(feature = "progress")]
pub fn setup_reader<P: AsRef<Path>>(
    input_path: P,
    pb: Option<&ProgressBar>,
) -> io::Result<Box<dyn BufRead>> {
    let path = input_path.as_ref();
    let file = File::open(path)?;
    let reader: Box<dyn io::Read> = match pb {
        Some(p) => Box::new(p.wrap_read(file)),
        None => Box::new(file),
    };
    decode(path, reader)
}

/// # Errors
///
/// Returns an error if the file cannot be opened, a decoder fails to
/// initialize, or the file extension is unsupported.
#[cfg(not(feature = "progress"))]
pub fn setup_reader<P: AsRef<Path>>(input_path: P) -> io::Result<Box<dyn BufRead>> {
    let path = input_path.as_ref();
    let file = File::open(path)?;
    decode(path, Box::new(file))
}
