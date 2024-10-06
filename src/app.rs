mod controller;

use axum::{
    routing::{get, post}, 
    Router, 
    response::IntoResponse,
    body::Bytes,
    extract::Path
};
use controller::*;
use tokio::fs;

#[tokio::main]
pub async fn serve() {
    let app = Router::new()
        .route("/static/*path", get(serve_static_file))
        .route("/", get(handle_index))
        .route("/contacts", get(handle_contacts))
        .route("/servo1", post(handle_servo1))
        .route("/servo2", post(handle_servo2))
        .route("/servo3", post(handle_servo3))
        .route("/servo4", post(handle_servo4))
        .route("/servo5", post(handle_servo5))
        .route("/servo6", post(handle_servo6));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_static_file(Path(path): Path<String>) -> impl IntoResponse {
    let file_path = format!("src/static/{}", path);
    
    match fs::read(&file_path).await {
        Ok(content) => {
            (axum::http::StatusCode::OK, Bytes::from(content))
        },
        Err(_) => {
            (axum::http::StatusCode::NOT_FOUND, "File not found".into())
        },
    }
}
