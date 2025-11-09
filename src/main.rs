use clap::Parser as _;
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

mod check;
mod cli;
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
    }

    Ok(())
}
