use crate::error::Result;

pub struct BloomFilter {
    // TODO: add fields (e.g. bit vector, number of hash functions)
}

impl BloomFilter {
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        todo!()
    }

    pub fn insert(&mut self, key: &[u8]) {
        todo!()
    }

    pub fn may_contain(&self, key: &[u8]) -> bool {
        todo!()
    }

    pub fn encode(&self) -> Vec<u8> {
        todo!()
    }

    pub fn decode(data: &[u8]) -> Result<Self> {
        todo!()
    }
}
