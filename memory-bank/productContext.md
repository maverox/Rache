# Product Context: Rache Key-Value Store

## Why This Project Exists

- **Problems Solved:**
    - Provides a simple and efficient way to store and retrieve data.
    - Offers persistent storage, ensuring data is not lost when the server restarts.
    - Enables remote access to data through a client-server architecture.
- **How It Should Work:**
    - Clients connect to the server and can perform basic operations like:
        - `put(key, value)`: Stores a key-value pair.
        - `get(key)`: Retrieves the value associated with a key.
        - `delete(key)`: Removes a key-value pair.
    - The server stores data in memory (memtable) and periodically flushes it to disk (SSTable) using an LSM tree structure.
    - A Write-Ahead Log (WAL) ensures data durability in case of crashes.
- **User Experience Goals:**
    - Simple and easy-to-use client API.
    - Fast data access and retrieval.
    - Reliable and consistent data storage.
