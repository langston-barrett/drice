use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use tracing::{debug, info};

use crate::check;
use crate::rustc;

pub(crate) struct DedupConfig {
    pub directory: PathBuf,
}

pub(crate) fn dedup(config: DedupConfig) -> anyhow::Result<()> {
    let dir = config.directory.as_path();

    // Create output directories
    let dups_dir = dir.join("dups");
    let ok_dir = dir.join("ok");
    let known_dir = dir.join("known");
    fs::create_dir_all(&dups_dir)
        .with_context(|| format!("failed to create directory: {}", dups_dir.display()))?;
    fs::create_dir_all(&ok_dir)
        .with_context(|| format!("failed to create directory: {}", ok_dir.display()))?;
    fs::create_dir_all(&known_dir)
        .with_context(|| format!("failed to create directory: {}", known_dir.display()))?;

    // Collect all .rs files in lexical order
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in
        fs::read_dir(dir).with_context(|| format!("failed to read directory: {}", dir.display()))?
    {
        let entry = entry
            .with_context(|| format!("failed to read entry in directory: {}", dir.display()))?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files.sort();

    debug!("Processing {} programs", files.len());

    // Track unique ICEs: map from ICE stderr to the first file that produced it
    let mut unique_ices: Vec<(PathBuf, String)> = Vec::new();

    let bar = indicatif::ProgressBar::new(
        u64::try_from(files.len())
            .with_context(|| format!("files length {} exceeds u64::MAX", files.len()))?,
    );
    for file in &files {
        debug!("Processing {}", file.display());
        let stderr = rustc::go(file.as_path())
            .with_context(|| format!("failed to run rustc on file: {}", file.display()))?;

        if !check::is_ice(&stderr) {
            let file_name = file.file_name().with_context(|| {
                format!("failed to get file name from path: {}", file.display())
            })?;
            let dest = ok_dir.join(file_name);
            info!("{}: not an ice, moving to ok/", file.display());
            fs::rename(file, &dest)
                .with_context(|| format!("failed to rename file {} to ok/", file.display()))?;
            bar.inc(1);
            continue;
        }

        if let Some(known_ice_path) = check::exists(&stderr) {
            let known_file_name = PathBuf::from(known_ice_path)
                .file_name()
                .with_context(|| {
                    format!("failed to get file name from known ICE path: {known_ice_path}")
                })?
                .to_owned();

            info!(
                "{}: duplicate of {}, moving to known/{}",
                file.display(),
                known_file_name.display(),
                known_file_name.display(),
            );
            fs::rename(file, known_dir.join(known_file_name))
                .with_context(|| format!("failed to rename file {} to known/", file.display()))?;
            bar.inc(1);
            continue;
        }

        let mut is_duplicate = false;
        for (original_file, original_stderr) in &unique_ices {
            if check::same(&stderr, original_stderr) {
                // Found a duplicate
                let file_name = file.file_name().with_context(|| {
                    format!("failed to get file name from path: {}", file.display())
                })?;
                let dest = dups_dir.join(file_name);
                info!(
                    "{}: duplicate of {}, moving to dups/",
                    file.display(),
                    original_file.display()
                );
                fs::rename(file, &dest).with_context(|| {
                    format!("failed to rename file {} to dups/", file.display())
                })?;
                is_duplicate = true;
                break;
            }
        }

        if !is_duplicate {
            unique_ices.push((file.clone(), stderr));
        }
        bar.inc(1);
    }

    Ok(())
}
