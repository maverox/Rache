# System Patterns: Rache Key-Value Store

## System Architecture

- **Client-Server:** The system follows a client-server architecture. Clients interact with the server to store and retrieve data.
- **LSM Tree:** The storage engine is based on the Log-Structured Merge Tree (LSM tree) architecture.

## Key Technical Decisions

- **Rust:** The project is implemented in Rust for performance, reliability, and memory safety.
- **LSM Tree:** The LSM tree was chosen for its efficient write performance.
- **Memtable:** An in-memory data structure (e.g., a B-Tree or Skip List) is used as the memtable.
- **SSTable:** Sorted String Tables (SSTables) are used for on-disk storage.
- **Write-Ahead Log (WAL):** A WAL is used to ensure data durability.
- **Bloom Filter:** Bloom filters are used to reduce disk lookups for non-existent keys.

## Design Patterns

- **Data Access Object (DAO):** The storage components (memtable, SSTable, WAL) can be seen as DAOs.
- **Command Pattern:** The client operations (put, get, delete) can be implemented using the command pattern.

## Component Relationships

- **Client:** Interacts with the server.
- **Server:** Handles client requests and interacts with the storage engine.
- **Memtable:** Stores data in memory.
- **SSTable:** Stores data on disk.
- **LSM Tree:** Manages the memtable and SSTable.
- **WAL:** Ensures data durability.
- **Bloom Filter:** Optimizes key existence checks.
