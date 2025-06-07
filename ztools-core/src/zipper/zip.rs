use crate::ZToolsError;
use crate::zipper::CompressionAlgorithm;
use flate2::Compression;
use flate2::write::GzEncoder;
use sevenz_rust::compress_to_path;
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;
use tar::Builder;

pub fn zip_file(
    file: String,
    algorithm: CompressionAlgorithm,
    base_out: String,
) -> Result<(), ZToolsError> {
    let file_path = Path::new(&file);
    if !file_path.exists() {
        return Err(ZToolsError::PathError(format!(
            "Input file '{}' does not exist",
            file_path.display()
        )));
    }
    match algorithm {
        CompressionAlgorithm::Gzip => {
            let final_out = if file_path.is_dir() {
                format!("{}.tar.gz", base_out)
            } else {
                let ext = file_path
                    .extension()
                    .and_then(|os| os.to_str())
                    .unwrap_or("");
                format!("{}.{}.gz", base_out, ext)
            };
            gzip_compression(&file, &final_out)?;
        }
        CompressionAlgorithm::SevenZip => {
            let final_out = format!("{}.7z", base_out);
            if let Some(parent) = Path::new(&final_out).parent() {
                fs::create_dir_all(parent)?;
            }
            seven_zip_compression(&file, &final_out)?;
        }
    }
    Ok(())
}

fn gzip_compression(file: &str, outfile: &str) -> Result<(), ZToolsError> {
    let input_path = Path::new(file);
    if input_path.is_dir() {
        let tar_gz_file = File::create(outfile)?;
        let encoder = GzEncoder::new(tar_gz_file, Compression::default());
        let mut tar_builder = Builder::new(encoder);
        let dir_name = input_path
            .file_name()
            .and_then(|os| os.to_str())
            .ok_or_else(|| ZToolsError::GzipError("Cannot determine directory name".into()))?;
        tar_builder.append_dir_all(dir_name, input_path)?;
        let encoder = tar_builder.into_inner()?;
        encoder.finish()?;
    } else {
        let input_file = File::open(input_path)?;
        let mut reader = std::io::BufReader::new(input_file);
        let out_f = File::create(outfile)?;
        let mut encoder = GzEncoder::new(out_f, Compression::default());
        copy(&mut reader, &mut encoder)?;
        encoder.finish()?;
    }
    Ok(())
}

fn seven_zip_compression(file: &str, outfile: &str) -> Result<(), ZToolsError> {
    compress_to_path(Path::new(file), Path::new(outfile))
        .map_err(|e| ZToolsError::SevenZipError(format!("{:?}", e)))?;
    Ok(())
}
