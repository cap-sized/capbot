use axum::{Router, routing::post};
use serenity::all::{ChannelId, Http};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::bot::controls::level::handle_level_post;


#[derive(Clone)]
pub struct SharedBotState {
    pub discord_http: Arc<Http>,
    pub devlog_channel_id: ChannelId,
}

pub async fn run_server(
    discord_http: Arc<Http>,
    target_channel_id: ChannelId,
    listen_addr: SocketAddr,
) {
    let shared_state: SharedBotState = SharedBotState {
        discord_http,
        devlog_channel_id: target_channel_id,
    };

    let app: Router = Router::new()
        .route("/level", post(handle_level_post))
        .with_state(shared_state);

    println!("Capbot HTTP server listening on {}", listen_addr);

    let listener: tokio::net::TcpListener = match tokio::net::TcpListener::bind(listen_addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}\nError: {}", listen_addr, e);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        eprintln!("Capbot Server error: {}", e);
    } else {
        println!("Capbot shut down gracefully.");
    }
}
