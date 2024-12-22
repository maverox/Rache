use rache::{server::Server, storage::LSMTree};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lsm_tree = LSMTree::new("wal.log", "sstables", 3, 2)?;
    let server = Server::new(lsm_tree);

    server.run("127.0.0.1:6666").await;
    Ok(())
}