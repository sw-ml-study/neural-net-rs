// Neural Network Server Library
// REST API server for neural network training and evaluation

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use neural_network::{
    activations::SIGMOID,
    examples,
    network::Network,
    training::{TrainingConfig, TrainingController},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    models: Arc<Mutex<HashMap<String, StoredModel>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            models: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

/// Stored model with metadata
#[derive(Clone)]
struct StoredModel {
    network: Network,
    example: String,
    epochs: u32,
    learning_rate: f64,
}

/// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

/// Example list response
#[derive(Serialize)]
struct ExampleInfo {
    name: String,
    description: String,
    architecture: Vec<usize>,
}

/// Train request
#[derive(Deserialize)]
struct TrainRequest {
    example: String,
    epochs: u32,
    learning_rate: f64,
}

/// Train response
#[derive(Serialize)]
struct TrainResponse {
    model_id: String,
    example: String,
    epochs: u32,
}

/// Eval request
#[derive(Deserialize)]
struct EvalRequest {
    model_id: String,
    input: Vec<f64>,
}

/// Eval response
#[derive(Serialize)]
struct EvalResponse {
    output: Vec<f64>,
}

/// Model info response
#[derive(Serialize)]
struct ModelInfoResponse {
    model_id: String,
    example: String,
    architecture: Vec<usize>,
    epochs: u32,
    learning_rate: f64,
    total_parameters: usize,
}

/// Health check endpoint
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

/// List available examples
async fn list_examples() -> Json<Vec<ExampleInfo>> {
    let example_names = examples::list_examples();
    let examples_info: Vec<ExampleInfo> = example_names
        .into_iter()
        .filter_map(|name| {
            examples::get_example(name).map(|ex| ExampleInfo {
                name: ex.name.to_string(),
                description: ex.description.to_string(),
                architecture: ex.recommended_arch.clone(),
            })
        })
        .collect();

    Json(examples_info)
}

/// Train a new model
async fn train(
    State(state): State<AppState>,
    Json(req): Json<TrainRequest>,
) -> Result<Json<TrainResponse>, (StatusCode, String)> {
    // Get example
    let example = examples::get_example(&req.example)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                format!("Unknown example: {}", req.example),
            )
        })?;

    // Create network
    let network = Network::new(example.recommended_arch.clone(), SIGMOID, req.learning_rate);

    // Create training config
    let config = TrainingConfig {
        epochs: req.epochs,
        checkpoint_interval: None,
        checkpoint_path: None,
        verbose: false,
        example_name: Some(example.name.to_string()),
    };

    // Train
    let mut controller = TrainingController::new(network, config);
    controller
        .train(example.inputs.clone(), example.targets.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Store model
    let model_id = Uuid::new_v4().to_string();
    let stored_model = StoredModel {
        network: controller.into_network(),
        example: req.example.clone(),
        epochs: req.epochs,
        learning_rate: req.learning_rate,
    };

    state
        .models
        .lock()
        .unwrap()
        .insert(model_id.clone(), stored_model);

    Ok(Json(TrainResponse {
        model_id,
        example: req.example,
        epochs: req.epochs,
    }))
}

/// Evaluate a model
async fn eval(
    State(state): State<AppState>,
    Json(req): Json<EvalRequest>,
) -> Result<Json<EvalResponse>, (StatusCode, String)> {
    // Get model
    let models = state.models.lock().unwrap();
    let stored_model = models
        .get(&req.model_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Model not found".to_string()))?;

    // Clone network for evaluation
    let mut network = stored_model.network.clone();

    // Validate input dimensions
    if req.input.len() != network.layers[0] {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Invalid input dimensions: expected {}, got {}",
                network.layers[0],
                req.input.len()
            ),
        ));
    }

    // Run prediction
    let input_matrix = neural_network::matrix::Matrix::from(req.input);
    let output = network.feed_forward(input_matrix);

    Ok(Json(EvalResponse {
        output: output.data,
    }))
}

/// Get model information
async fn model_info(
    State(state): State<AppState>,
    Path(model_id): Path<String>,
) -> Result<Json<ModelInfoResponse>, (StatusCode, String)> {
    let models = state.models.lock().unwrap();
    let stored_model = models
        .get(&model_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Model not found".to_string()))?;

    // Calculate total parameters
    let mut total_params = 0;
    for weight in &stored_model.network.weights {
        total_params += weight.rows * weight.cols;
    }
    for bias in &stored_model.network.biases {
        total_params += bias.rows;
    }

    Ok(Json(ModelInfoResponse {
        model_id,
        example: stored_model.example.clone(),
        architecture: stored_model.network.layers.clone(),
        epochs: stored_model.epochs,
        learning_rate: stored_model.learning_rate,
        total_parameters: total_params,
    }))
}

/// Run the web server on the specified address
pub async fn run_server(addr: &str) -> Result<(), anyhow::Error> {
    let state = AppState::new();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/examples", get(list_examples))
        .route("/api/train", post(train))
        .route("/api/eval", post(eval))
        .route("/api/models/:id", get(model_info))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
