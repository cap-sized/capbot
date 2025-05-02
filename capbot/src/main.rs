mod bot;
mod server;
mod config;

use serenity::all::{Client, GatewayIntents};
use std::sync::Arc;

use bot::handler::Handler;
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

    
    let discord_http_client: Arc<serenity::all::Http> = Arc::clone(&client.http);
    tokio::spawn(async move {
        server::run_server(discord_http_client, cfg.bad_data_channel_id, cfg.listen_addr).await;
    });

    // Start capbot
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
