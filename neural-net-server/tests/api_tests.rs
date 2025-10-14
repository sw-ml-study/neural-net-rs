// Integration tests for REST API endpoints
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

async fn start_test_server(port: u16) -> tokio::task::JoinHandle<Result<(), anyhow::Error>> {
    let addr = format!("127.0.0.1:{}", port);
    tokio::spawn(async move {
        neural_net_server::run_server(&addr).await
    })
}

#[tokio::test]
async fn test_list_examples() {
    let handle = start_test_server(3010).await;
    sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:3010/api/examples")
        .send()
        .await
        .expect("Should get response");

    assert!(response.status().is_success());

    let body: serde_json::Value = response.json().await.expect("Should parse JSON");
    assert!(body.is_array(), "Should return array of examples");
    assert!(body.as_array().unwrap().len() > 0, "Should have at least one example");

    handle.abort();
}

#[tokio::test]
async fn test_train_endpoint() {
    let handle = start_test_server(3011).await;
    sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let request_body = json!({
        "example": "and",
        "epochs": 100,
        "learning_rate": 0.5
    });

    let response = client
        .post("http://127.0.0.1:3011/api/train")
        .json(&request_body)
        .send()
        .await
        .expect("Should get response");

    assert!(response.status().is_success());

    let body: serde_json::Value = response.json().await.expect("Should parse JSON");
    assert!(body["model_id"].is_string(), "Should return model_id");

    handle.abort();
}

#[tokio::test]
async fn test_eval_endpoint() {
    let handle = start_test_server(3012).await;
    sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();

    // First, train a model
    let train_body = json!({
        "example": "and",
        "epochs": 100,
        "learning_rate": 0.5
    });

    let train_response = client
        .post("http://127.0.0.1:3012/api/train")
        .json(&train_body)
        .send()
        .await
        .expect("Should train model");

    let train_result: serde_json::Value = train_response.json().await.unwrap();
    let model_id = train_result["model_id"].as_str().unwrap();

    // Now evaluate it
    let eval_body = json!({
        "model_id": model_id,
        "input": [0.0, 0.0]
    });

    let response = client
        .post("http://127.0.0.1:3012/api/eval")
        .json(&eval_body)
        .send()
        .await
        .expect("Should get response");

    assert!(response.status().is_success());

    let body: serde_json::Value = response.json().await.expect("Should parse JSON");
    assert!(body["output"].is_array(), "Should return output array");

    handle.abort();
}

#[tokio::test]
async fn test_model_info_endpoint() {
    let handle = start_test_server(3013).await;
    sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();

    // Train a model first
    let train_body = json!({
        "example": "xor",
        "epochs": 100,
        "learning_rate": 0.5
    });

    let train_response = client
        .post("http://127.0.0.1:3013/api/train")
        .json(&train_body)
        .send()
        .await
        .expect("Should train model");

    let train_result: serde_json::Value = train_response.json().await.unwrap();
    let model_id = train_result["model_id"].as_str().unwrap();

    // Get model info
    let response = client
        .get(&format!("http://127.0.0.1:3013/api/models/{}", model_id))
        .send()
        .await
        .expect("Should get response");

    assert!(response.status().is_success());

    let body: serde_json::Value = response.json().await.expect("Should parse JSON");
    assert!(body["architecture"].is_array(), "Should return architecture");
    assert!(body["example"].is_string(), "Should return example name");

    handle.abort();
}

#[tokio::test]
async fn test_train_invalid_example() {
    let handle = start_test_server(3014).await;
    sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let request_body = json!({
        "example": "invalid_example",
        "epochs": 100,
        "learning_rate": 0.5
    });

    let response = client
        .post("http://127.0.0.1:3014/api/train")
        .json(&request_body)
        .send()
        .await
        .expect("Should get response");

    assert!(
        response.status().is_client_error(),
        "Should return 4xx for invalid example"
    );

    handle.abort();
}

#[tokio::test]
async fn test_eval_nonexistent_model() {
    let handle = start_test_server(3015).await;
    sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let eval_body = json!({
        "model_id": "nonexistent-model-id",
        "input": [0.0, 0.0]
    });

    let response = client
        .post("http://127.0.0.1:3015/api/eval")
        .json(&eval_body)
        .send()
        .await
        .expect("Should get response");

    assert!(
        response.status().is_client_error(),
        "Should return 4xx for nonexistent model"
    );

    handle.abort();
}
