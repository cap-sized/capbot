use axum::{Router, routing::post};
use serenity::{
    all::Http,
    prelude::TypeMapKey,
};
use std::sync::Arc;


// use crate::bot::controls::level::{LogLevel, handle_level_post};
use bot::controls::level::{LogLevel, handle_level_post};
use config::Config;

use crate::{bot, config};

// Add more global variables here if needed
#[derive(Clone)]
pub struct SharedBotState {
    pub discord_http: Arc<Http>,
    pub config: Config,
    pub log_level: LogLevel,
}

impl SharedBotState {
    // Constructor for SharedBotState
    pub fn new(discord_http: Arc<Http>, config: Config, log_level: LogLevel) -> Self {
        Self {
            discord_http,
            config,
            log_level,
        }
    }
}

impl TypeMapKey for SharedBotState {
    type Value = Arc<SharedBotState>;
}

pub async fn run_server(shared_state: Arc<SharedBotState>) {

    let app: Router = Router::new()
        .route("/level", post(handle_level_post))
        .with_state((*shared_state).clone());

    println!("Capbot HTTP server listening on {}", shared_state.config.listen_addr);

    let listener: tokio::net::TcpListener = match tokio::net::TcpListener::bind(shared_state.config.listen_addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}\nError: {}", shared_state.config.listen_addr, e);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        eprintln!("Capbot Server error: {}", e);
    } else {
        println!("Capbot shut down gracefully.");
    }
}
