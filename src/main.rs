use anyhow::Result;
use axum::{Router, response::Html, routing::get};
use tracing_subscriber;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on http://{}" , listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> Html<String> {
    Html(String::from("Hello, world!"))
}