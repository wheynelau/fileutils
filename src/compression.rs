/// Settings for the zstd writer.
#[derive(Clone, Debug)]
pub struct CompressionConfig {
    /// Compression level (`1` to `22`).
    pub level: i32,
    /// Number of worker threads; `0` keeps it single-threaded.
    pub workers: u32,
}

impl CompressionConfig {
    /// Create a config with the given `level` and `workers`.
    ///
    /// ```
    /// use fileutils::CompressionConfig;
    ///
    /// let config = CompressionConfig::new(5, 4);
    /// assert_eq!(config.level, 5);
    /// ```
    #[must_use]
    pub const fn new(level: i32, workers: u32) -> Self {
        Self { level, workers }
    }

    pub(crate) fn build_encoder<W: std::io::Write>(
        &self,
        writer: W,
    ) -> Result<zstd::stream::write::Encoder<'static, W>, std::io::Error> {
        let mut encoder = zstd::stream::write::Encoder::new(writer, self.level)?;
        if self.workers >= 1 {
            encoder.multithread(self.workers)?;
        }
        Ok(encoder)
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            level: 3,
            workers: 4,
        }
    }
}
