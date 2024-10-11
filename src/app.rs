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
        .route("/servo", post(handle_servo));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
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
