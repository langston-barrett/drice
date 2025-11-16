use clap::Parser as _;
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

mod check;
mod cli;
mod dedup;
mod extract;
mod ice;
mod rustc;

fn verbosity_to_log_level(verbosity: u8) -> Level {
    match verbosity {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    }
}

#[inline]
fn init_tracing(cli: &cli::Cli) {
    let verbose = verbosity_to_log_level(cli.verbose);
    let builder = tracing_subscriber::fmt::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_target(false)
        .with_max_level(verbose);
    if let Level::INFO | Level::WARN | Level::ERROR = verbose {
        let builder = builder.without_time();
        builder.init();
    } else {
        builder.init();
    }
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    init_tracing(&cli);

    match cli.command {
        cli::Command::Check { file } => {
            check::check(check::CheckConfig { file })?;
        }
        cli::Command::Extract { issue_or_path } => {
            extract::extract(extract::ExtractConfig { issue_or_path })?;
        }
        cli::Command::Rustc { path } => {
            eprintln!("{}", rustc::go(path.as_path())?);
        }
        cli::Command::Dedup { directory } => {
            dedup::dedup(dedup::DedupConfig { directory })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ice::ICES;

    #[test]
    fn test_every_rs_file_has_entry_in_ices() {
        use std::fs;
        use std::path::Path;

        let ice_dir = Path::new("ice");
        let mut missing_entries = Vec::new();

        for entry in fs::read_dir(ice_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let expected_path = format!("ice/{file_name}");
                let found = ICES.iter().any(|(rs_path, _)| *rs_path == expected_path);
                if !found {
                    missing_entries.push(expected_path);
                }
            }
        }

        assert!(
            missing_entries.is_empty(),
            "The following .rs files in ice/ do not have entries in ICES: {missing_entries:?}",
        );
    }
}
