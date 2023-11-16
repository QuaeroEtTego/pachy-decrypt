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
        let path_in = self.0.as_str();
        let path_out = path_in
            .rsplitn(2, '.')
            .skip(1)
            .collect::<Vec<_>>()
            .join(".");

        for key_set in key_sets.iter() {
            let key = key_set.key();
            let iv = key_set.iv();

            // Since it's just a script, I don't want to waste time by using an OpenSSL crate.
            let command = Command::new("openssl")
                .args([
                    "enc",
                    "-aes-256-cbc",
                    "-d",
                    "-in",
                    path_in,
                    "-out",
                    &path_out,
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
                if fs::remove_file(path_in).is_err() {
                    eprintln!("Failed to remove {path_in}");
                }

                println!("Successfully decrypted {path_in} with {key_set:?}");
                break;
            }
        }

        Ok(())
    }
}
