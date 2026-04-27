use anyhow::{anyhow, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use nix::libc::glob;
use walkdir::{DirEntry, WalkDir};

fn get_glob_matcher(config: &str) -> Result<GlobSet> {
    let mut glob_builder = GlobSetBuilder::new();

    for line in config.split('\n') {
        if let Ok(glob) = Glob::new(line) {
            glob_builder.add(glob);
        }
    }

    glob_builder.build().map_err(|err| anyhow!(err))
}


pub fn prepare_limitations(config_path: &str) -> Result<()> {
    let glob_set = get_glob_matcher(config_path)?;
    WalkDir::new("./")
        .into_iter()
        .flat_map(Result::ok)
        .filter(|entry| does_entry_matches(entry, &glob_set))
        .for_each(block);
    Ok(())
}

fn does_entry_matches(entry: &DirEntry, glob_set: &GlobSet) -> bool {
    let path = entry.path();

    let relative = path.strip_prefix("./").unwrap_or(path);

    glob_set.is_match(relative)
}

fn block(entry: DirEntry) {
    println!("Entry: {:?}", entry);
}
