use askama::Template;
use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, Router},
};
use axum_core::response::IntoResponse;
mod routes;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    (StatusCode::OK, Html(template.render().unwrap())).into_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = libpasskey::passkey::app_state().await?;

    let app = Router::new()
        .route("/", get(index))
        .merge(routes::create_router(state));

    println!("Starting server on http://localhost:3001");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
