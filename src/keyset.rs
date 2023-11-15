use super::ForensicResult;

use std::{fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeySet {
    key: String,
    iv: String,
}

impl KeySet {
    pub fn load() -> ForensicResult<Vec<KeySet>> {
        let file = File::open("./data/key-iv.json")?;
        let buf_reader = BufReader::new(file);

        Ok(serde_json::from_reader::<BufReader<File>, Vec<KeySet>>(
            buf_reader,
        )?)
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn iv(&self) -> &str {
        &self.iv
    }
}
