use std::{fs, path::PathBuf};

use anyhow::Context;
use tracing::debug;

use crate::rustc;

pub(crate) struct CheckConfig {
    pub file: PathBuf,
}

pub(crate) fn is_ice(out: &str) -> bool {
    out.contains("error: internal compiler error:")
        || out.contains("error: the compiler unexpectedly panicked")
        // This produces lots of false positives with stack overflow...
        || out.contains("error: rustc interrupted by SIGSEGV, printing backtrace")
}

fn code_uses_internal_features(code: &str) -> bool {
    for feat in [
        "break rust",
        "core_intrinsics", // feature(..)
        "#[custom_mir",
        "#[lang",
        "lang_items", // feature(..)
        "mir!",
        "#![no_core]",
        "platform_intrinsics", // feature(..)
        "rustc_attrs",         // [  ]
        "#[rustc_",
        // "#[rustc_dummy]",
        // "#[rustc_has_incoherent_inherent_impls]",
        // "#[rustc_intrinsic]",
        "rustc_layout_scalar_valid_range_end", // rustc attr
        "#[rustc_symbol_name]",
        "#[rustc_variance]",
        "#[rustc_variance]",
        "staged_api", // feature(..)
                      // ???
                      // "::SIGSEGV",
                      // "SIGSEGV::",
                      // "span_delayed_bug_from_inside_query",
    ] {
        if code.contains(feat) {
            return true;
        }
    }
    false
}

fn uses_internal_features(out: &str) -> bool {
    for feat in [
        "core_intrinsics",
        "Projecting into SIMD type", // TODO: remove and triage
        "break rust",
        "is internal to the compiler or standard library",
        "is incomplete and may not be safe to use and/or cause compiler crashes",
        "is an experimental feature", // #[loop_match]
    ] {
        if out.contains(feat) {
            return true;
        }
    }
    false
}

pub(crate) fn same(l: &str, r: &str) -> bool {
    if l == r {
        return true;
    }
    let l_path = extract_file_path(l);
    let r_path = extract_file_path(r);
    let l_msg = extract_message(l);
    let r_msg = extract_message(r);
    let l_stack = extract_query_stack(l);
    let r_stack = extract_query_stack(r);
    debug!("{l_path:?}");
    debug!("{r_path:?}");
    debug!("{}", l_path == r_path);
    debug!("{l_msg:?}");
    debug!("{r_msg:?}");
    debug!("{}", l_msg == r_msg);
    debug!("{l_stack:?}");
    debug!("{r_stack:?}");
    debug!("{}", l_stack == r_stack);
    if l_path.is_some() && l_path == r_path && l_stack == r_stack {
        if let Some(l_msg) = l_msg
            && let Some(r_msg) = r_msg
            && let Some(l_fst) = l_msg.split_ascii_whitespace().next()
            && let Some(r_fst) = r_msg.split_ascii_whitespace().next()
        {
            // TODO: Replace with a better (string distance?) check
            if !l_fst.starts_with('`')
                && !l_fst.starts_with('[')
                && !r_fst.starts_with('`')
                && !r_fst.starts_with('[')
                && l_fst != r_fst
            {
                return false;
            }
        }
        return true;
    }
    false
}

pub(crate) fn extract_file_path(s: &str) -> Option<String> {
    for line in s.lines() {
        if line.starts_with("thread 'rustc'")
            && let Some(idx) = line.find("panicked at ")
        {
            let idx = idx + "panicked at ".len();
            return Some(line[idx..line.len() - 1].to_owned());
        }
        if let Some(idx) = line.find("note: delayed at ") {
            let idx = idx + "note: delayed at ".len();
            return Some(line[idx..line.len() - 1].to_owned());
        }
    }
    None
}

pub(crate) fn extract_message(s: &str) -> Option<String> {
    let mut is_next = false;
    for line in s.lines() {
        if is_next {
            return Some(line.to_owned());
        }
        if let Some(line) = line.strip_prefix("error: internal compiler error: ") {
            let first = &line[0..line.find(' ').unwrap_or(line.len())];
            if first.contains('/') && first.contains(':') {
                return Some(line[first.len() + 1..].to_owned());
            }
            return Some(line.to_owned());
        }
        if line.starts_with("thread 'rustc'") && line.contains("panicked at ") {
            is_next = true;
        }
    }
    None
}

