use axum::{Router, extract::State, http::StatusCode, routing::post};
use serenity::all::{ChannelId, Http};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
struct SharedBotState {
    discord_http: Arc<Http>,
    bad_data_channel_id: ChannelId,
}

pub async fn run_server(
    discord_http: Arc<Http>,
    target_channel_id: ChannelId,
    listen_addr: SocketAddr,
) {
    let shared_state: SharedBotState = SharedBotState {
        discord_http,
        bad_data_channel_id: target_channel_id,
    };

    let app: Router = Router::new()
        .route("/capbot", post(handle_recon_post))
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

async fn handle_recon_post(
    State(state): State<SharedBotState>,
    body: String,
) -> Result<StatusCode, (StatusCode, String)> {
    println!("Received POST request on /capbot with:\n{}", body);

    match state
        .bad_data_channel_id
        .say(&state.discord_http, format!("Bad data: {}", body))
        .await
    {
        Ok(_) => {
            println!("Bad data message forwarded to bad data channel");
            Ok(StatusCode::OK) // Return HTTP 200 OK
        }
        Err(e) => {
            eprintln!("Failed to send bad data message {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to send bad data message {}", e),
            ))
        }
    }
}
