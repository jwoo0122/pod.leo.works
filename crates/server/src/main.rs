use axum::{extract::Path, routing::get, Router};
use std::fs::read_dir;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/episode/:name", get(episode));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Ready on localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn episode(Path(name): Path<String>) -> Result<_, String> {
    if let Ok(episodes) = read_dir("../episodes") {
        for episode in episodes {
            let episode = episode?;
            println!("{}", episode.path());
        }
    }
}
