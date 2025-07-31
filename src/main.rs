use jsonweblog::WebServer;
use tracing::{info, Level};
use tracing_subscriber;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Parse command line arguments
    let requested_port = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);

    // Find an available port starting from the requested port
    let actual_port = WebServer::find_available_port_for_new(requested_port).await?;
    
    /*
    if actual_port != requested_port {
        info!("Port {} was not available, using port {} instead", requested_port, actual_port);
    }
    */

    // Create and start the server
    let server = WebServer::new(actual_port);
    
    // Set up graceful shutdown
    tokio::select! {
        result = server.start() => {
            if let Err(e) = result {
                eprintln!("Server error: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully...");
        }
    }

    Ok(())
}
