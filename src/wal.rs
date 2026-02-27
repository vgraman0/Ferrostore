use std::path::Path;

use crate::error::Result;

pub struct Wal {
    // TODO: add fields (e.g. File handle)
}

impl Wal {
    pub fn create(path: &Path) -> Result<Self> {
        todo!()
    }

    pub fn open(path: &Path) -> Result<Self> {
        todo!()
    }

    pub fn append(&mut self, key: &[u8], value: Option<&[u8]>) -> Result<()> {
        todo!()
    }

    pub fn recover(path: &Path) -> Result<Vec<(Vec<u8>, Option<Vec<u8>>)>> {
        todo!()
    }

    pub fn sync(&mut self) -> Result<()> {
        todo!()
    }
}
