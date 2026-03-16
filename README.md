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


---

## Roadmap

- SSTable builder & reader with block-based layout and embedded
  bloom filters
- Manifest file for atomic SSTable tracking
- Background compaction
- Benchmarks and performance profiling
---

## Building

```bash
cargo build
cargo test
```

Requires Rust 2021 edition.

---

## Design

Blog post coming soon.
