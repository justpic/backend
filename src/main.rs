mod error;

use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let app = Router::new();
    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    
    axum::serve(listener, app).await?;
    Ok(())
}
