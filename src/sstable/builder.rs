use std::path::Path;

use crate::error::Result;

pub struct SSTableBuilder {
    // TODO: add fields (e.g. file handle, current block buffer, index entries)
}

impl SSTableBuilder {
    pub fn new(path: &Path) -> Result<Self> {
        todo!()
    }

    pub fn add(&mut self, key: &[u8], value: Option<&[u8]>) -> Result<()> {
        todo!()
    }

    pub fn finish(self) -> Result<()> {
        todo!()
    }
}
