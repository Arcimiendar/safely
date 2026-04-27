use anyhow::{anyhow, Result};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use walkdir::{DirEntry, WalkDir};

fn build_matcher(config: &str) -> Result<Gitignore> {
    let mut builder = GitignoreBuilder::new("./");
    for line in config.lines() {
        builder.add_line(None, line).map_err(|err| anyhow!(err))?;
    }
    builder.build().map_err(|err| anyhow!(err))
}


pub fn prepare_limitations(config: &str) -> Result<()> {
    let matcher = build_matcher(config)?;
    WalkDir::new("./")
        .into_iter()
        .flat_map(Result::ok)
        .filter(|entry| does_entry_match(entry, &matcher))
        .for_each(block);
    Ok(())
}

fn does_entry_match(entry: &DirEntry, matcher: &Gitignore) -> bool {
    matcher
        .matched(entry.path(), entry.file_type().is_dir())
        .is_ignore()
}

fn block(entry: DirEntry) {
    println!("Entry: {:?}", entry);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn matcher_matches_pattern_at_any_depth() {
        let matcher = build_matcher("*.txt").unwrap();
        assert!(matcher.matched("a.txt", false).is_ignore());
        assert!(matcher.matched("nested/a.txt", false).is_ignore());
        assert!(!matcher.matched("a.rs", false).is_ignore());
    }

    #[test]
    fn matcher_anchors_patterns_with_leading_slash() {
        let matcher = build_matcher("/build").unwrap();
        assert!(matcher.matched("build", true).is_ignore());
        assert!(!matcher.matched("src/build", true).is_ignore());
    }

    #[test]
    fn matcher_supports_negation() {
        let matcher = build_matcher("*.log\n!keep.log").unwrap();
        assert!(matcher.matched("a.log", false).is_ignore());
        assert!(!matcher.matched("keep.log", false).is_ignore());
    }

    #[test]
    fn matcher_ignores_comments_and_blank_lines() {
        let matcher = build_matcher("# comment\n\n*.txt").unwrap();
        assert!(matcher.matched("a.txt", false).is_ignore());
    }

    #[test]
    fn does_entry_match_returns_true_for_matching_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("hit.txt");
        fs::write(&file, "").unwrap();

        let matcher = build_matcher("*.txt").unwrap();
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

        let matcher = build_matcher("*.txt").unwrap();
        let entry = WalkDir::new(dir.path())
            .into_iter()
            .flat_map(Result::ok)
            .find(|e| e.file_name() == "miss.rs")
            .unwrap();

        assert!(!does_entry_match(&entry, &matcher));
    }

    #[test]
    fn prepare_limitations_succeeds_with_valid_config() {
        assert!(prepare_limitations("*.txt").is_ok());
    }
}