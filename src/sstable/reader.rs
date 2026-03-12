use std::path::Path;

use crate::error::Result;

pub struct SSTableReader {
    // TODO: fields needed:
    //   - mmap: Mmap (memory-mapped file via memmap2 crate)
    //   - index: Vec<(Vec<u8>, u64)> (last key + offset for each data block)
    //   - bloom: BloomFilter (decoded from the footer)
}

impl SSTableReader {
    pub fn open(path: &Path) -> Result<Self> {
        // TODO:
        //   1. Open the .sst file and memory-map it with memmap2
        //   2. Read the footer (last N bytes) to get index_block_offset and bloom_offset
        //   3. Decode the bloom filter from bloom_offset..index_block_offset region
        //   4. Parse the index block from index_block_offset..bloom_offset
        //      into Vec<(last_key, block_offset)>
        todo!()
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        // TODO:
        //   1. Check bloom filter first — if may_contain returns false, return None
        //   2. Binary search the index to find which data block might contain the key
        //      (find the first block whose last_key >= key)
        //   3. Read that data block from the mmap
        //   4. Scan entries within the block for an exact key match
        //   5. Return the value if found, None otherwise
        //   6. Handle tombstones (value_len == sentinel) — return None
        todo!()
    }

    pub fn scan(&self, start: &[u8], end: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        // TODO:
        //   1. Binary search the index to find the first block whose last_key >= start
        //   2. Starting from that block, iterate through entries across blocks
        //   3. Collect all entries where key is in [start, end]
        //   4. Skip tombstones
        //   5. Stop once you hit a key > end or run out of blocks
        todo!()
    }

    pub fn may_contain(&self, key: &[u8]) -> bool {
        // TODO:
        //   Delegate to self.bloom.may_contain(key)
        todo!()
    }
}
