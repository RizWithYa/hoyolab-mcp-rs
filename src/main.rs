pub mod client;
pub mod config;
pub mod ds;
pub mod format;
pub mod mcp;

use client::HoYoLabClient;
use config::AppConfig;
use mcp::McpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut config_path = None;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--config" && i + 1 < args.len() {
            config_path = Some(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }

    let config = AppConfig::load_from_file_or_env(config_path.as_deref());
    let client = HoYoLabClient::new(config);
    let server = McpServer::new(client);

    server.run_stdio().await?;
    Ok(())
}
