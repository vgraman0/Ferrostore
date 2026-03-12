use core::f64;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::error::Result;

pub struct BloomFilter {
    bits: Vec<u8>,
    num_hashes: u32,
    bit_count: usize,
}

impl BloomFilter {
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let n = expected_items as f64;
        let ln2 = f64::consts::LN_2;

        let bit_count = (-n * false_positive_rate.ln() / (ln2 * ln2)).ceil() as usize;
        let num_hashes = ((bit_count as f64 / n) * ln2).ceil() as u32;
        let bits = vec![0u8; bit_count.div_ceil(8)];

        BloomFilter {
            bits,
            num_hashes,
            bit_count,
        }
    }

    fn hash_positions(&self, key: &[u8]) -> Vec<usize> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value: u64 = hasher.finish();

        let h1 = hash_value as u32;
        let h2 = (hash_value >> 32) as u32;

        let m = self.bit_count as u32;
        (0..self.num_hashes)
            .map(|i| (h1.wrapping_add(i.wrapping_mul(h2)) % m) as usize)
            .collect()
    }

    pub fn insert(&mut self, key: &[u8]) {
        for pos in self.hash_positions(key) {
            self.bits[pos / 8] |= 1 << (pos % 8);
        }
    }

    pub fn may_contain(&self, key: &[u8]) -> bool {
        self.hash_positions(key)
            .iter()
            .all(|&pos| self.bits[pos / 8] & (1 << (pos % 8)) != 0)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.num_hashes.to_le_bytes());
        bytes.extend_from_slice(&self.bit_count.to_le_bytes());
        bytes.extend_from_slice(&self.bits);

        bytes
    }

    pub fn decode(data: &[u8]) -> Result<Self> {
        let num_hashes = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let bit_count = usize::from_le_bytes(data[4..12].try_into().unwrap());
        let bits = data[12..].to_vec();

        Ok(BloomFilter {
            bits,
            num_hashes,
            bit_count,
        })
    }
}
