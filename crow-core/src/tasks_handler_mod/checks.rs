use anyhow::{anyhow, Result};
use mime_guess::from_path;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

use super::traits::FileChecks;

pub struct FileInfo {
    pub path: PathBuf,
}

impl FileChecks for FileInfo {
    fn check_hash(&self) -> Result<String> {
        let content = fs::read(&self.path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }

    fn check_size(&self) -> Result<u64> {
        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.len())
    }

    fn check_name(&self) -> Result<()> {
        if let Some(name) = self.path.file_name() {
            if name.to_string_lossy().is_empty() {
                return Err(anyhow!("File name is empty"));
            }
            Ok(())
        } else {
            Err(anyhow!("File has no name"))
        }
    }

    fn check_type(&self) -> Result<String> {
        let mime = from_path(&self.path).first_or_octet_stream();
        Ok(mime.essence_str().to_string())
    }
}
