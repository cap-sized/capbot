mod bot;
mod server;

use reqwest::Client as ReqwestClient;
use serenity::all::{ChannelId, Client, GatewayIntents};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use bot::handler::Handler;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().expect("Failed to load .env file. (Ensure that there's one at src/)");

    let token: String = env::var("DISCORD_TOKEN").expect("Error reading DISCORD_TOKEN from .env");

    // Channel to report bad data or whatever
    let bad_data_channel_id_str: String =
        env::var("BAD_DATA_CHANNEL_ID").expect("Error reading BAD_DATA_CHANNEL_ID from .env");
    let bad_data_channel_id: ChannelId = ChannelId::new(
        bad_data_channel_id_str
            .parse::<u64>()
            .expect("BAD_DATA_CHANNEL_ID must be an integer"),
    );

    // Get HTTP server listen address
    let listen_addr_str: String =
        env::var("HTTP_LISTEN_ADDR").expect("Error reading HTTP_LISTEN_ADDR from .env");
    let listen_addr: SocketAddr = listen_addr_str
        .parse()
        .expect("HTTP_LISTEN_ADDR must be a valid socket address (e.g., 0.0.0.0:3000)");

    // Bot Intents (Bot -> View Channels + Send Messages in OAuth2)
    let intents: GatewayIntents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Request Client
    let reqwest_client: Arc<ReqwestClient> = Arc::new(ReqwestClient::new());

    // Disocrd Client client
    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Handler {
            reqwest_client: Arc::clone(&reqwest_client),
        })
        .await
        .expect("Error creating Discord client");

    // Sharing a client with capbot's HTTP server
    let discord_http_client: Arc<serenity::all::Http> = Arc::clone(&client.http);
    tokio::spawn(async move {
        server::run_server(discord_http_client, bad_data_channel_id, listen_addr).await;
    });

    // Start capbot
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
