use clap::{Parser, Subcommand};
use std::path::Path;

use ztools_core::zipper::CompressionAlgorithm;

#[derive(Parser, Debug)]
#[command(name = "ztools", version, about = "A zip/unzip tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compress a file or directory (gzip by default)
    Zip {
        /// File or directory to compress
        file: String,

        /// Force gzip even if --7z is also present (default if neither is present)
        #[arg(long, conflicts_with = "use_7z")]
        gzip: bool,

        /// Use 7z compression
        #[arg(long = "7z", conflicts_with = "gzip")]
        use_7z: bool,

        /// Override the output base name (no extension).
        /// For a file: <base>.<orig_ext>.gz
        /// For a directory: <base>.tar.gz
        #[arg(short = 'f')]
        outfile: Option<String>,
    },

    /// Decompress a .gz/.tar.gz/.tgz or .7z file
    Unzip {
        /// Compressed file to decompress
        file: String,

        /// Override the output base name:
        /// - For *.txt.gz → <base>.txt
        /// - For *.tar.gz or *.tgz → <base>/ directory
        /// - For *.7z → <base>/ directory
        #[arg(short = 'f')]
        outfile: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    #[allow(unused_variables)]
    match cli.command {
        Commands::Zip {
            file,
            gzip,
            use_7z,
            outfile,
        } => {
            // Decide on algorithm: if --7z present, use 7z; else gzip.
            let alg = if use_7z {
                CompressionAlgorithm::SevenZip
            } else {
                // if explicitly --gzip or neither, default to Gzip
                CompressionAlgorithm::Gzip
            };

            // Determine base name:
            // If -f was provided, use that; otherwise derive from `file`.
            let base_name = if let Some(ref f) = outfile {
                f.clone()
            } else {
                // If `file` is "/path/to/foo.txt" or "/path/to/dir/", strip directories
                let p = Path::new(&file);
                if p.is_dir() {
                    p.file_name()
                        .and_then(|os| os.to_str())
                        .unwrap_or("output")
                        .to_string()
                } else {
                    // It's a file. We want its full stem (without extension)
                    p.file_stem()
                        .and_then(|os| os.to_str())
                        .unwrap_or("output")
                        .to_string()
                }
            };

            ztools_core::zipper::zip::zip_file(file, alg, base_name);
        }

        Commands::Unzip { file, outfile } => {
            // Determine base name for output: if -f was provided, use it; else derive from input.
            let base_name = if let Some(ref f) = outfile {
                f.clone()
            } else {
                // strip extensions .tar.gz/.tgz/.gz/.7z
                let p = Path::new(&file);
                let filename = p.file_name().and_then(|os| os.to_str()).unwrap_or("");
                // remove common suffixes in order:
                if filename.ends_with(".tar.gz") {
                    filename.trim_end_matches(".tar.gz").to_string()
                } else if filename.ends_with(".tgz") {
                    filename.trim_end_matches(".tgz").to_string()
                } else if filename.ends_with(".gz") {
                    filename.trim_end_matches(".gz").to_string()
                } else if filename.ends_with(".7z") {
                    filename.trim_end_matches(".7z").to_string()
                } else {
                    // fallback: just drop extension
                    p.file_stem()
                        .and_then(|os| os.to_str())
                        .unwrap_or("output")
                        .to_string()
                }
            };

            ztools_core::zipper::unzip::unzip_file(file, base_name);
        }
    }
}
