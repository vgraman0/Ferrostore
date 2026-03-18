use rusty_lsm::error;
use rusty_lsm::sstable::{SSTableBuilder, SSTableReader};

fn build_sstable(
    dir: &std::path::Path,
    entries: &[(&[u8], Option<&[u8]>)],
) -> error::Result<SSTableReader> {
    let path = dir.join("test.sst");
    let mut builder = SSTableBuilder::new(&path, entries.len(), 4096, 0.01)?;
    for (key, value) in entries {
        builder.add(key, *value)?;
    }
    builder.finish()?;
    SSTableReader::open(&path)
}

#[test]
fn roundtrip_single_entry() -> error::Result<()> {
    let dir = tempfile::tempdir()?;
    let reader = build_sstable(dir.path(), &[(b"key", Some(b"val"))])?;

    assert_eq!(reader.get(b"key")?, Some(b"val".to_vec()));
    Ok(())
}

#[test]
fn roundtrip_multiple_entries() -> error::Result<()> {
    let dir = tempfile::tempdir()?;
    let entries: Vec<(&[u8], Option<&[u8]>)> = (0..10)
        .map(|i| {
            let key: &[u8] = Box::leak(format!("key{i:03}").into_bytes().into_boxed_slice());
            let val: &[u8] = Box::leak(format!("val{i:03}").into_bytes().into_boxed_slice());
            (key, Some(val))
        })
        .collect();

    let reader = build_sstable(dir.path(), &entries)?;

    for (key, value) in &entries {
        assert_eq!(reader.get(key)?, value.map(|v| v.to_vec()));
    }
    Ok(())
}

#[test]
fn get_returns_none_for_missing_key() -> error::Result<()> {
    let dir = tempfile::tempdir()?;
    let reader = build_sstable(dir.path(), &[(b"aaa", Some(b"v"))])?;

    assert_eq!(reader.get(b"zzz_missing")?, None);
    Ok(())
}

#[test]
fn tombstone_roundtrip() -> error::Result<()> {
    let dir = tempfile::tempdir()?;
    let reader = build_sstable(
        dir.path(),
        &[(b"alive", Some(b"v")), (b"dead", None)],
    )?;

    // Tombstone returns None from get(), same as a missing key
    assert_eq!(reader.get(b"dead")?, None);
    assert_eq!(reader.get(b"alive")?, Some(b"v".to_vec()));
    Ok(())
}

#[test]
fn scan_returns_range() -> error::Result<()> {
    let dir = tempfile::tempdir()?;
    let entries: Vec<(&[u8], Option<&[u8]>)> = vec![
        (b"a", Some(b"1")),
        (b"b", Some(b"2")),
        (b"c", Some(b"3")),
        (b"d", Some(b"4")),
        (b"e", Some(b"5")),
    ];
    let reader = build_sstable(dir.path(), &entries)?;

    let results = reader.scan(b"b", b"d")?;
    assert_eq!(
        results,
        vec![
            (b"b".to_vec(), b"2".to_vec()),
            (b"c".to_vec(), b"3".to_vec()),
            (b"d".to_vec(), b"4".to_vec()),
        ]
    );
    Ok(())
}

#[test]
fn scan_returns_empty_for_out_of_range() -> error::Result<()> {
    let dir = tempfile::tempdir()?;
    let reader = build_sstable(
        dir.path(),
        &[(b"a", Some(b"1")), (b"b", Some(b"2"))],
    )?;

    let results = reader.scan(b"x", b"z")?;
    assert!(results.is_empty());
    Ok(())
}

#[test]
fn bloom_filter_rejects_missing_keys() -> error::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("test.sst");
    let count = 100;
    let mut builder = SSTableBuilder::new(&path, count, 4096, 0.01)?;
    for i in 0..count {
        let key = format!("key{i:04}");
        let val = format!("val{i:04}");
        builder.add(key.as_bytes(), Some(val.as_bytes()))?;
    }
    builder.finish()?;
    let reader = SSTableReader::open(&path)?;

    // With 100 entries and 1% FPR, testing 100 absent keys should yield very few false positives
    let false_positives: usize = (0..100)
        .filter(|i| reader.may_contain(format!("missing{i:04}").as_bytes()))
        .count();
    assert!(false_positives < 5, "too many false positives: {false_positives}");
    Ok(())
}

#[test]
fn many_entries_span_multiple_blocks() -> error::Result<()> {
    let dir = tempfile::tempdir()?;

    // Each entry is ~108 bytes (4 + 50 + 4 + 50), so ~40 entries per 4096-byte block.
    // 200 entries should span ~5 blocks.
    let entries: Vec<(&[u8], Option<&[u8]>)> = (0..200)
        .map(|i| {
            let key: &[u8] =
                Box::leak(format!("key{i:04}_{:0>44}", "").into_bytes().into_boxed_slice());
            let val: &[u8] =
                Box::leak(format!("val{i:04}_{:0>44}", "").into_bytes().into_boxed_slice());
            (key, Some(val))
        })
        .collect();

    let reader = build_sstable(dir.path(), &entries)?;

    // Spot-check first, middle, and last entries
    for &idx in &[0, 99, 199] {
        let (key, value) = &entries[idx];
        assert_eq!(reader.get(key)?, value.map(|v| v.to_vec()), "failed at index {idx}");
    }
    Ok(())
}
