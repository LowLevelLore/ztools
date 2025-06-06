// src/lib.rs (or wherever your library entry point is)

pub mod unzip;
pub mod zip;

use clap::ValueEnum;

static GZIP_MAGIC_HEADER: [u8; 2] = [0x1f, 0x8b];
static SEVEN_ZIP_MAGIC_HEADER: [u8; 6] = [0x37, 0x7a, 0xbc, 0xaf, 0x27, 0x1c];

#[derive(Debug, Clone, ValueEnum)]
pub enum CompressionAlgorithm {
    #[clap(name = "gzip")]
    Gzip,
    #[clap(name = "7zip")]
    SevenZip,
}
