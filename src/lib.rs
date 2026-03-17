pub mod bloom;
pub mod compaction;
pub mod db;
pub mod error;
pub mod manifest;
pub mod memtable;
pub mod sstable;
pub mod wal;

pub use db::DB;
pub use error::{Error, Result};

/// A single key-value entry where the value may be `None` (tombstone/delete marker).
pub type Entry = (Vec<u8>, Option<Vec<u8>>);

/// A collection of entries with optional values (used in WAL recovery and block reads).
pub type Entries = Vec<Entry>;

/// A collection of owned key-value pairs (used in scan results).
pub type ScanResult = Vec<(Vec<u8>, Vec<u8>)>;

/// An index mapping keys to byte offsets.
pub type IndexEntries = Vec<(Vec<u8>, u64)>;
