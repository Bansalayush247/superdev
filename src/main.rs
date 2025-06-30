use axum::{routing::post, Router};
use tokio::net::TcpListener;
mod handlers;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/keypair", post(handlers::keypair::generate_keypair))
        .route("/token/create", post(handlers::token::create_token))
        .route("/token/mint", post(handlers::token::mint_token))
         .route("/message/sign", post(handlers::message::sign_message))
        .route("/message/verify", post(handlers::message::verify_message))
        .route("/send/sol", post(handlers::transfer::send_sol))
        .route("/send/token", post(handlers::transfer::send_token));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
