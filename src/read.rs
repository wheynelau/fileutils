use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[cfg(feature = "progress")]
use indicatif::ProgressBar;

/// # Errors
///
/// Returns an error if the file cannot be opened, the zstd decoder fails to
/// initialize, or the file extension is not `.zst` or `.jsonl`.
#[cfg(feature = "progress")]
pub fn setup_reader<P: AsRef<Path>>(
    input_path: P,
    pb: Option<&ProgressBar>,
) -> io::Result<Box<dyn BufRead>> {
    let path = input_path.as_ref();
    let file = File::open(path)?;

    #[cfg(feature = "compression")]
    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("zst"))
    {
        let reader: Box<dyn io::Read> = match pb {
            Some(p) => Box::new(p.wrap_read(file)),
            None => Box::new(file),
        };
        let mut decoder = zstd::stream::read::Decoder::new(reader)?;
        decoder.window_log_max(31)?;
        return Ok(Box::new(BufReader::new(decoder)));
    }

    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("jsonl"))
    {
        let reader: Box<dyn io::Read> = match pb {
            Some(p) => Box::new(p.wrap_read(file)),
            None => Box::new(file),
        };
        Ok(Box::new(BufReader::new(reader)))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unsupported file type",
        ))
    }
}

/// # Errors
///
/// Returns an error if the file cannot be opened, the zstd decoder fails to
/// initialize, or the file extension is not `.zst` or `.jsonl`.
#[cfg(not(feature = "progress"))]
pub fn setup_reader<P: AsRef<Path>>(input_path: P) -> io::Result<Box<dyn BufRead>> {
    let path = input_path.as_ref();
    let file = File::open(path)?;

    #[cfg(feature = "compression")]
    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("zst"))
    {
        let mut decoder = zstd::stream::read::Decoder::new(file)?;
        decoder.window_log_max(31)?;
        return Ok(Box::new(BufReader::new(decoder)));
    }

    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("jsonl"))
    {
        Ok(Box::new(BufReader::new(file)))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unsupported file type",
        ))
    }
}
