use std::{fs::File, path::Path};

use memmap2::Mmap;

use crate::{
    bloom::BloomFilter,
    error::{Error, Result},
    sstable::SSTABLE_MAGIC,
    Entries, IndexEntries, ScanResult,
};

pub struct SSTableReader {
    mmap: Mmap,
    index_entries: IndexEntries,
    bloom_filter: BloomFilter,
    index_block_offset: usize,
}

impl SSTableReader {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        let footer = mmap
            .last_chunk::<24>()
            .ok_or(Error::Corruption("file too small for footer".into()))?;

        let index_block_offset = u64::from_le_bytes(footer[0..8].try_into().unwrap()) as usize;
        let bloom_offset = u64::from_le_bytes(footer[8..16].try_into().unwrap()) as usize;
        let magic_number = u64::from_le_bytes(footer[16..24].try_into().unwrap());

        if magic_number != SSTABLE_MAGIC {
            return Err(Error::Corruption("invalid SSTable magic number".into()));
        }

        let bloom_bytes =
            Self::read_bytes_from(&mmap, bloom_offset, mmap.len() - 24 - bloom_offset)?;
        let bloom_filter = BloomFilter::decode(bloom_bytes)?;

        let mut index_entries = Vec::new();
        let mut current_offset = index_block_offset;
        while current_offset < bloom_offset {
            let key_len = Self::read_u32_from(&mmap, current_offset)? as usize;
            current_offset += 4;
            let key = Self::read_bytes_from(&mmap, current_offset, key_len)?.to_vec();
            current_offset += key_len;
            let block_offset = Self::read_u64_from(&mmap, current_offset)?;
            current_offset += 8;
            index_entries.push((key, block_offset));
        }

        Ok(SSTableReader {
            mmap,
            index_entries,
            bloom_filter,
            index_block_offset,
        })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        if !self.may_contain(key) {
            return Ok(None);
        }

        let idx = self.index_entries.partition_point(|x| x.0.as_slice() < key);
        if idx >= self.index_entries.len() {
            return Ok(None);
        }

        let block_start = self.index_entries[idx].1 as usize;
        let block = self.read_block(block_start, self.block_end_offset(idx))?;

        for (k, v) in block.into_iter() {
            if k.as_slice() == key {
                return Ok(v);
            }
        }

        Ok(None)
    }

    pub fn scan(&self, start: &[u8], end: &[u8]) -> Result<ScanResult> {
        let mut entries = Vec::new();
        let idx = self
            .index_entries
            .partition_point(|x| x.0.as_slice() < start);
        if idx >= self.index_entries.len() {
            return Ok(entries);
        }

        for i in idx..self.index_entries.len() {
            let (last_key, offset) = &self.index_entries[i];
            let block = self.read_block(*offset as usize, self.block_end_offset(i))?;
            for (key, val) in block {
                if let Some(val) = val {
                    if key.as_slice() >= start && key.as_slice() <= end {
                        entries.push((key, val));
                    }
                }
            }

            if last_key.as_slice() > end {
                break;
            }
        }
        Ok(entries)
    }

    pub fn may_contain(&self, key: &[u8]) -> bool {
        self.bloom_filter.may_contain(key)
    }

    fn block_end_offset(&self, idx: usize) -> usize {
        if idx == self.index_entries.len() - 1 {
            self.index_block_offset
        } else {
            self.index_entries[idx + 1].1 as usize
        }
    }

    fn read_block(
        &self,
        block_start_offset: usize,
        block_end_offset: usize,
    ) -> Result<Entries> {
        let mut entries = Vec::new();
        let mut current_offset = block_start_offset;

        while current_offset < block_end_offset {
            let key_len = Self::read_u32_from(&self.mmap, current_offset)? as usize;
            current_offset += 4;
            let key = Self::read_bytes_from(&self.mmap, current_offset, key_len)?.to_vec();
            current_offset += key_len;
            let val_len = Self::read_u32_from(&self.mmap, current_offset)?;
            current_offset += 4;
            let val = if val_len == u32::MAX {
                None
            } else {
                let val_len = val_len as usize;
                Some(Self::read_bytes_from(&self.mmap, current_offset, val_len)?.to_vec())
            };
            current_offset += if val_len == u32::MAX { 0 } else { val_len } as usize;
            entries.push((key, val));
        }

        Ok(entries)
    }

    fn read_bytes_from(mmap: &[u8], offset: usize, len: usize) -> Result<&[u8]> {
        mmap.get(offset..offset + len)
            .ok_or_else(|| Error::Corruption(format!("unexpected end of data at offset {offset}")))
    }

    fn read_u32_from(mmap: &[u8], offset: usize) -> Result<u32> {
        let bytes = Self::read_bytes_from(mmap, offset, 4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn read_u64_from(mmap: &[u8], offset: usize) -> Result<u64> {
        let bytes = Self::read_bytes_from(mmap, offset, 8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }
}
