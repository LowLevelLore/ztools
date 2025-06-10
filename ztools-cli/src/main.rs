use clap::{Parser, Subcommand};
use colored::Colorize;
use confy;
use libunftp::Server;
use libunftp::options::Shutdown;
use std::env;
use std::net::TcpListener;
use std::path::Path;
use std::path::PathBuf;
use tokio::signal;
use unftp_sbe_fs::ServerExt;
use ztools_core::ZToolsError;
use ztools_core::repr::Representations;
use ztools_core::zipper::CompressionAlgorithm;

static _VERSION: &str = "0.0.1";

#[derive(serde::Serialize, serde::Deserialize)]
struct ZtoolsConfig {
    scripts_directory: String,
}

impl Default for ZtoolsConfig {
    fn default() -> ZtoolsConfig {
        ZtoolsConfig {
            #[allow(deprecated)]
            scripts_directory: env::home_dir()
                .expect("Cannot get home dir, default cannot be constructed")
                .join(".config")
                .join("ztools")
                .join("scripts")
                .to_string_lossy()
                .to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "ztools", version = _VERSION, about = "A zip/unzip tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Zip {
        file: String,
        #[arg(long, conflicts_with = "use_7z")]
        gzip: bool,
        #[arg(long = "7z", conflicts_with = "gzip")]
        use_7z: bool,
        #[arg(short = 'f')]
        outfile: Option<String>,
    },

    Unzip {
        file: String,
        #[arg(short = 'f')]
        outfile: Option<String>,
    },

    Run {
        name: String,
        #[arg(last = true, trailing_var_arg = true)]
        args: Vec<String>,
    },

    Convert {
        value: String,
        #[arg(short, long, default_value = "d")]
        to: Representations,
    },

    Ftp {
        dir: String,
        #[arg(short, long, default_value = "6969")]
        port: String,
    },
}

fn load_config() -> ZtoolsConfig {
    match confy::load("ztools", "config") {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{}", format!("Error loading config: {}", err).yellow());
            let cfg = ZtoolsConfig::default();
            if let Err(e) = confy::store("ztools", "config", &cfg) {
                eprintln!("{}", format!("Error saving default config: {}", e).yellow());
            }
            cfg
        }
    }
}

fn handle_error(err: ZToolsError) {
    match err {
        ZToolsError::Io(e) => eprintln!("{}", format!("I/O error: {}", e).red()),
        ZToolsError::CompressionError(msg) => {
            eprintln!("{}", format!("Compression error: {}", msg).red())
        }
        ZToolsError::InvalidInput(msg) => eprintln!("{}", format!("Invalid input: {}", msg).red()),
        ZToolsError::PathError(msg) => eprintln!("{}", format!("Path error: {}", msg).red()),
        ZToolsError::SevenZipError(msg) => eprintln!("{}", format!("7z error: {}", msg).red()),
        ZToolsError::GzipError(msg) => eprintln!("{}", format!("Gzip error: {}", msg).red()),
        ZToolsError::UntarError(msg) => eprintln!("{}", format!("Untar error: {}", msg).red()),
        ZToolsError::SpawnError(msg) => eprintln!("{}", format!("Spawn error: {}", msg).red()),
        ZToolsError::PermissionError(msg) => {
            eprintln!("{}", format!("Permission error: {}", msg).red())
        }
    }
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn start_ftp_server(dir: String, port: u16) {
    let path = PathBuf::from(&dir);

    if !path.is_dir() {
        eprintln!("{} is not a valid directory", dir);
        return;
    }
    if !port_is_available(port) {
        eprintln!("Port {port} is not available");
        return;
    }

    let shutdown = async {
        signal::ctrl_c().await.expect("failed to listen for ctrl-c");
        Shutdown::new()
    };

    let server = Server::with_fs(path)
        .greeting("Welcome ztools File Server by xZist")
        .passive_ports(50000..=65535)
        .shutdown_indicator(shutdown)
        .build()
        .unwrap();

    // directly await the listen future
    if let Err(e) = server.listen(format!("0.0.0.0:{}", port)).await {
        eprintln!("FTP server terminated with error: {e}");
    } else {
        println!("FTP server exited cleanly");
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = load_config();

    match cli.command {
        Commands::Zip {
            file,
            gzip: _,
            use_7z,
            outfile,
        } => {
            let alg = if use_7z {
                CompressionAlgorithm::SevenZip
            } else {
                CompressionAlgorithm::Gzip
            };

            let base_name = if let Some(ref f) = outfile {
                f.clone()
            } else {
                let p = Path::new(&file);
                if p.is_dir() {
                    p.file_name()
                        .and_then(|os| os.to_str())
                        .unwrap_or("output")
                        .to_string()
                } else {
                    p.file_stem()
                        .and_then(|os| os.to_str())
                        .unwrap_or("output")
                        .to_string()
                }
            };

            if let Err(err) = ztools_core::zipper::zip::zip_file(file, alg, base_name) {
                handle_error(err);
            }
        }
        Commands::Unzip { file, outfile } => {
            let base_name = if let Some(ref f) = outfile {
                f.clone()
            } else {
                let p = Path::new(&file);
                let filename = p.file_name().and_then(|os| os.to_str()).unwrap_or("");
                if filename.ends_with(".tar.gz") {
                    filename.trim_end_matches(".tar.gz").to_string()
                } else if filename.ends_with(".tgz") {
                    filename.trim_end_matches(".tgz").to_string()
                } else if filename.ends_with(".gz") {
                    filename.trim_end_matches(".gz").to_string()
                } else if filename.ends_with(".7z") {
                    filename.trim_end_matches(".7z").to_string()
                } else {
                    p.file_stem()
                        .and_then(|os| os.to_str())
                        .unwrap_or("output")
                        .to_string()
                }
            };

            if let Err(err) = ztools_core::zipper::unzip::unzip_file(file, base_name) {
                handle_error(err);
            }
        }
        Commands::Run { name, args } => {
            if let Err(err) = ztools_core::run_script(&name, &config.scripts_directory, &args) {
                handle_error(err);
            }
        }
        Commands::Convert { value, to } => match ztools_core::convert_repr(&value, to.clone()) {
            Ok(result) => println!("{}", result),
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Ftp { dir, port } => {
            start_ftp_server(dir, port.parse().expect("Invalid Port Number")).await;
        }
    }

    Ok(())
}
