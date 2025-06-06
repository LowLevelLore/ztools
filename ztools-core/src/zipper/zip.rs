use flate2::Compression;
use flate2::write::GzEncoder;
use sevenz_rust::compress_to_path;
use std::fs::{self, File};
use std::io::{self, copy};
use std::path::Path;
use tar::Builder;

use crate::zipper::CompressionAlgorithm;

/// `zip_file(...)` drives compression logic
///
/// - `file`: input file or directory path
/// - `algorithm`: Gzip or SevenZip
/// - `base_out`: the “base” name (no extension) for the output in the current directory
///     • For a *file* (e.g. `foo.txt`): we will create either `base_out.txt.gz` (gzip)
///       or `base_out.7z` (7z).
///     • For a *directory* (`my_dir/`): we will create `base_out.tar.gz` if gzip,
///       or `base_out.7z` if 7z.
pub fn zip_file(file: String, algorithm: CompressionAlgorithm, base_out: String) {
    let input_path = Path::new(&file);
    match algorithm {
        CompressionAlgorithm::Gzip => {
            // If it’s a directory, produce `<base_out>.tar.gz`
            // If it’s a file, produce `<base_out>.<orig_ext>.gz`
            let final_out = if input_path.is_dir() {
                format!("{}.tar.gz", base_out)
            } else {
                let ext = input_path
                    .extension()
                    .and_then(|os| os.to_str())
                    .unwrap_or("");
                format!("{}.{}.gz", base_out, ext)
            };
            gzip_compression(&file, &final_out)
                .unwrap_or_else(|err| panic!("gzip_compression failed: {}", err));
        }

        CompressionAlgorithm::SevenZip => {
            // Always produce `base_out.7z`
            let final_out = format!("{}.7z", base_out);
            // Make sure parent dir exists (if user put subdirectories in base_out)
            let out_path = Path::new(&final_out);
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            seven_zip_compression(&file, &final_out)
                .unwrap_or_else(|err| panic!("seven_zip_compression failed: {}", err));
        }
    }
}

fn gzip_compression(file: &str, outfile: &str) -> io::Result<()> {
    let input_path = Path::new(file);

    if input_path.is_dir() {
        // Create a tarball stream and then gzip it
        let tar_gz_file = File::create(outfile)?;
        let encoder = GzEncoder::new(tar_gz_file, Compression::default());
        let mut tar_builder = Builder::new(encoder);

        // Append the entire directory under its own directory name
        let dir_name = input_path
            .file_name()
            .and_then(|os| os.to_str())
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "Cannot determine directory name")
            })?;
        tar_builder.append_dir_all(dir_name, input_path)?;
        let encoder = tar_builder.into_inner()?; // GzEncoder
        encoder.finish()?; // finalize .gz
        Ok(())
    } else {
        // It’s a single file → gzip directly to `<outfile>`
        let input_file = File::open(input_path)?;
        let mut reader = std::io::BufReader::new(input_file);
        let out_f = File::create(outfile)?;
        let mut encoder = GzEncoder::new(out_f, Compression::default());
        copy(&mut reader, &mut encoder)?;
        encoder.finish()?;
        Ok(())
    }
}

fn seven_zip_compression(file: &str, outfile: &str) -> io::Result<()> {
    let input_path = Path::new(file);
    let outfile_path = Path::new(outfile);
    compress_to_path(input_path, outfile_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
    Ok(())
}
