use rusty_lsm::bloom::BloomFilter;

#[test]
fn inserted_key_is_found() {
    let mut bf = BloomFilter::new(100, 0.01);
    bf.insert(b"hello");

    assert!(bf.may_contain(b"hello"));
}

#[test]
fn missing_key_is_not_found() {
    let mut bf = BloomFilter::new(100, 0.01);
    bf.insert(b"hello");

    assert!(!bf.may_contain(b"world"));
}

#[test]
fn multiple_inserts_all_found() {
    let mut bf = BloomFilter::new(100, 0.01);
    let keys: Vec<&[u8]> = vec![b"apple", b"banana", b"cherry", b"date", b"elderberry"];

    for key in &keys {
        bf.insert(key);
    }

    for key in &keys {
        assert!(bf.may_contain(key), "expected to find {:?}", key);
    }
}

#[test]
fn empty_filter_finds_nothing() {
    let bf = BloomFilter::new(100, 0.01);

    assert!(!bf.may_contain(b"anything"));
}

#[test]
fn encode_decode_roundtrip() {
    let mut bf = BloomFilter::new(100, 0.01);
    bf.insert(b"foo");
    bf.insert(b"bar");

    let encoded = bf.encode();
    let decoded = BloomFilter::decode(&encoded).unwrap();

    assert!(decoded.may_contain(b"foo"));
    assert!(decoded.may_contain(b"bar"));
    assert!(!decoded.may_contain(b"baz"));
}

#[test]
fn false_positive_rate_is_reasonable() {
    let n = 1000;
    let mut bf = BloomFilter::new(n, 0.01);

    for i in 0..n {
        bf.insert(format!("key-{}", i).as_bytes());
    }

    // All inserted keys must be found
    for i in 0..n {
        assert!(bf.may_contain(format!("key-{}", i).as_bytes()));
    }

    // Check false positive rate with keys that were never inserted
    let mut false_positives = 0;
    let test_count = 10000;
    for i in 0..test_count {
        if bf.may_contain(format!("missing-{}", i).as_bytes()) {
            false_positives += 1;
        }
    }

    let fp_rate = false_positives as f64 / test_count as f64;
    assert!(fp_rate < 0.05, "false positive rate too high: {:.3}", fp_rate);
}
