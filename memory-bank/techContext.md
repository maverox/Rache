# Technical Context: Rache Key-Value Store

## Technologies Used

- **Language:** Rust
- **Storage Engine:** LSM Tree
- **Data Structures:**
    - Memtable: (e.g., B-Tree, Skip List)
    - SSTable
    - Bloom Filter
- **Concurrency:** Tokio (or similar async runtime)

## Development Setup

- **Rust Toolchain:** Requires a Rust installation (rustc, cargo).
- **Dependencies:** Defined in `Cargo.toml`.
- **Build:** `cargo build`
- **Run:** `cargo run --bin rache` (for the server) and `cargo run --bin client` (for the client)

## Technical Constraints

- **Memory Management:** Rust's ownership and borrowing system helps prevent memory leaks and data races.
- **Performance:** The LSM tree architecture is optimized for write performance, but read performance can be affected by the number of SSTables.
- **Concurrency:** Managing concurrent access to the memtable and SSTables requires careful synchronization.

## Dependencies

- The dependencies are listed in the `Cargo.toml` file. Common dependencies might include:
    - `tokio` (for async runtime)
    - `bytes` (for byte manipulation)
    - `serde` (for serialization/deserialization)
    - `log` and `env_logger` (for logging)
