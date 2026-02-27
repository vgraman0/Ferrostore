use std::path::PathBuf;

use crate::error::Result;

pub struct DB {
    // TODO: add fields (e.g. MemTable, WAL, SSTable readers, Manifest, path)
}

impl DB {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        todo!()
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        todo!()
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        todo!()
    }

    pub fn delete(&self, key: &[u8]) -> Result<()> {
        todo!()
    }

    pub fn scan(&self, start: &[u8], end: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        todo!()
    }
}
