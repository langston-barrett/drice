use std::{fs, path::PathBuf};

use anyhow::Context;

use crate::{check, rustc};

pub(crate) struct GlanceConfig {
    pub file: PathBuf,
}

pub(crate) fn glance(config: GlanceConfig) -> anyhow::Result<()> {
    let content = if config.file.extension().and_then(|s| s.to_str()) == Some("rs") {
        rustc::go(config.file.as_path())
            .with_context(|| format!("failed to run rustc on file: {}", config.file.display()))?
    } else {
        fs::read_to_string(config.file.as_path())
            .with_context(|| format!("failed to read file: {}", config.file.display()))?
    };
    let path = check::extract_file_path(&content);
    let message = check::extract_message(&content);
    let stack = check::extract_query_stack(&content);
    println!("{}", path.as_deref().unwrap_or("(no path)"));
    println!("{}", message.as_deref().unwrap_or("(no message)"));
    println!("{}", stack.as_deref().unwrap_or("(no query stack)"));
    Ok(())
}
