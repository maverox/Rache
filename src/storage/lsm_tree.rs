use super::{SSTable, Wal};
use crate::common_enums::CompactionStrategy;
use crate::storage::mem_table::MemTable;
use log::{info, warn};
use std::{fs, path::Path, sync::Arc, vec}; // Add logging

#[cfg(feature = "size_tiered")]
const DEFAULT_COMPACTION_STRATEGY: CompactionStrategy = CompactionStrategy::SizeTiered;

#[cfg(feature = "level_based")]
const DEFAULT_COMPACTION_STRATEGY: CompactionStrategy = CompactionStrategy::LevelBased;

pub struct LSMTree {
    wal: Arc<Wal>,
    memtable: Arc<MemTable>,
    sstable_dir: String,
    levels: Vec<Vec<SSTable>>,
    compaction_threshold: usize,
    compaction_strategy: CompactionStrategy,
}

impl LSMTree {
    /// Create a new LSM Tree
    pub fn new(
        wal_path: &str,
        sstable_dir: &str,
        memtable_max_size: usize,
        compaction_threshold: usize,
    ) -> Result<Self, std::io::Error> {
        info!(
            "Creating new LSMTree with wal_path: {}, sstable_dir: {}",
            wal_path, sstable_dir
        );
        fs::create_dir_all(sstable_dir)?;
        let levels = Vec::from(vec![Vec::new()]);
        let wal_path = Path::new(wal_path);
        let wal = Arc::new(Wal::new(wal_path)?);
        let memtable = Arc::new(MemTable::new(memtable_max_size));

        memtable.load_from_wal(wal_path)?;

        let mut lsm_tree = LSMTree {
            wal,
            memtable,
            sstable_dir: sstable_dir.to_string(),
            levels,
            compaction_threshold,
            compaction_strategy: DEFAULT_COMPACTION_STRATEGY,
        };

        lsm_tree.load_levels()?;
        Ok(lsm_tree)
    }

    /// Write a key-value pair
    pub fn write(&mut self, key: String, value: String) -> Result<(), std::io::Error> {
        info!("Writing key: {}, value: {}", key, value);
        // Append to Wal
        self.wal.append(&key, &value)?;
        // Insert into MemTable
        self.memtable.insert(key.clone(), value);

        // Flush MemTable to SSTable if full
        if self.memtable.is_full() {
            warn!("MemTable is full, flushing to SSTable");
            let sstable_path = Path::new(&self.sstable_dir).join(format!("sstable_{}_{}.txt", 0, self.levels.get(0).map_or(0, |v| v.len()))); 

            self.memtable.flush_to_sstable(&sstable_path)?;
            let sstable = SSTable::new(&sstable_path)?;
            if let Some(level) = self.levels.get_mut(0) {
                level.push(sstable);
            } else {
                self.levels.push(vec![sstable]);
            }

            self.memtable = Arc::new(MemTable::new(self.memtable.max_size)); // Reset MemTable

            // Reset Wal
            self.wal.reset()?;

            // Trigger compaction if too many SSTables
            if self.levels[0].len() >= self.compaction_threshold {
                warn!("Compaction triggered");
                self.compact()?;
            }
        }
        Ok(())
    }

    /// Read a key-value pair
    pub fn read(&self, key: &str) -> Result<Option<String>, std::io::Error> {
        info!("Reading key: {}", key);
        // Check MemTable
        if let Some(value) = self.memtable.get(key) {
            info!("Key: {} found in MemTable", key);
            return Ok(Some(value));
        }

        // Check SSTables (from newest to oldest)
        for (level_index, level) in self.levels.iter().enumerate() {
            for (sstable_index, sstable) in level.iter().enumerate() {
                let path = Path::new(&self.sstable_dir).join(format!("sstable_{}_{}.txt", level_index, sstable_index));

                if !sstable.might_contain(key) {
                    continue;
                }
                if let Some(value) = sstable.read(&path, key)? {
                    info!("Key: {} found in SSTable {:?}", key, &path);
                    return Ok(Some(value));
                }
            }
        }
        warn!("Key: {} not found", key);
        Ok(None)
    }

    /// load levels of SSTables (Can be improved significantly)
    fn load_levels(&mut self) -> Result<(), std::io::Error> {
        info!("Loading levels...");
        let mut levels = Vec::new();
        let mut i = 0;
        loop {
            let mut level = Vec::new();
            for j in 0..self.compaction_threshold {
                let path = Path::new(&self.sstable_dir).join(format!("sstable_{}_{}.txt", i, j));
                if !path.exists() {
                    break;
                }
                let sstable = SSTable::load(&path)?;
                level.push(sstable);
                i += 1;
            }
            if level.is_empty() {
                warn!("No SSTables found for level {}", i);
                break;
            }
            levels.push(level);
        }
        self.levels = levels;
        Ok(())
    }

    fn compact(&mut self) -> Result<(), std::io::Error> {
        match self.compaction_strategy {
            CompactionStrategy::SizeTiered => self.compact_size_tiered(),
            CompactionStrategy::LevelBased => self.compact_level_based(),
        }
    }

    fn compact_level_based(&mut self) -> Result<(), std::io::Error> {
        info!("Starting compaction");

        fn compact(lsm_tree: &mut LSMTree, level: usize) -> Result<(), std::io::Error> {
            if level >= lsm_tree.levels.len() {
                lsm_tree.levels.push(Vec::new());
            }

            if lsm_tree.levels[level].len() >= lsm_tree.compaction_threshold {
                let mut sstable_paths = Vec::new();
                for sstable_index in 0..lsm_tree.levels[level].len() {
                    // Collect SSTable paths for merging
                    sstable_paths.push(
                        Path::new(&lsm_tree.sstable_dir)
                            .join(format!("sstable_{}_{}.txt", level, sstable_index)),
                    );
                }

                let new_level = level + 1;
                let output_index = lsm_tree.levels.get(new_level).map_or(0, |v| v.len());
                let output_path = Path::new(&lsm_tree.sstable_dir)
                    .join(format!("sstable_{}_{}.txt", new_level, output_index));

                SSTable::merge(
                    &sstable_paths
                        .iter()
                        .map(|p| p.as_path())
                        .collect::<Vec<&Path>>(),
                    &output_path,
                )?;

                lsm_tree.levels[level].clear();
                if let Some(level) = lsm_tree.levels.get_mut(new_level) {
                    level.push(SSTable::load(&output_path)?);
                } else {
                    lsm_tree.levels.push(vec![SSTable::load(&output_path)?]);
                }

                // Recursively compact the next level if needed
                compact(lsm_tree, new_level)?;
            }

            Ok(())
        }

        for level in 0..self.levels.len() {
            compact(self, level)?;
        }

        Ok(())
    }

    fn compact_size_tiered(&mut self) -> Result<(), std::io::Error> {
        info!("Starting compaction");
        todo!()
    }
}
