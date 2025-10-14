// Neural Network Server - Main entry point
// REST API server with CLI argument parsing

use clap::Parser;

/// Neural Network REST API Server
///
/// Provides REST API endpoints for training and evaluating neural networks,
/// with Server-Sent Events (SSE) for real-time training progress.
#[derive(Parser, Debug)]
#[command(name = "neural-net-server")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host address to bind to (e.g., 0.0.0.0 for all interfaces, 127.0.0.1 for localhost)
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    /// Port number to listen on
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);

    println!("Neural Network Server");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Starting server on http://{}...", addr);
    println!();
    println!("Endpoints:");
    println!("  - Web UI:          http://{}/", addr);
    println!("  - Health:          http://{}/health", addr);
    println!("  - API Examples:    http://{}/api/examples", addr);
    println!("  - Train (sync):    POST http://{}/api/train", addr);
    println!("  - Train (stream):  POST http://{}/api/train/stream", addr);
    println!("  - Evaluate:        POST http://{}/api/eval", addr);
    println!("  - Model Info:      GET  http://{}/api/models/:id", addr);
    println!();

    if let Err(e) = neural_net_server::run_server(&addr).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
