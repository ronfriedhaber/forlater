use std::{fs::File, io::Write, path::PathBuf, time::Duration};

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

    pub fn write<V: Serialize>(&self, key: &str, value: &V) -> Result<()> {
        File::open(self.path_for_key(key))?.write_all(serde_json::to_string(value)?.as_bytes())?;

        Ok(())
    }

    pub fn read<T: DeserializeOwned>(&self, key: &str) -> Result<T> {
        let data = std::fs::read(self.path_for_key(key))?;
        let x: T = serde_json::from_slice(&data)?;

        Ok(x)
    }
}
