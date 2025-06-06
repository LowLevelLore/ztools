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
    if &buffer[257..262] == b"ustar" || &buffer[257..265] == b"POSIX.1-" {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Extracts a `.tar` archive into the given directory.
pub fn untar_file(tar_path: &str, output_dir: &str) -> io::Result<()> {
    let file = File::open(tar_path)?;
    let mut archive = Archive::new(file);
    archive.unpack(output_dir)?;
    Ok(())
}

/// Main entry for `unzip` subcommand:
///
/// - `file_path`: the compressed file (e.g. `foo.txt.gz` or `dir.tar.gz` or `archive.7z`)
/// - `out_base`: the base name (no extension) for the output (overrides default if provided)
///
/// Behavior:
/// 1. Read magic header to decide gzip vs 7z.
/// 2. If gzip:
///    • If it’s a “.tar.gz” or “.tgz” by magic, first create `out_base.tar` in cwd, then untar into `out_base/`, then delete the temporary `.tar`.
///    • Else (just `.gz` of a single file): write `out_base.<orig_ext>` in cwd.
/// 3. If 7z: decompress directly into a directory named `out_base/` in cwd.
pub fn unzip_file(file_path: String, out_base: String) {
    // Open the file and read its first few bytes
    let mut f = File::open(&file_path).expect("Failed to open file");
    let mut header = [0u8; 6];
    f.read_exact(&mut header)
        .expect("Failed to read file header");

    // Decide where the user wants output.
    // We always work in the current working directory.
    let cwd = std::env::current_dir().expect("Failed to read cwd");

    if header.starts_with(&GZIP_MAGIC_HEADER) {
        // GZIP detected. Determine if this is a tar.gz or just a file.gz
        let p = Path::new(&file_path);
        let fname = p.file_name().and_then(|os| os.to_str()).unwrap_or("output");

        // If the filename ends with .tar.gz or .tgz, handle it as a tarball.
        if fname.ends_with(".tar.gz") || fname.ends_with(".tgz") {
            // 1) Create a temporary `.tar` in cwd named `<out_base>.tar`
            let temp_tar_name = format!("{}.tar", out_base);
            let temp_tar_path = cwd.join(&temp_tar_name);
            let temp_tar_str = temp_tar_path.to_str().unwrap();

            // Decompress the gzip stream into that `*.tar`
            unzip_gzip_file(&file_path, temp_tar_str)
                .unwrap_or_else(|e| panic!("Failed to decompress gzip file: {}", e));

            // 2) Untar it into a directory named `out_base/`
            let untar_dir = cwd.join(&out_base);
            fs::create_dir_all(&untar_dir).expect("Failed to create output directory for tar");

            untar_file(temp_tar_str, untar_dir.to_str().unwrap())
                .unwrap_or_else(|e| panic!("Failed to untar file: {}", e));

            // 3) Remove the temporary `*.tar`
            fs::remove_file(temp_tar_path).expect("Failed to remove temporary .tar");
        } else {
            // It’s a single-file gzip (e.g. foo.txt.gz). We need to write out `out_base.<orig_ext>`
            // Derive the original extension by stripping “.gz” from the filename
            let inner_name = if fname.ends_with(".gz") {
                &fname[..fname.len() - 3]
            } else {
                fname
            };
            // “inner_name” is now something like “foo.txt”
            // Grab the extension of “foo.txt”
            let inner_ext = Path::new(inner_name)
                .extension()
                .and_then(|os| os.to_str())
                .unwrap_or("");
            // Build “out_base.<inner_ext>”
            let output_file_name = if inner_ext.is_empty() {
                out_base.clone()
            } else {
                format!("{}.{}", out_base, inner_ext)
            };
            let output_path = cwd.join(&output_file_name);

            unzip_gzip_file(&file_path, output_path.to_str().unwrap())
                .unwrap_or_else(|e| panic!("Failed to decompress gzip file: {}", e));
        }
    } else if header.starts_with(&SEVEN_ZIP_MAGIC_HEADER) {
        // 7z detected. We decompress directly into a folder named `out_base/`
        let out_dir = cwd.join(&out_base);
        fs::create_dir_all(&out_dir).expect("Failed to create output directory for 7z");

        unzip_seven_zip(&file_path, out_dir.to_str().unwrap())
            .unwrap_or_else(|e| panic!("Failed to decompress 7z archive: {}", e));
    } else {
        panic!(
            "Unknown/unsupported format for '{}'. Expected gzip or 7z.",
            file_path
        );
    }
}

/// Decompress a `.gz` into the given output path (which may be a `.tar` or a final file)
fn unzip_gzip_file(input: &str, output_path: &str) -> io::Result<()> {
    let gz = File::open(input)?;
    let mut decoder = GzDecoder::new(gz);
    let mut out_file = File::create(output_path)?;
    copy(&mut decoder, &mut out_file)?;
    Ok(())
}

/// Decompress a `.7z` archive into `output_dir/`
fn unzip_seven_zip(input: &str, output_dir: &str) -> io::Result<()> {
    let input_path = Path::new(input);
    let output_path = Path::new(output_dir);
    decompress_file(input_path, output_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))
}
