# Project Brief: Rache Key-Value Store

## Core Requirements and Goals

- **Project Name:** Rache
- **Type:** Key-Value Store
- **Language:** Rust
- **Description:** A persistent key-value store with client and server components.
- **Key Features:**
    - Client-server architecture for remote data access.
    - Persistent storage using an LSM tree.
    - Memtable for in-memory data storage.
    - SSTable for on-disk data storage.
    - Write-Ahead Log (WAL) for data durability.
    - Bloom filter for efficient key existence checks.
- **Goals:**
    - Provide a reliable and efficient key-value storage solution.
    - Implement a robust storage engine based on the LSM tree architecture.
    - Offer a simple client API for interacting with the store.
    - Ensure data durability and consistency.
