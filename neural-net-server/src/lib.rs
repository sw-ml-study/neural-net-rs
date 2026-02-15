// Neural Network Server Library
// REST API server for neural network training and evaluation

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Json, sse::{Event, Sse}},
    routing::{get, post},
    Router,
};
use futures::stream::{self, Stream};
use std::convert::Infallible;
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
    /// Optional seed for reproducible training (random if not specified)
    seed: Option<u64>,
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

    // Create network (seeded if seed provided, random otherwise)
    let network = match req.seed {
        Some(seed) => Network::new_seeded(example.recommended_arch.clone(), SIGMOID, req.learning_rate, seed),
        None => Network::new(example.recommended_arch.clone(), SIGMOID, req.learning_rate),
    };

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

/// Train with SSE progress streaming
async fn train_stream(
    State(state): State<AppState>,
    Json(req): Json<TrainRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    // Get example
    let example = examples::get_example(&req.example)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                format!("Unknown example: {}", req.example),
            )
        })?;

    // Create channel for progress updates (use std mpsc for Send compatibility)
    let (tx, rx) = std::sync::mpsc::channel::<(u32, f64)>();

    // Spawn blocking training task
    let example_name = req.example.clone();
    let epochs = req.epochs;
    let learning_rate = req.learning_rate;
    let seed = req.seed;
    let state_clone = state.clone();
    let inputs = example.inputs.clone();
    let targets = example.targets.clone();
    let arch = example.recommended_arch.clone();

    tokio::task::spawn_blocking(move || {
        // Create network (seeded if seed provided, random otherwise)
        let network = match seed {
            Some(s) => Network::new_seeded(arch, SIGMOID, learning_rate, s),
            None => Network::new(arch, SIGMOID, learning_rate),
        };

        // Create training config
        let config = TrainingConfig {
            epochs,
            checkpoint_interval: None,
            checkpoint_path: None,
            verbose: false,
            example_name: Some(example_name.clone()),
        };

        let mut controller = TrainingController::new(network, config);

        // Add callback to send progress
        let tx_clone = tx.clone();
        controller.add_callback(Box::new(move |epoch, loss, _network| {
            let _ = tx_clone.send((epoch, loss));
        }));

        // Train the network
        if let Ok(()) = controller.train(inputs, targets) {
            // Store model after training
            let model_id = Uuid::new_v4().to_string();
            let stored_model = StoredModel {
                network: controller.into_network(),
                example: example_name,
                epochs,
                learning_rate,
            };
            state_clone
                .models
                .lock()
                .unwrap()
                .insert(model_id, stored_model);
        }
    });

    // Create SSE stream from std mpsc receiver
    let stream = stream::unfold(rx, |rx| async move {
        // Convert std::sync::mpsc to async stream
        match rx.try_recv() {
            Ok((epoch, loss)) => {
                let data = serde_json::json!({
                    "epoch": epoch,
                    "loss": loss
                });
                Some((
                    Ok::<_, Infallible>(Event::default().data(data.to_string())),
                    rx
                ))
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // Wait a bit and try again
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                Some((
                    Ok::<_, Infallible>(Event::default().comment("heartbeat")),
                    rx
                ))
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => None,
        }
    });

    Ok(Sse::new(stream))
}

/// Run the web server on the specified address
pub async fn run_server(addr: &str) -> Result<(), anyhow::Error> {
    use tower_http::services::ServeDir;
    use tower_http::cors::CorsLayer;

    let state = AppState::new();

    // API routes
    let api_routes = Router::new()
        .route("/health", get(health))
        .route("/api/examples", get(list_examples))
        .route("/api/train", post(train))
        .route("/api/train/stream", post(train_stream))
        .route("/api/eval", post(eval))
        .route("/api/models/:id", get(model_info))
        .with_state(state);

    // Static file serving for future web UI
    let app = api_routes
        .nest_service("/", ServeDir::new("static").fallback(ServeDir::new("static/index.html")))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);
    println!("API endpoints available at /api/*");
    println!("Static files served from ./static/");

    axum::serve(listener, app).await?;

    Ok(())
}
