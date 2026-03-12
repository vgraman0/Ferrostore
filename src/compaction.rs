pub struct CompactionManager {
    // TODO: fields needed:
    //   - thread_handle: Option<JoinHandle<()>> (the background compaction thread)
    //   - shutdown: Arc<AtomicBool> (signal to tell the thread to stop)
    //   - db_internals: Arc<...> (shared access to manifest, sstable list, db directory)
}

impl CompactionManager {
    pub fn new() -> Self {
        // TODO:
        //   1. Create the shutdown flag (Arc<AtomicBool>, initially false)
        //   2. Store references to shared DB state (manifest, sstable list, db path)
        //   3. Don't start the thread yet — that's what start() does
        todo!()
    }

    pub fn start(&self) {
        // TODO:
        //   1. Spawn a background thread that loops:
        //      a. Check shutdown flag — if true, break
        //      b. Check if there are enough L0 SSTables to trigger compaction
        //         (e.g. >= 4 L0 files)
        //      c. If so, pick two SSTables to merge:
        //         - Open both with SSTableReader
        //         - Merge-sort their entries into a new SSTable via SSTableBuilder
        //         - Drop duplicate keys (keep the newer version)
        //         - Drop tombstones if both versions are deleted
        //         - Update the manifest (add new SSTable, remove old two)
        //         - Delete the old .sst files
        //      d. Sleep for a short interval before checking again
        todo!()
    }

    pub fn stop(&self) {
        // TODO:
        //   1. Set the shutdown flag to true
        //   2. Join the background thread (wait for it to finish)
        todo!()
    }
}
