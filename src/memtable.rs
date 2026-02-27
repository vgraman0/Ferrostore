pub struct MemTable {
    // TODO: add fields (e.g. SkipList or BTreeMap)
}

impl MemTable {
    pub fn new() -> Self {
        todo!()
    }

    pub fn put(&self, key: &[u8], value: &[u8]) {
        todo!()
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        todo!()
    }

    pub fn delete(&self, key: &[u8]) {
        todo!()
    }

    pub fn scan(&self, start: &[u8], end: &[u8]) -> Vec<(Vec<u8>, Option<Vec<u8>>)> {
        todo!()
    }

    pub fn approximate_size(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Returns all entries as (key, value) pairs.
    /// A `None` value indicates a tombstone (deletion marker).
    pub fn entries(&self) -> Vec<(Vec<u8>, Option<Vec<u8>>)> {
        todo!()
    }
}
