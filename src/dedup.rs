use std::fs;
use std::path::PathBuf;

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
    fs::create_dir_all(&dups_dir)?;
    fs::create_dir_all(&ok_dir)?;

    // Collect all .rs files in lexical order
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files.sort();

    debug!("Processing {} programs", files.len());

    // Track unique ICEs: map from ICE stderr to the first file that produced it
    let mut unique_ices: Vec<(PathBuf, String)> = Vec::new();

    let bar = indicatif::ProgressBar::new(u64::try_from(files.len()).unwrap());
    for file in &files {
        debug!("Processing {}", file.display());
        let stderr = rustc::go(file.as_path())?;

        if !check::is_ice(&stderr) {
            let file_name = file.file_name().unwrap();
            let dest = ok_dir.join(file_name);
            info!("{}: not an ice, moving to ok/", file.display());
            fs::rename(file, &dest)?;
            continue;
        }

        let mut is_duplicate = false;
        for (original_file, original_stderr) in &unique_ices {
            if check::same(&stderr, original_stderr) {
                // Found a duplicate
                let file_name = file.file_name().unwrap();
                let dest = dups_dir.join(file_name);
                info!(
                    "{}: duplicate of {}, moving to dups/",
                    file.display(),
                    original_file.display()
                );
                fs::rename(file, &dest)?;
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
