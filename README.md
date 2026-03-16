# Ferrostore 

A persistent, write-optimized key-value storage engine in Rust, built from first principles on LSM-tree architecture.

Skip list memtable · write-ahead log · bloom-filtered SSTables · background compaction

---

## Usage

```rust
let mut db = DB::open("my_db")?;

db.put(b"name", b"Ferrostore")?;
db.get(b"name")?;              // Some(b"Ferrostore")
db.delete(b"name")?;
db.scan(b"a", b"z")?;          // Vec of (key, value) pairs
```

### Building

```bash
cargo build
cargo test
```

Requires Rust 2021 edition.

---

## Roadmap

### Core Engine
- [x] Skip list memtable with write-ahead log
- [x] SSTable builder with block-based layout and embedded bloom filters
- [ ] SSTable reader with point lookups and range scans
- [ ] Integrate SSTables into DB read/write path (memtable flushing, read ordering)
- [ ] Manifest file for atomic SSTable tracking
- [ ] Leveled compaction (L0 overlapping, L1+ non-overlapping, 10x size targets)
- [ ] CRC32 checksums for SSTables, WAL, and manifest

### Performance
- [ ] Cache-line-aligned skip list nodes
- [ ] Cache-line-aligned SSTable blocks
- [ ] Software prefetch for SSTable binary search
- [ ] LRU block cache for SSTable data blocks

### Concurrency
- [ ] Concurrent memtable access (multiple readers, single writer)
- [ ] MVCC with sequence numbers and snapshot isolation

### Advanced
- [ ] Learned index (piecewise linear approximation) for SSTables
- [ ] Adaptive compaction (per-level leveled vs tiered strategy)
- [ ] io_uring async SSTable I/O (Linux)

### Benchmarking
- [ ] YCSB benchmark harness (workloads A/B/C/D/F)
- [ ] Learned index vs traditional index comparison
- [ ] Cache optimization impact (hardware perf counters)
- [ ] Comparative benchmarks against RocksDB and Sled

### Quality
- [ ] Comprehensive test suite (property-based, crash consistency, >90% coverage)
- [ ] API documentation and performance tuning guide
---

## Design

Blog post coming soon.
