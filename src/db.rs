use std::path::PathBuf;

use crate::{
    config::DbConfig,
    error::Result,
    memtable::{MemTable, SkipListMemTable},
    wal::Wal,
    ScanResult,
};

pub struct DB {
    memtable: SkipListMemTable,
    wal: Wal,
    config: DbConfig,
}

impl DB {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        Self::open_with_config(path, DbConfig::default())
    }

    pub fn open_with_config(path: impl Into<PathBuf>, config: DbConfig) -> Result<Self> {
        let path = path.into();
        let mut memtable =
            SkipListMemTable::new(config.skip_list_max_level, config.skip_list_probability);

        if path.exists() {
            for (key, value) in Wal::recover(path.as_path())? {
                match value {
                    Some(v) => memtable.put(key, v),
                    None => memtable.delete(&key),
                }
            }
        }

        let wal = Wal::open(path.as_path())?;
        Ok(Self {
            memtable,
            wal,
            config,
        })
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        self.memtable.put(key.to_vec(), value.to_vec());
        self.wal.append(key, Some(value))?;

        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.memtable.get(key).map(|v| v.to_vec()))
    }

    pub fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.memtable.delete(key);
        self.wal.append(key, None)?;

        Ok(())
    }

    pub fn scan(&self, start: &[u8], end: &[u8]) -> Result<ScanResult> {
        Ok(self
            .memtable
            .scan(start, end)
            .into_iter()
            .map(|v| (v.0.to_vec(), v.1.to_vec()))
            .collect())
    }
}
