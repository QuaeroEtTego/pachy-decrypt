use super::{ForensicResult, KeySet};

use std::{
    fs,
    process::{Command, Stdio},
    sync::Arc,
};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Pachy(String);

impl Pachy {
    pub fn from_files() -> ForensicResult<Vec<Pachy>> {
        // Collect all paths of .pachy files as vec of String
        Ok(WalkDir::new("./data/file")
            .into_iter()
            .flat_map(|e| e.map(|v| v.into_path()))
            .filter(|p| p.extension().map(|e| e.eq("pachy")).unwrap_or(false))
            .flat_map(|p| p.into_os_string().into_string())
            .map(Self)
            .collect::<Vec<_>>())
    }

    pub fn decrypt(&self, key_sets: Arc<Vec<KeySet>>) -> ForensicResult<()> {
        let path = self.0.as_str();

        for key_iv in key_sets.iter() {
            let key = key_iv.key();
            let iv = key_iv.iv();

            // Since it's just a script, I don't want to waste time by using an OpenSSL crate.
            let command = Command::new("openssl")
                .args([
                    "enc",
                    "-aes-256-cbc",
                    "-d",
                    "-in",
                    path,
                    "-out",
                    &path.replace(".pachy", ""),
                    "-K",
                    key,
                    "-iv",
                    iv,
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?
                .wait()?;

            if command.success() {
                if fs::remove_file(path).is_err() {
                    eprintln!("Failed to remove {path}");
                }

                println!("Successfully decrypted {path} with {key_iv:?}");
                break;
            }
        }

        Ok(())
    }
}
