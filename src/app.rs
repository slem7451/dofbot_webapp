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

///Запуск сервера
#[tokio::main]
pub async fn serve() {
    env_logger::init();
    
    let app = Router::new()
        .route("/static/*path", get(serve_static_file)) //Подгрузка статических файлов
        .route("/", get(handle_index))
        .route("/contacts", get(handle_contacts))
        .route("/servo", post(handle_servo))
        .route("/pose", post(handle_pose))
        .route("/trajectory", post(handle_trajectory));

    //let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();   //Запуск сервера локально
    let listener = tokio::net::TcpListener::bind("192.168.1.11:3000").await.unwrap(); //Запуск сервера на роботе
    axum::serve(listener, app).await.unwrap();
}


/// Без данной функции, сервер не увидит статические файлы
/// * `path` - GET-параметр пути обращения к файлу
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
