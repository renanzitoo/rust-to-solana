use std::fs;
use serde::{Serialize, Deserialize};
use rand::Rng;
use axum::{
    routing::{get, post, delete},
    http::StatusCode,
    Router, Json, extract::{Path, State},
};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::sync::Arc;

const FILE_PATH: &str = "urls.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Url {
    short: String,
    long: String,
    user: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUrlRequest {
    long: String,
    user: String,
}

#[derive(Serialize, Deserialize)]
struct UrlResponse {
    short: String,
    long: String,
}

type AppState = Arc<Mutex<Vec<Url>>>;

fn load_urls() -> Vec<Url> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn generate_short_url() -> String {
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            charset[idx] as char
        })
        .collect()
}

async fn save_to_file(urls: &[Url]) {
    if let Ok(json) = serde_json::to_string(urls) {
        let _ = tokio::fs::write(FILE_PATH, json).await;
    }
}

#[axum::debug_handler]
async fn create_short_url(
    State(state): State<AppState>,
    Json(req): Json<CreateUrlRequest>,
) -> (StatusCode, Json<UrlResponse>) {
    let short = generate_short_url();
    let url = Url {
        short: short.clone(),
        long: req.long.clone(),
        user: req.user,
    };

    let mut urls = state.lock().await;
    urls.push(url);
    let urls_data = urls.clone();
    drop(urls);
    save_to_file(&urls_data).await;

    (
        StatusCode::CREATED,
        Json(UrlResponse {
            short,
            long: req.long,
        }),
    )
}

async fn get_short_url(
    State(state): State<AppState>,
    Path(short): Path<String>,
) -> Result<Json<UrlResponse>, StatusCode> {
    let urls = state.lock().await;

    if let Some(url) = urls.iter().find(|u| u.short == short) {
        Ok(Json(UrlResponse {
            short: url.short.clone(),
            long: url.long.clone(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_short_url(
    State(state): State<AppState>,
    Path(short): Path<String>,
) -> StatusCode {
    let mut urls = state.lock().await;
    let len_before = urls.len();
    urls.retain(|u| u.short != short);

    if urls.len() < len_before {
        let urls_data = urls.clone();
        drop(urls);
        save_to_file(&urls_data).await;
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn list_user_urls(
    State(state): State<AppState>,
    Path(user): Path<String>,
) -> Json<Vec<UrlResponse>> {
    let urls = state.lock().await;
    let user_urls: Vec<UrlResponse> = urls
        .iter()
        .filter(|u| u.user == user)
        .map(|u| UrlResponse {
            short: u.short.clone(),
            long: u.long.clone(),
        })
        .collect();

    Json(user_urls)
}

#[tokio::main]
async fn main() {
    let urls = Arc::new(Mutex::new(load_urls()));

    let app = Router::new()
        .route("/urls", post(create_short_url))
        .route("/urls/:short", get(get_short_url))
        .route("/urls/:short", delete(delete_short_url))
        .route("/users/:user", get(list_user_urls))
        .with_state(urls);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("API URL Shortener running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
