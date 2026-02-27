use std::path::Path;

use crate::error::Result;

pub struct Manifest {
    // TODO: add fields (e.g. list of active SSTable IDs)
}

impl Manifest {
    pub fn create(dir: &Path) -> Result<Self> {
        todo!()
    }

    pub fn load(dir: &Path) -> Result<Self> {
        todo!()
    }

    pub fn add_sstable(&mut self, id: u64) -> Result<()> {
        todo!()
    }

    pub fn remove_sstable(&mut self, id: u64) -> Result<()> {
        todo!()
    }

    pub fn active_sstables(&self) -> &[u64] {
        todo!()
    }
}
