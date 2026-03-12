use std::path::Path;

use crate::error::Result;

pub struct Manifest {
    // TODO: fields needed:
    //   - dir: PathBuf (directory where the MANIFEST file lives)
    //   - active: Vec<u64> (list of active SSTable IDs, ordered)
}

impl Manifest {
    pub fn create(dir: &Path) -> Result<Self> {
        // TODO:
        //   1. Initialize with an empty active list
        //   2. Write an empty MANIFEST file to dir/MANIFEST
        //   3. Store the dir path
        todo!()
    }

    pub fn load(dir: &Path) -> Result<Self> {
        // TODO:
        //   1. Read dir/MANIFEST
        //   2. Parse the list of active SSTable IDs from it
        //      (e.g. one ID per line, or a simple binary format)
        //   3. Return the populated Manifest
        todo!()
    }

    pub fn add_sstable(&mut self, id: u64) -> Result<()> {
        // TODO:
        //   1. Add id to self.active
        //   2. Persist atomically: write to a temp file, then rename over MANIFEST
        //      (this ensures a crash mid-write doesn't corrupt the manifest)
        todo!()
    }

    pub fn remove_sstable(&mut self, id: u64) -> Result<()> {
        // TODO:
        //   1. Remove id from self.active
        //   2. Persist atomically (same temp-file + rename approach)
        todo!()
    }

    pub fn active_sstables(&self) -> &[u64] {
        // TODO:
        //   Return a slice of self.active
        todo!()
    }
}
