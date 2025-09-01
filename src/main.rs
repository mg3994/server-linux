use server::{config::Config, server::Server};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment
    let config = Config::from_env()?;

    // Create and run the server
    let server = Server::new(config)?;
    
    // Try to create development certificates if they don't exist
    if let Err(e) = server.create_dev_certificates().await {
        eprintln!("Warning: Could not create development certificates: {}", e);
        eprintln!("Please provide valid TLS certificates or install OpenSSL");
    }
    
    server.run().await?;

    Ok(())
}
