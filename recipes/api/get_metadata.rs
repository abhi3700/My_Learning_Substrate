//! Get metadata from a running substrate node

use subxt::runtime_api::{RuntimeApi, RuntimeApiClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace with your node's WebSocket endpoint
    let client = RuntimeApiClient::new()
        .set_url("wss://your-node-rpc-endpoint.com")
        .build()
        .await?;

    // Fetch and print out the metadata as a hex string
    let metadata = client.metadata().await?;
    println!("{}", hex::encode(metadata.to_bytes()));

    // Optionally, save metadata to a file
    std::fs::write("metadata.scale", metadata.to_bytes())?;

    Ok(())
}
