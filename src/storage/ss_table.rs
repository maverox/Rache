use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

use super::BloomFilter;

/// SSTable operations
pub(super) struct SSTable {
    pub(crate) bloom_filter: BloomFilter,
    index: BTreeMap<String, u64>,
}

impl SSTable {
    /// Create a new SSTable with a Bloom filter
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let mut bloom_filter = BloomFilter::new(1000);
        let mut index = BTreeMap::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut offset = 0;
        let mut index_file = BufWriter::new(File::create(path.with_extension("index"))?);

        for line in reader.lines() {
            let line = line?;
            if let Some((key, _)) = line.split_once(':') {
                bloom_filter.insert(key);
                index.insert(key.to_string(), offset);
                index_file.write_all(format!("{}:{}\n", key, offset).as_bytes())?;
            }
            offset += line.len() as u64 + 1; // +1 for newline character
        }
        index_file.flush()?;

        Ok(SSTable {
            bloom_filter,
            index,
        })
    }

    /// Load an existing SSTable and its Bloom filter
    pub(crate) fn load(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut bloom_filter = BloomFilter::new(1000);
        let mut index = BTreeMap::new();
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
    pub fn read(&self, path: &Path, key: &str) -> Result<Option<String>, std::io::Error> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        if let Some(&offset) = self.index.get(key) {
            reader.seek(SeekFrom::Start(offset))?;
            let mut line = String::new();
            reader.read_line(&mut line)?;
            if let Some((_, v)) = line.split_once(':') {
                return Ok(Some(v.to_string()));
            }
        }
        Ok(None)
    }

    /// Merge multiple SSTables into one
    pub fn merge(
        sstable_paths: &[&Path],
        output_path: &Path,
    ) -> Result<(), std::io::Error> {
        let file = File::create(output_path)?;
        let index_file = File::create(output_path.with_extension("index"))?;
        let mut writer = BufWriter::new(file);
        let mut index_writer = BufWriter::new(index_file);
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
        let mut offset = 0;
        for (key, value) in entries {
            let ss_table_line = format!("{}:{}\n", key, value);
            let index_line = format!("{}:{}\n", key, offset);
            writer.write_all(ss_table_line.as_bytes())?;
            index_writer.write_all(index_line.as_bytes())?;
            
            offset += ss_table_line.len() as u64;
        }
        writer.flush()?;
        index_writer.flush()?;

        Ok(())
    }
}
