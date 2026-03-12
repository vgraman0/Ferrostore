use std::path::Path;

use crate::error::Result;

pub struct SSTableBuilder {
    // TODO: fields needed:
    //   - file: File (the .sst file being written)
    //   - current_block: Vec<u8> (buffer for the current 4KB data block)
    //   - index_entries: Vec<(Vec<u8>, u64)> (last key + byte offset for each completed block)
    //   - bloom: BloomFilter (populated as keys are added)
    //   - current_offset: u64 (tracks byte position in the file)
    //   - entry_count: usize (for sizing the bloom filter, or pass to BloomFilter::new)
}

impl SSTableBuilder {
    pub fn new(path: &Path) -> Result<Self> {
        // TODO:
        //   1. Create/open the .sst file at path
        //   2. Initialize an empty block buffer
        //   3. Initialize empty index_entries
        //   4. Create a new BloomFilter (estimate expected_items or use a reasonable default)
        //   5. Set current_offset to 0
        todo!()
    }

    pub fn add(&mut self, key: &[u8], value: Option<&[u8]>) -> Result<()> {
        // TODO:
        //   1. Insert the key into the bloom filter
        //   2. Encode the entry into the current block buffer:
        //      [key_len (u32 LE)] [key bytes] [value_len (u32 LE)] [value bytes]
        //      For tombstones (value=None), use a sentinel like value_len = u32::MAX
        //   3. If current_block.len() >= 4096 (4KB block size):
        //      a. Write the block to the file
        //      b. Record (last_key_in_block, block_start_offset) in index_entries
        //      c. Update current_offset
        //      d. Clear the block buffer
        todo!()
    }

    pub fn finish(self) -> Result<()> {
        // TODO:
        //   1. Flush any remaining data in current_block as a final block
        //      (record its index entry too)
        //   2. Write the index block:
        //      - Record the offset where the index block starts
        //      - For each index entry: [key_len (u32 LE)] [key] [offset (u64 LE)]
        //   3. Write the bloom filter:
        //      - Record the offset where the bloom data starts
        //      - Write bloom.encode()
        //   4. Write a footer (fixed size, at the very end of the file):
        //      [index_block_offset (u64 LE)] [bloom_offset (u64 LE)] [magic number]
        //   5. Flush/sync the file
        todo!()
    }
}
