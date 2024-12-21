use std::hash::Hasher;
/// Bloom Filter
pub(super) struct BloomFilter {
    pub bit_array: Vec<bool>,
    pub size: usize,
}

impl BloomFilter {
    pub fn new(size: usize) -> Self {
        BloomFilter {
            bit_array: vec![false; size],
            size,
        }
    }

    pub fn hash(&self, key: &str, seed: u64) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(key.as_bytes());
        hasher.write_u64(seed);
        (hasher.finish() as usize) % self.size
    }

    pub fn insert(&mut self, key: &str) {
        for i in 0..3 {
            let index = self.hash(key, i);
            self.bit_array[index] = true;
        }
    }

    pub fn might_contain(&self, key: &str) -> bool {
        for i in 0..3 {
            let index = self.hash(key, i);
            if !self.bit_array[index] {
                return false;
            }
        }
        true
    }
}