use std::fs::File;
use anyhow::{anyhow, Result};
use serde_yaml_ng::from_reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {

}


impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        from_reader(&mut file).map_err(|e| anyhow!(e))
    }
}