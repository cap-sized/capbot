mod bot;
mod server;
mod config;

use serenity::all::{Client, GatewayIntents};
use server::SharedBotState;
use std::sync::Arc;

use bot::{controls::level::LogLevel, handler::Handler};
use config::Config;


#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    let cfg = Config::load();


    // Bot Intents (Bot -> View Channels + Send Messages in OAuth2)
    let intents: GatewayIntents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Disocrd Client client
    let mut client: Client = Client::builder(&cfg.discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating Discord client");

    let shared_state_instance = Arc::new(SharedBotState::new(
        Arc::clone(&client.http),
        cfg,
        LogLevel::new()
    ));

    {
        let mut guard = client.data.write().await;
        guard.insert::<SharedBotState>(
            shared_state_instance.clone()
        );

    }
    
    tokio::spawn(async move {
        server::run_server(shared_state_instance).await;
    });

    // Start capbot
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