pub(crate) fn extract_query_stack(s: &str) -> Option<String> {
    let lines: Vec<&str> = s.lines().collect();
    let start_idx = lines
        .iter()
        .position(|line| line.trim() == "query stack during panic:")?;
    let end_idx = lines[start_idx..]
        .iter()
        .position(|line| line.trim() == "end of query stack")?;
    let mut out = String::with_capacity(end_idx);
    for line in lines[start_idx..=start_idx + end_idx].iter().copied() {
        // exclude names from program
        if let Some(first) = line.find('`')
            && let Some(last) = line.rfind('`')
        {
            out.push_str(&line[..first]);
            out.push('`');
            out.push_str(&line[last..]);
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    Some(out)
}

pub(crate) fn exists(s: &str) -> Option<&'static str> {
    for (nm, stderr) in crate::ice::ICES.iter().copied() {
        if same(s, stderr) {
            return Some(nm);
        }
    }
    None
}

pub(crate) fn check(config: CheckConfig) -> anyhow::Result<()> {
    let p = format!("{}", config.file.display());
    let mut s = fs::read_to_string(config.file.as_path())
        .with_context(|| format!("failed to read file: {}", config.file.display()))?;
    if p.ends_with(".rs") {
        if code_uses_internal_features(s.as_str()) {
            eprintln!("{p}: skipping, uses internal features");
            return Ok(());
        }
        s = rustc::go(config.file.as_path())
            .with_context(|| format!("failed to run rustc on file: {}", config.file.display()))?;
    }
    if !is_ice(s.as_str()) {
        eprintln!("{p}: not an ICE");
        return Ok(());
    }
    if uses_internal_features(s.as_str()) {
        eprintln!("{p}: skipping, uses internal features");
        return Ok(());
    }
    if let Some(existing) = exists(s.as_str()) {
        eprintln!("{p}: duplicate of {existing}");
    } else {
        eprintln!("{p}: appears new!");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::same;
    use crate::{check::extract_file_path, ice::ICES};

    #[test]
    fn test_extract_file_path() {
        assert_eq!(extract_file_path("foo").as_deref(), None);
        assert_eq!(
            extract_file_path(
                "thread 'rustc' (843541) panicked at compiler/rustc_hir_analysis/src/hir_ty_lowering/dyn_trait.rs:425:17:"
            ).as_deref(),
            Some("compiler/rustc_hir_analysis/src/hir_ty_lowering/dyn_trait.rs:425:17")
        );
    }

    #[test]
    fn test_same_reflexive() {
        for (_, content) in ICES {
            assert!(same(content, content));
        }
    }

    #[ignore = "spuriously fails in CI"]
    #[test]
    fn test_same_distinct() {
        for (i, (nm0, content1)) in ICES.iter().copied().enumerate() {
            for (j, (nm1, content2)) in ICES.iter().skip(i + 1).copied().enumerate() {
                if i != j {
                    if nm0 == "ice/122529.rs" && nm1 == "ice/141124.rs" {
                        continue; // TODO: Are these dups?
                    }
                    if nm0 == "ice/127643.rs" && nm1 == "ice/131046.rs" {
                        continue; // TODO these are maybe dups?
                    }
                    if nm0 == "ice/127643.rs" && nm1 == "ice/131406.rs" {
                        continue; // TODO these are maybe dups?
                    }
                    if nm0 == "ice/131046.rs" && nm1 == "ice/131406.rs" {
                        continue; // TODO these are maybe dups?
                    }
                    if nm0 == "ice/135845.rs" && nm1 == "ice/139738.rs" {
                        continue; // TODO: Are these dups?
                    }
                    if nm0 == "ice/139738.rs" && nm1 == "ice/141504.rs" {
                        continue; // See below
                    }
                    if nm0 == "ice/135845.rs" && nm1 == "ice/141504.rs" {
                        continue; // See latter, subtle semantic distinction
                    }
                    if nm0 == "ice/123959.rs" && nm1 == "ice/98322.rs" {
                        continue; // TODO?
                    }

                    assert!(
                        !same(content1, content2),
                        "ICE {nm0} should not be the same as ICE {nm1}",
                    );
                }
            }
        }
    }

    #[test]
    fn test_dup_files_same_as_original() {
        use std::fs;
        use std::path::Path;

        // tracing_subscriber::fmt::fmt()
        //     .with_max_level(tracing::Level::TRACE)
        //     .without_time()
        //     .init();

        let dup_dir = Path::new("ice/dup");
        let ice_dir = Path::new("ice");

        for entry in fs::read_dir(dup_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("out") {
                let file_name = path.file_name().unwrap().to_str().unwrap();

                let dup_path = dup_dir.join(file_name);
                let ice_path = ice_dir.join(file_name);

                let dup_content = fs::read_to_string(&dup_path)
                    .unwrap_or_else(|_| panic!("Failed to read {}", dup_path.display()));
                let ice_content = fs::read_to_string(&ice_path)
                    .unwrap_or_else(|_| panic!("Failed to read {}", ice_path.display()));

                if file_name == "144241.out" {
                    // different query stacks, but same line
                    continue;
                }

                assert!(
                    same(&dup_content, &ice_content),
                    "File ice/dup/{file_name} should be a duplicate of ice/{file_name}",
                );
            }
        }
    }

    #[test]
    fn test_every_rs_file_has_out_file() {
        use std::fs;
        use std::path::Path;

        let directories = [Path::new("ice"), Path::new("ice/dup")];

        for dir in &directories {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    let file_stem = path.file_stem().unwrap().to_str().unwrap();
                    let out_path = dir.join(format!("{file_stem}.out"));

                    assert!(
                        out_path.exists(),
                        "File {} should have a corresponding .out file at {}",
                        path.display(),
                        out_path.display()
                    );
                }
            }
        }
    }
}
