use clap::{Parser, Subcommand};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{self, Command, ExitStatus},
};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::ExitStatusExt;

use vfs::generate_metadata_json;

#[derive(Parser)]
#[command(name = "xtask", about = "Development automation for this workspace")]
struct Cli {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    /// Run cargo fmt across the workspace
    Fmt,
    /// Run clippy with wasm warnings as errors
    Lint,
    /// Start Trunk dev server
    Serve,
    /// Build release assets with Trunk
    Build,
    /// Generate JSON metadata describing the data/ directory
    MetadataGenerate {
        /// Path to the data directory (defaults to ./app/data)
        #[arg(long, default_value = "../data")]
        root: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        CommandKind::Fmt => run(Command::new("cargo").arg("fmt")),
        CommandKind::Lint => run(Command::new("cargo")
            .arg("clippy")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--")
            .arg("-D")
            .arg("warnings")),
        CommandKind::Serve => run(Command::new("trunk").arg("serve")),
        CommandKind::Build => run(Command::new("trunk").arg("build").arg("--release")),
        CommandKind::MetadataGenerate { root } => generate_metadata(&root),
    };

    match result {
        Ok(status) if status.success() => {}
        Ok(status) => process::exit(status.code().unwrap_or(1)),
        Err(err) => {
            eprintln!("Task failed: {err}");
            process::exit(1);
        }
    }
}

fn run(cmd: &mut Command) -> std::io::Result<ExitStatus> {
    eprintln!("Running: {:?}", cmd);
    cmd.status()
}

fn generate_metadata(root: &Path) -> std::io::Result<ExitStatus> {
    let json = match generate_metadata_json(root) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed generating metadata: {err}");
            return Ok(status_from_code(1));
        }
    };

    let out_path = Path::new("src").join("vfs.json");
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out_path, json)?;
    println!("Metadata written to {}", out_path.display());

    Ok(status_from_code(0))
}

fn status_from_code(code: i32) -> ExitStatus {
    #[cfg(unix)]
    {
        ExitStatus::from_raw(code)
    }
    #[cfg(windows)]
    {
        ExitStatus::from_raw(code as u32)
    }
}
