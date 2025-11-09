use std::{fs, path::Path, process};

pub(crate) fn go(path: &Path) -> anyhow::Result<String> {
    let temp_file = tempfile::NamedTempFile::new()?;
    let mut cmd = process::Command::new("rustc");
    let mut cmd = cmd
        .arg("+nightly")
        .arg("--crate-name=drice")
        .arg("--crate-type=lib")
        .arg("--emit=mir")
        .arg("-o")
        .arg(temp_file.path());
    let content = fs::read_to_string(path)?;
    if content.contains("// @compile-flags: --edition=2024") {
        cmd = cmd.arg("--edition=2024");
    }
    let output = cmd
        .arg(path)
        // .env("RUSTC_ICE", "0")
        .env("RUST_BACKTRACE", "1")
        .stderr(process::Stdio::piped())
        .output()?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    Ok(stderr.to_string())
}
