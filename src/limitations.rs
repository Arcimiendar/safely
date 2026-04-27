use anyhow::{anyhow, Context, Result};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use nix::mount::{mount, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::unistd::{getgid, getuid};
use std::fs;
use walkdir::{DirEntry, WalkDir};
use crate::args::Config;

fn build_matcher(config: &Config) -> Result<Gitignore> {
    let mut builder = GitignoreBuilder::new(&config.root_dir);
    for line in config.content.lines() {
        builder.add_line(None, line).map_err(|err| anyhow!(err))?;
    }
    builder.build().map_err(|err| anyhow!(err))
}


pub fn prepare_limitations(config: &Config) -> Result<()> {
    let matcher = build_matcher(config)?;
    enter_namespaces()?;
    WalkDir::new(&config.root_dir)
        .into_iter()
        .flat_map(Result::ok)
        .filter(|entry| does_entry_match(entry, &matcher))
        .try_for_each(block)?;
    Ok(())
}

fn enter_namespaces() -> Result<()> {
    let uid = getuid().as_raw();
    let gid = getgid().as_raw();
    unshare(CloneFlags::CLONE_NEWUSER | CloneFlags::CLONE_NEWNS)
        .context("unshare(CLONE_NEWUSER | CLONE_NEWNS) failed")?;
    fs::write("/proc/self/setgroups", "deny").context("writing setgroups")?;
    fs::write("/proc/self/uid_map", format!("0 {uid} 1")).context("writing uid_map")?;
    fs::write("/proc/self/gid_map", format!("0 {gid} 1")).context("writing gid_map")?;
    Ok(())
}

fn does_entry_match(entry: &DirEntry, matcher: &Gitignore) -> bool {
    matcher
        .matched(entry.path(), entry.file_type().is_dir())
        .is_ignore()
}

fn block(entry: DirEntry) -> Result<()> {
    let target = entry.path();
    if entry.file_type().is_dir() {
        mount(
            Some("tmpfs"),
            target,
            Some("tmpfs"),
            MsFlags::empty(),
            Some("size=0"),
        )
        .with_context(|| format!("tmpfs over {}", target.display()))?;
    } else {
        mount(
            Some("/dev/null"),
            target,
            None::<&str>,
            MsFlags::MS_BIND,
            None::<&str>,
        )
        .with_context(|| format!("bind /dev/null over {}", target.display()))?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn config(content: &str) -> Config {
        Config::new(content.to_string(), PathBuf::from("."))
    }

    #[test]
    fn matcher_matches_pattern_at_any_depth() {
        let matcher = build_matcher(&config("*.txt")).unwrap();
        assert!(matcher.matched("a.txt", false).is_ignore());
        assert!(matcher.matched("nested/a.txt", false).is_ignore());
        assert!(!matcher.matched("a.rs", false).is_ignore());
    }

    #[test]
    fn matcher_anchors_patterns_with_leading_slash() {
        let matcher = build_matcher(&config("/build")).unwrap();
        assert!(matcher.matched("build", true).is_ignore());
        assert!(!matcher.matched("src/build", true).is_ignore());
    }

    #[test]
    fn matcher_supports_negation() {
        let matcher = build_matcher(&config("*.log\n!keep.log")).unwrap();
        assert!(matcher.matched("a.log", false).is_ignore());
        assert!(!matcher.matched("keep.log", false).is_ignore());
    }

    #[test]
    fn matcher_ignores_comments_and_blank_lines() {
        let matcher = build_matcher(&config("# comment\n\n*.txt\n#*.rs")).unwrap();
        assert!(matcher.matched("a.txt", false).is_ignore());
        assert!(!matcher.matched("main.rs", false).is_ignore());
    }

    #[test]
    fn matcher_ignores_empty_lines() {
        let matcher = build_matcher(&config("")).unwrap();
        assert!(!matcher.matched("a.txt", false).is_ignore());
    }

    #[test]
    fn does_entry_match_returns_true_for_matching_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("hit.txt");
        fs::write(&file, "").unwrap();

        let matcher = build_matcher(
            &Config::new("*.txt".into(), dir.path().to_path_buf())
        ).unwrap();
        let entry = WalkDir::new(dir.path())
            .into_iter()
            .flat_map(Result::ok)
            .find(|e| e.file_name() == "hit.txt")
            .unwrap();

        assert!(does_entry_match(&entry, &matcher));
    }

    #[test]
    fn does_entry_match_returns_false_for_non_match() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("miss.rs");
        fs::write(&file, "").unwrap();

        let matcher = build_matcher(
            &Config::new("*.txt".into(), dir.path().to_path_buf())
        ).unwrap();
        let entry = WalkDir::new(dir.path())
            .into_iter()
            .flat_map(Result::ok)
            .find(|e| e.file_name() == "miss.rs")
            .unwrap();

        assert!(!does_entry_match(&entry, &matcher));
    }
}