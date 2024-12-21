use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::sync::Mutex;


/// Write-Ahead Log (Wal)
pub struct Wal {
    path: String,
    file: Mutex<BufWriter<File>>,
}

impl Wal {
    /// Create a new Wal
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Wal {
            path: path.to_string(),
            file: Mutex::new(BufWriter::new(file)),
        })
    }

    /// Append a log entry
    pub fn append(&self, key: &str, value: &str) -> Result<(), std::io::Error> {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}:{}", key, value)?;
        file.flush()?;
        Ok(())
    }
}