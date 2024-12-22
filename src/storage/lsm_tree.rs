use std::{fs, path::Path, sync::Arc};
use super::{Wal, MemTable, SSTable};
pub struct LSMTree {
    wal: Arc<Wal>,
    memtable: Arc<MemTable>,
    sstable_dir: String,
    sstable_counter: usize,
    sstables: Vec<SSTable>,
    compaction_threshold: usize,
}

impl LSMTree {
    /// Create a new LSM Tree
    pub fn new(
        wal_path: &str,
        sstable_dir: &str,
        memtable_max_size: usize,
        compaction_threshold: usize,
    ) -> Result<Self, std::io::Error> {
        fs::create_dir_all(sstable_dir)?;
        Ok(LSMTree {
            wal: Arc::new(Wal::new(wal_path)?),
            memtable: Arc::new(MemTable::new(memtable_max_size)),
            sstable_dir: sstable_dir.to_string(),
            sstable_counter: 0,
            sstables: Vec::new(),
            compaction_threshold,
        })
    }

    /// Write a key-value pair
    pub fn write(&mut self, key: String, value: String) -> Result<(), std::io::Error> {
        // Append to Wal
        self.wal.append(&key, &value)?;
        // Insert into MemTable
        self.memtable.insert(key, value);

        // Flush MemTable to SSTable if full
        if self.memtable.is_full() {
            let sstable_path =
                Path::new(&self.sstable_dir).join(format!("sstable_{}.txt", self.sstable_counter));
            let keys = self.memtable.flush_to_sstable(&sstable_path)?;
            let sstable = SSTable::new(keys, &sstable_path)?;
            self.sstables.push(sstable);
            self.sstable_counter += 1;
            self.memtable = Arc::new(MemTable::new(self.memtable.max_size)); // Reset MemTable

            // Reset Wal 
            self.wal.reset()?;
            // Trigger compaction if too many SSTables
            if self.sstables.len() > self.compaction_threshold {
                self.compact()?;
            }
        }
        Ok(())
    }

    /// Read a key-value pair
    pub fn read(&self, key: &str) -> Result<Option<String>, std::io::Error> {
        // Check MemTable
        if let Some(value) = self.memtable.get(key) {
            return Ok(Some(value));
        }

        // Check SSTables (from newest to oldest)
        for (i, sstable) in self.sstables.iter().enumerate().rev() {
            if sstable.might_contain(key) {
                let sstable_path = Path::new(&self.sstable_dir).join(format!("sstable_{}.txt", i));
                if let Some(value) = SSTable::read(&sstable_path, key)? {
                    return Ok(Some(value));
                }
            }
        }
        Ok(None)
    }

    /// Compact SSTables
    fn compact(&mut self) -> Result<(), std::io::Error> {
        let num_to_compact = self.compaction_threshold; // Compact the oldest two SSTables
        let mut sstable_paths = Vec::new();

        for i in 0..num_to_compact {
            let path = Path::new(&self.sstable_dir).join(format!("sstable_{}.txt", i));
            sstable_paths.push(path);
        }

        let output_path =
            Path::new(&self.sstable_dir).join(format!("sstable_{}.txt", self.sstable_counter));
        let keys = SSTable::merge(
            &sstable_paths
                .iter()
                .map(|p| p.as_path())
                .collect::<Vec<&Path>>(),
            &output_path,
        )?;

        // Replace old SSTables with the compacted one
        self.sstables.drain(0..num_to_compact);
        self.sstables.push(SSTable::new(keys, &output_path)?);
        self.sstable_counter += 1;

        // Remove old SSTable files
        for path in sstable_paths {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}