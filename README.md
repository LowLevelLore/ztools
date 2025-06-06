# ztools-cli

**ztools-cli** is a simple command‐line utility written in Rust for compressing and decompressing files and directories. It supports:

- **Gzip** (automatically handles single files vs. directories → `.gz` vs. `.tar.gz`)
- **7-Zip** (creates/reports `.7z` archives)
- Intelligent naming and automatic handling of intermediate tarball steps
- Optional `-f` flag to override output base names

---

## Table of Contents

1. [Features](#features)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Usage](#usage)
   - [Zip (Compress)](#zip-compress)
   - [Unzip (Decompress)](#unzip-decompress)
5. [Examples](#examples)
6. [Project Layout](#project-layout)
7. [License](#license)

---

## Features

- **Automatically choose Gzip or 7-Zip**
  - By default, `zip` uses Gzip unless `--7z` is explicitly provided.
  - `unzip` inspects the file’s magic header to decide Gzip vs. 7-Zip automatically.
- **Smart handling of directories vs. files (Gzip)**
  - When compressing a directory with Gzip, a `.tar.gz` is created.
  - When compressing a single file with Gzip, a `.gz` is created.
- **Custom output base name**
  - Pass `-f <base>` to override the output filename or folder:
    - For `sample.txt`: `-f foo` → `foo.txt.gz`
    - For `mydir/`: `-f bar` → `bar.tar.gz` (Gzip) or `bar.7z` (7-Zip)
- **Reversible and transparent decompression**
  - Gzip single‐file archives (`.gz`) produce the original filename.
  - Gzip directory archives (`.tar.gz` or `.tgz`) produce a folder with the same base by default (or custom via `-f`).
  - 7-Zip archives produce a directory named after the archive by default (or custom via `-f`).
- **Zero external dependencies at runtime**
  - Everything is bundled as a standalone Rust binary (only requires `glibc` or equivalent on Unix).

---

## Prerequisites

- **Rust & Cargo**
  - You can install Rust via [rustup](https://rustup.rs/).
  - Verify installation:
    ```sh
    rustc --version
    cargo --version
    ```
- **(Optional) 7-Zip CLI Tools**
  - The crate `sevenz-rust` uses a pure-Rust implementation of 7-Zip. No external 7z binary is required.

---

## Installation

1. **Clone the repository**
   ```sh
        git clone https://github.com/yourusername/ztools.git
        cd ztools
   ```
2. **Run install.sh**
   ```sh
        chmod +x ./install.sh
        ./install.sh
   ```

## Usage

1. **To zip a file**

   ```sh
   ztools zip <file-or-dir> [--gzip] [--7z] [-f <base>]
   ```
   - ### `<file-or-dir>`

     Path to a file or directory to compress.
   - ### `--gzip`

     Force Gzip compression (also the default if neither `--gzip` nor `--7z` is provided).
   - ### `--7z`

     Use 7-Zip compression instead of Gzip.
   - ### `-f <base>`

     Override the output base name (no extension).


     - If the input is a **single file**, the output will be `<base>.<orig_ext>.gz` for Gzip or `<base>.7z` for 7-Zip.
     - If the input is a **directory**, the output will be `<base>.tar.gz` for Gzip or `<base>.7z` for 7-Zip.
2. **To unzip a file**

   ```sh
   ztools unzip <archive-file> [-f <base>]
   ```
   - ### `<archive-file>`

     Path to the archive file to decompress. Supported formats: `.gz`, `.tar.gz`, `.tgz`, `.7z`.
   - ### `-f <base>`

     Override the output base name (no extension or folder).


     - For a single-file Gzip archive (`file.txt.gz`), `-f output` will create `output.txt`.
     - For a directory archive (`dir.tar.gz`, `.tgz`, or `.7z`), `-f output` will extract into a folder named `output/`.
