use std::{collections::{BTreeMap, HashSet}, fs::File, io::{BufWriter, Write}, path::Path, sync::RwLock};

/// MemTable (in-memory store)
pub(super) struct MemTable {
    pub map: RwLock<BTreeMap<String, String>>,
    pub max_size: usize,
}

impl MemTable {
    /// Create a new MemTable
    pub fn new(max_size: usize) -> Self {
        MemTable {
            map: RwLock::new(BTreeMap::new()),
            max_size,
        }
    }

    /// Insert a key-value pair
    pub fn insert(&self, key: String, value: String) {
        let mut map = self.map.write().unwrap();
        map.insert(key, value);
    }

    /// Get a value by key
    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.map.read().unwrap();
        map.get(key).cloned()
    }

    /// Check if the MemTable is full
    pub fn is_full(&self) -> bool {
        let map = self.map.read().unwrap();
        map.len() >= self.max_size
    }

    /// Flush MemTable to an SSTable
    pub fn flush_to_sstable(&self, path: &Path) -> Result<HashSet<String>, std::io::Error> {
        let map = self.map.read().unwrap();
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let mut keys = HashSet::new();
        for (key, value) in map.iter() {
            writeln!(writer, "{}:{}", key, value)?;
            keys.insert(key.clone());
        }
        writer.flush()?;
        Ok(keys)
    }
}