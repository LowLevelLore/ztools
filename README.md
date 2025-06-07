# ztools-cli

**ztools-cli** is a simple command‐line utility written in Rust for compressing and decompressing files and directories. It supports:

* **Gzip** (automatically handles single files vs. directories → `.gz` vs. `.tar.gz`)
* **7-Zip** (creates/reports `.7z` archives)
* Intelligent naming and automatic handling of intermediate tarball steps
* Optional `-f` flag to override output base names
* **Script execution** (run custom scripts with arbitrary flags and arguments)

---

## Table of Contents

- [ztools-cli](#ztools-cli)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Zip (Compress)](#zip-compress)
    - [Unzip (Decompress)](#unzip-decompress)
    - [Run (Execute Scripts)](#run-execute-scripts)
  - [Examples](#examples)
  - [License](#license)

---

## Features

* **Automatically choose Gzip or 7-Zip**

  * By default, `zip` uses Gzip unless `--7z` is explicitly provided.
  * `unzip` inspects the file’s magic header to decide Gzip vs. 7-Zip automatically.
* **Smart handling of directories vs. files (Gzip)**

  * When compressing a directory with Gzip, a `.tar.gz` is created.
  * When compressing a single file with Gzip, a `.gz` is created.
* **Custom output base name**

  * Pass `-f <base>` to override the output filename or folder:

    * For **single files**: output `<base>.<orig_ext>.gz` or `<base>.7z`
    * For **directories**: output `<base>.tar.gz` or `<base>.7z`
* **Reversible and transparent decompression**

  * Gzip single‐file archives (`.gz`) produce the original filename.
  * Gzip directory archives (`.tar.gz`, `.tgz`) and 7z archives extract into named folders.
* **Script execution**

  * `run` subcommand launches scripts from a configured directory, forwarding any flags or arguments.
  * Customizable script directory in `~/.config/ztools` or default `/etc/ztools/scripts/`.
* **Zero external dependencies at runtime**

  * Everything is bundled as a standalone Rust binary (only requires `glibc` or equivalent on Unix).

---

## Prerequisites

* **Rust & Cargo**

  * Install via [rustup](https://rustup.rs/).
  * Verify:

    ```sh
    rustc --version
    cargo --version
    ```
* **(Optional) 7-Zip**

  * Pure-Rust implementation used; no external `7z` binary required.

---

## Installation

```sh
git clone https://github.com/yourusername/ztools.git
cd ztools
chmod +x ./install.sh
./install.sh
```

---

## Usage

### Zip (Compress)

```sh
ztools zip <file-or-dir> [--gzip] [--7z] [-f <base>]
```

  - <file-or-dir>
     Path to a file or directory to compress.
     
  - --gzip
     Force Gzip compression (also the default if neither --gzip nor --7z is provided).

  - --7z
     Use 7-Zip compression instead of Gzip.

  - -f <base>
    Override the output base name (no extension).
    - If the input is a **single file**, the output will be <base>.<orig_ext>.gz for Gzip or <base>.7z for 7-Zip.
    - If the input is a **directory**, the output will be <base>.tar.gz for Gzip or <base>.7z for 7-Zip.

### Unzip (Decompress)

```sh
ztools unzip <archive-file> [-f <base>]
```

  - <archive-file>
     Path to the archive file to decompress. Supported formats: .gz, .tar.gz, .tgz, .7z.

  - -f <base>
    Override the output base name (no extension or folder).
    - For a single-file Gzip archive (file.txt.gz), -f output will create output.txt.
    - For a directory archive (dir.tar.gz, .tgz, or .7z), -f output will extract into a folder named output/.

### Run (Execute Scripts)

```sh
ztools run <script-name> -- [--flags] [--key value]
```

* **`<script-name>`**: the script file in the configured scripts directory.
* All following flags, options, and positional arguments are forwarded to the script.
* Example:

  ```sh
  ztools run test_script.sh -- --flag --name Alice foo bar
  ```

---

## Examples

```sh
# Compress a directory with gzip (default)
ztools zip mydir

# Compress a file with 7z
ztools zip sample.txt --7z -f archives/sample

# Extract a tar.gz with custom output folder
ztools unzip backup.tar.gz -f backup_folder

# Run a test script with flags and args
ztools run test_script.sh -- --flag --name Alice foo bar
```

---

## License

MIT © LowLevelLore (Mihir Patel)
