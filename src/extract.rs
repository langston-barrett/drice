use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;

use crate::{check, rustc};

pub(crate) struct ExtractConfig {
    pub issue_or_path: String,
}

fn extract_rust_code_block(markdown: &str) -> Option<String> {
    let mut in_code_block = false;
    let mut code_lines = Vec::new();

    for line in markdown.lines() {
        if line.trim().starts_with("```") {
            if in_code_block {
                return Some(code_lines.join("\n"));
            }
            let lang = line.trim().trim_start_matches("```").trim();
            if lang == "rust" || lang == "Rust" {
                in_code_block = true;
            }
        } else if in_code_block {
            code_lines.push(line);
        }
    }
    None
}

pub(crate) fn run_rustc_on_temp(code: &str) -> anyhow::Result<String> {
    let mut temp_file =
        tempfile::NamedTempFile::with_suffix(".rs").context("failed to create temporary file")?;
    temp_file
        .write_all(code.as_bytes())
        .context("failed to write code to temporary file")?;
    temp_file
        .flush()
        .context("failed to flush temporary file")?;
    rustc::go(temp_file.path()).with_context(|| {
        format!(
            "failed to run rustc on temporary file: {}",
            temp_file.path().display()
        )
    })
}

pub(crate) fn extract(config: ExtractConfig) -> anyhow::Result<()> {
    let base = config
        .issue_or_path
        .strip_suffix(".rs")
        .unwrap_or(&config.issue_or_path);
    let rs_path = PathBuf::from(format!("ice/{base}.rs"));
    if rs_path.exists() {
        eprintln!("Duplicate of {}", rs_path.display());
        return Ok(());
    }

    let path = PathBuf::from(&config.issue_or_path);
    let (code, stderr) = if path.exists() {
        let code = fs::read_to_string(path.as_path())
            .with_context(|| format!("failed to read file: {}", path.display()))?;
        (
            code,
            rustc::go(path.as_path())
                .with_context(|| format!("failed to run rustc on file: {}", path.display()))?,
        )
    } else {
        let output = Command::new("gh")
            .arg("issue")
            .arg("view")
            .arg(&config.issue_or_path)
            .arg("--repo")
            .arg("rust-lang/rust")
            .output()
            .with_context(|| {
                format!(
                    "failed to execute gh command for issue: {}",
                    config.issue_or_path
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to fetch issue: {stderr}");
        }

        let issue_body = String::from_utf8_lossy(&output.stdout);
        let code = extract_rust_code_block(&issue_body).ok_or(anyhow::anyhow!("No code block"))?;
        let stderr = run_rustc_on_temp(&code)?;
        (code, stderr)
    };

    if !check::is_ice(stderr.as_str()) {
        return Err(anyhow::anyhow!("Not an ICE:\n{stderr}"));
    }
    if let Some(existing) = check::exists(&stderr) {
        eprintln!("Duplicate of {existing}");
        let path = PathBuf::from(existing);
        let mut base =
            PathBuf::from(path.file_name().with_context(|| {
                format!("failed to get file name from path: {}", path.display())
            })?);
        let dups = PathBuf::from("ice/dup");
        let mut dup_rs_path = dups.clone();
        dup_rs_path.push(&base);
        let mut dup_out_path = dups.clone();
        base.set_extension("out");
        dup_out_path.push(base);
        if !dup_rs_path.exists() || !dup_out_path.exists() {
            fs::write(&dup_rs_path, code).with_context(|| {
                format!(
                    "failed to write duplicate RS file: {}",
                    dup_rs_path.display()
                )
            })?;
            fs::write(&dup_out_path, stderr).with_context(|| {
                format!(
                    "failed to write duplicate OUT file: {}",
                    dup_out_path.display()
                )
            })?;
        }
        return Ok(());
    }

    let rs_path = PathBuf::from(format!("ice/{base}.rs"));
    let out_path = PathBuf::from(format!("ice/{base}.out"));

    fs::write(&rs_path, code)
        .with_context(|| format!("failed to write RS file: {}", rs_path.display()))?;
    fs::write(&out_path, stderr)
        .with_context(|| format!("failed to write OUT file: {}", out_path.display()))?;

    println!("Saved to {} and {}", rs_path.display(), out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::check::is_ice;
    use crate::rustc;
    use std::fs;
    use std::path::Path;

    #[ignore = "takes a long time"]
    #[test]
    fn test_every_rs_file_produces_ice() {
        let directories = [Path::new("ice"), Path::new("ice/dup")];
        let mut failures = Vec::new();

        for dir in directories {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    let stderr = match rustc::go(&path) {
                        Ok(stderr) => stderr,
                        Err(e) => {
                            failures.push(format!(
                                "{}: Failed to run rustc: {}",
                                path.display(),
                                e
                            ));
                            continue;
                        }
                    };

                    if !is_ice(stderr.as_str()) {
                        failures.push(format!(
                            "{} does not produce an ICE\n\n{stderr}",
                            path.display()
                        ));
                    }
                }
            }
        }

        assert!(
            failures.is_empty(),
            "The following files do not produce ICE messages:\n{}",
            failures.join("\n")
        );
    }
}
