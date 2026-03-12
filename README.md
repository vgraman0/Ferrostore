# RustyLSM

A persistent key-value storage engine built on a **Log-Structured Merge-tree (LSM)** architecture, written in Rust.

LSM trees are the backbone of modern storage systems like LevelDB, RocksDB, and Cassandra. They're optimized for **write-heavy workloads** — every write is sequential (no random I/O), giving you throughput that spinning disks and SSDs both love.

---

## How LSM Trees Work

<p align="center">
  <img src="/assets/2026-03-12-23-26-28.png" width="600">
</p>

### Write Path
Every write first appends to the **Write-Ahead Log** (WAL) for durability, then inserts into the **MemTable** — an in-memory sorted structure. When the memtable hits a size threshold, it's flushed to disk as an immutable **SSTable** (Sorted String Table).

### Read Path
Reads check the **MemTable** first (most recent data), then walk through SSTables from newest to oldest. **Bloom filters** attached to each SSTable let the engine skip files that definitely don't contain the key — avoiding unnecessary disk reads.

### Compaction
Over time, SSTables accumulate. A background compaction process merges overlapping tables, discards old versions and tombstones, and produces fewer, larger files. This keeps read amplification in check.

---

## Features Implemented

### SkipList MemTable
In-memory sorted store backed by a custom skip list. Supports `put`, `get`, `delete`, and `scan` — all in O(log n). Tombstone-based deletes ensure correctness during scans and future SSTable flushes.

### Write-Ahead Log (WAL)
Binary-serialized, append-only log. Each entry is tagged (`put` or `delete`) with length-prefixed keys and values. On startup, the WAL is replayed into the memtable — no data is lost between restarts.

### Bloom Filter
Probabilistic key lookup with a configurable false-positive rate. Uses a double-hashing technique (splitting a single 64-bit hash into two 32-bit halves) to simulate multiple hash functions. Serializable to bytes for embedding in SSTables.

### Database API
Clean top-level interface tying it all together:

```rust
let mut db = DB::open("my_db")?;

db.put(b"name", b"RustyLSM")?;
db.get(b"name")?;              // Some(b"RustyLSM")
db.delete(b"name")?;
db.scan(b"a", b"z")?;          // Vec of (key, value) pairs
```

Every mutation flows through the WAL before touching the memtable, and recovery is automatic on `open`.

---

## Planned / In Progress

- **SSTable builder & reader** — sorted string tables with 4 KB block-based layout, index blocks, and embedded bloom filters
- **Manifest file** — atomic tracking of the active set of SSTables
- **Background compaction** — merge overlapping SSTables to reduce read amplification

---

## Demo

<!-- TODO: Add demo video for basic operations -->

<!-- TODO: Add demo video for crash recovery -->

---

## Getting Started

```bash
cargo build
cargo test
```

Requires Rust 2021 edition.
