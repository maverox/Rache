use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
    sync::Mutex,
};

/// Write-Ahead Log (Wal)
pub struct Wal {
    file: Mutex<BufWriter<File>>,
}

impl Wal {
    /// Create a new Wal
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        Ok(Wal {
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

    /// reset the wal
    pub fn reset(&self) -> Result<(), std::io::Error> {
        let mut file = self.file.lock().unwrap();
        file.get_mut().set_len(0)?;
        file.flush()?;
        Ok(())
    }
}
