use std::{collections::BTreeMap, fs::File, io::{BufRead, BufWriter, Write}, path::Path, sync::RwLock};

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
    
    /// Check and load from the Wal
    pub fn load_from_wal(&self, wal: &Path) -> Result<(), std::io::Error> {
        let file = File::open(wal)?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if let Some((key, value)) = line.split_once(':') {
                self.insert(key.to_string(), value.to_string());
            }
        }
        Ok(())
    }

    /// Insert a key-value pair
    pub fn insert(&self, key: String, value: String) {
        let mut map = self.map.write().unwrap();
        if value.is_empty() {
            map.remove(&key);
        } else {
            map.insert(key, value);
        }
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
    pub fn flush_to_sstable(&self, path: &Path) -> Result<(), std::io::Error> {
        let map = self.map.read().unwrap();
        let ss_table_file = File::create(path)?;
        let mut ss_table_writer = BufWriter::new(ss_table_file);
        let index_file = File::create(path.with_extension("index"))?;
        let mut index_writer = BufWriter::new(index_file);
        let mut offset = 0;

        for (key, value) in map.iter() {
            let ss_table_line = format!("{}:{}\n", key, value);
            let index_line = format!("{}:{}\n", key, offset);
            ss_table_writer.write_all(ss_table_line.as_bytes())?;
            index_writer.write_all(index_line.as_bytes())?;
            offset += ss_table_line.len() as u64;
        }
        ss_table_writer.flush()?;
        index_writer.flush()?;
        
        Ok(())
    }
}