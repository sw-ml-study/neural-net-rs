// Neural Network Server - Main entry point
// This will be implemented after tests are written (TDD)

#[tokio::main]
async fn main() {
    println!("Neural Network Server");
    println!("Starting server...");

    if let Err(e) = neural_net_server::run_server("127.0.0.1:3000").await {
        eprintln!("Server error: {}", e);
    }
}
