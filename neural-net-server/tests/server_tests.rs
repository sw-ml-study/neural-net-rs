// Integration tests for neural-net-server
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_server_starts() {
    // Start server in background
    let handle = tokio::spawn(async {
        // This will fail until we implement the server
        neural_net_server::run_server("127.0.0.1:3001").await
    });

    // Give server time to start
    sleep(Duration::from_millis(100)).await;

    // Try to connect
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:3001/health")
        .send()
        .await;

    assert!(response.is_ok(), "Server should respond to health check");

    // Cleanup
    handle.abort();
}

#[tokio::test]
async fn test_health_endpoint() {
    let handle = tokio::spawn(async {
        neural_net_server::run_server("127.0.0.1:3002").await
    });

    sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:3002/health")
        .send()
        .await
        .expect("Should get response");

    assert!(response.status().is_success(), "Health check should return 200");

    let body: serde_json::Value = response.json().await.expect("Should parse JSON");
    assert_eq!(body["status"], "ok", "Health check should return ok status");

    handle.abort();
}
