#[derive(Clone, Debug)]
pub struct CompressionConfig {
    pub level: i32,
    pub workers: u32,
}

impl CompressionConfig {
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
