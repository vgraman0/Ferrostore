use std::path::Path;

use crate::error::Result;

pub struct SSTableReader {
    // TODO: add fields (e.g. Mmap handle, index, bloom filter)
}

impl SSTableReader {
    pub fn open(path: &Path) -> Result<Self> {
        todo!()
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        todo!()
    }

    pub fn scan(&self, start: &[u8], end: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        todo!()
    }

    pub fn may_contain(&self, key: &[u8]) -> bool {
        todo!()
    }
}
