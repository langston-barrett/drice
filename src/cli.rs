use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(name = "drice")]
#[command(about = "Dr. Ice diagnoses internal compiler errors (ICEs) in rustc")]
pub(crate) struct Cli {
    /// Increase verbosity
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
    )]
    pub(crate) verbose: u8,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub(crate) enum Command {
    /// Check if a program is a known ICE
    Check {
        /// Path to the Rust source file to check
        file: PathBuf,
    },
    /// Extract a MCVE from a GitHub issue
    Extract {
        /// Issue number from rust-lang/rust or path to local file
        issue_or_path: String,
    },
    /// Run nightly rustc to see if a program ICEs
    Rustc {
        /// Rust source file (.rs)
        path: PathBuf,
    },
    /// Deduplicate ICEs in a directory
    Dedup {
        /// Directory containing Rust source files to deduplicate
        directory: PathBuf,
    },
    /// Print file path, message, and query stack from a text or Rust file
    Glance {
        /// Path to a text file (stderr output) or Rust source file (.rs)
        file: PathBuf,
    },
}
