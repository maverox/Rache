# `Rache` an LSM Tree based in memory kv store implementation

This project is an implementation of a Log-Structured Merge (LSM) Tree in Rust. It includes components such as SSTables, Bloom filters, Write-Ahead Log (WAL), Memtable, and compaction strategies.

## Features

- **SSTable**: Immutable data structure for storing key-value pairs.
- **Bloom Filter**: Probabilistic data structure for fast membership testing.
- **Write-Ahead Log (WAL)**: Ensures durability by logging changes before they are applied.
- **Memtable**: In-memory data structure for fast writes.
- **Compaction**: Merging and reorganizing SSTables to optimize read performance.

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/maverox/Rache.git
    cd Rache
    ```

2. Build the project:
    ```sh
    cargo build
    ```

## Binaries

### `rache`

`rache` is the main service that runs the LSM Tree implementation. It handles all the core functionalities such as managing SSTables, Bloom filters, WAL, and Memtable.

### `client`

`client` is a REPL (Read-Eval-Print Loop) CLI that allows users to interact with the `rache` service. It provides commands to perform operations such as inserting, deleting, and querying key-value pairs.

To run the `rache` service:
```sh
cargo run --bin rache
```

To run the `client` CLI:
```sh
cargo run --bin client
```

### REPL Commands


``` 
   $ write <key> <value>
```

```
   $ read <key>
```

``` 
   $ delete <key> 
```