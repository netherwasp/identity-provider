use axum::{ Json, Router, response::IntoResponse, routing::get };
use tower_http::services::{ ServeDir, ServeFile };
use serde_json::json;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // .fallback_service(
        //     ServeDir::new("my-app/browser").fallback(
        //         ServeFile::new("my-app/browser/index.html")
        //     )
        // )
        .route("/test", get(test));

    let addr = "127.0.0.1:4000";
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn test() -> impl IntoResponse {
    println!("test");
    Json(json!({ "test": "test" }))
}
