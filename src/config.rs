use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub skip_list_max_level: usize,
    pub skip_list_probability: f64,
    pub block_size: usize,
    pub bloom_false_positive_rate: f64,
    pub bloom_expected_entries: usize,
    pub l0_compaction_threshold: usize,
    pub compaction_poll_interval: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            skip_list_max_level: 16,
            skip_list_probability: 0.5,
            block_size: 4096,
            bloom_false_positive_rate: 0.01,
            bloom_expected_entries: 10_000,
            l0_compaction_threshold: 4,
            compaction_poll_interval: Duration::from_millis(500),
        }
    }
}
