use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use serde::{Serialize, de::DeserializeOwned};

pub mod error;
pub type Result<T> = std::result::Result<T, error::Error>;

pub struct Cache {
    base: PathBuf,

    // Not used as of now
    ttl: Option<Duration>,
}

impl Cache {
    pub fn new(base: PathBuf, ttl: Option<Duration>) -> Cache {
        Cache { base, ttl }
    }

    fn path_for_key(&self, key: &str) -> PathBuf {
        self.base.join(key).with_extension(".ucache")
    }

    fn expired(&self, path: &Path) -> Result<bool> {
        let Some(ttl) = self.ttl else {
            return Ok(false);
        };

        let modified = std::fs::metadata(path)?.modified()?;
        let expires_at = modified + ttl;

        Ok(SystemTime::now() > expires_at)
    }

    pub fn write<V: Serialize>(&self, key: &str, value: &V) -> Result<()> {
        let _ = std::fs::create_dir(&self.base);
        File::open(self.path_for_key(key))?.write_all(serde_json::to_string(value)?.as_bytes())?;

        Ok(())
    }

    pub fn read<T: DeserializeOwned>(&self, key: &str) -> Result<T> {
        let path = self.path_for_key(key);
        if self.expired(&path)? {
            let _ = std::fs::remove_file(&path);
            return Err(error::Error::Expired);
        }

        let data = std::fs::read(path)?;
        let x: T = serde_json::from_slice(&data)?;

        Ok(x)
    }
}
