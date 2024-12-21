use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use super::BloomFilter;

/// SSTable operations
pub(super) struct SSTable {
    bloom_filter: BloomFilter,
    index: BTreeMap<String, u64>,
}

impl SSTable {
    /// Create a new SSTable with a Bloom filter
    pub fn new(keys: HashSet<String>, path: &Path) -> Result<Self, std::io::Error> {
        let mut bloom_filter = BloomFilter::new(1000);
        let mut index = BTreeMap::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut offset = 0;

        for line in reader.lines() {
            let line = line?;
            if let Some((key, _)) = line.split_once(':') {
                bloom_filter.insert(key);
                index.insert(key.to_string(), offset);
            }
            offset += line.len() as u64 + 1; // +1 for newline character
        }

        Ok(SSTable {
            bloom_filter,
            index,
        })
    }

    /// Check if a key might exist using the Bloom filter
    pub fn might_contain(&self, key: &str) -> bool {
        self.bloom_filter.might_contain(key)
    }

    /// Read a key from an SSTable
    pub fn read(path: &Path, key: &str) -> Result<Option<String>, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if let Some((k, v)) = line.split_once(':') {
                if k == key {
                    return Ok(Some(v.to_string()));
                }
            }
        }
        Ok(None)
    }

    /// Merge multiple SSTables into one
    pub fn merge(
        sstable_paths: &[&Path],
        output_path: &Path,
    ) -> Result<HashSet<String>, std::io::Error> {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);
        let mut key_set = HashSet::new();
        let mut entries: BTreeMap<String, String> = BTreeMap::new();

        // Read all SSTables
        for path in sstable_paths {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Some((key, value)) = line.split_once(':') {
                    entries.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Write merged entries to the new SSTable
        for (key, value) in entries {
            writeln!(writer, "{}:{}", key, value)?;
            key_set.insert(key);
        }
        writer.flush()?;
        Ok(key_set)
    }
}
