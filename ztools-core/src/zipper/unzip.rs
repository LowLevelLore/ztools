use crate::ZToolsError;
use crate::zipper::{GZIP_MAGIC_HEADER, SEVEN_ZIP_MAGIC_HEADER};
use flate2::read::GzDecoder;
use sevenz_rust::decompress_file;
use std::fs::{self, File};
use std::io::{self, Read, copy};
use std::path::Path;
use tar::Archive;

#[allow(dead_code)]
fn is_tar_file(path: &str) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 265];
    file.read_exact(&mut buffer)?;
    Ok(&buffer[257..262] == b"ustar" || &buffer[257..265] == b"POSIX.1-")
}

pub fn untar_file(tar_path: &str, output_dir: &str) -> Result<(), ZToolsError> {
    let file = File::open(tar_path)?;
    let mut archive = Archive::new(file);
    archive.unpack(output_dir)?;
    Ok(())
}

pub fn unzip_file(file_path: String, out_base: String) -> Result<(), ZToolsError> {
    let input_path = Path::new(&file_path);
    if !input_path.exists() {
        return Err(ZToolsError::PathError(format!(
            "Input file '{}' does not exist",
            input_path.display()
        )));
    }
    let mut f = File::open(&file_path)?;
    let mut header = [0u8; 6];
    f.read_exact(&mut header)?;
    let cwd = std::env::current_dir()?;

    if header.starts_with(&GZIP_MAGIC_HEADER) {
        let p = input_path;
        let fname = p.file_name().and_then(|os| os.to_str()).unwrap_or("output");

        if fname.ends_with(".tar.gz") || fname.ends_with(".tgz") {
            let temp_tar_name = format!("{}.tar", out_base);
            let temp_tar_path = cwd.join(&temp_tar_name);
            let temp_tar_str = temp_tar_path
                .to_str()
                .ok_or_else(|| ZToolsError::InvalidInput("Invalid temp tar file path".into()))?;

            unzip_gzip_file(&file_path, temp_tar_str)?;
            let untar_dir = cwd.join(&out_base);
            fs::create_dir_all(&untar_dir)?;
            untar_file(temp_tar_str, untar_dir.to_str().unwrap_or("out"))?;
            fs::remove_file(temp_tar_path)?;
        } else {
            let inner_name = if fname.ends_with(".gz") {
                &fname[..fname.len() - 3]
            } else {
                fname
            };
            let inner_ext = Path::new(inner_name)
                .extension()
                .and_then(|os| os.to_str())
                .unwrap_or("");
            let output_file_name = if inner_ext.is_empty() {
                out_base.clone()
            } else {
                format!("{}.{}", out_base, inner_ext)
            };
            let output_path = cwd.join(&output_file_name);
            unzip_gzip_file(&file_path, output_path.to_str().unwrap_or("output"))?;
        }
    } else if header.starts_with(&SEVEN_ZIP_MAGIC_HEADER) {
        let out_dir = cwd.join(&out_base);
        fs::create_dir_all(&out_dir)?;
        unzip_seven_zip(&file_path, out_dir.to_str().unwrap_or("out"))?;
    } else {
        return Err(ZToolsError::CompressionError(format!(
            "Unknown or unsupported format for '{}'",
            file_path
        )));
    }

    Ok(())
}

fn unzip_gzip_file(input: &str, output_path: &str) -> Result<(), ZToolsError> {
    let gz = File::open(input)?;
    let mut decoder = GzDecoder::new(gz);
    let mut out_file = File::create(output_path)?;
    copy(&mut decoder, &mut out_file)?;
    Ok(())
}

fn unzip_seven_zip(input: &str, output_dir: &str) -> Result<(), ZToolsError> {
    decompress_file(Path::new(input), Path::new(output_dir))
        .map_err(|e| ZToolsError::SevenZipError(format!("{:?}", e)))?;
    Ok(())
}
