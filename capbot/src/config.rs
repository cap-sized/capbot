use serenity::all::ChannelId;
use std::env;
use std::net::SocketAddr;

pub struct Config {
    pub discord_token: String,
    pub bad_data_channel_id: ChannelId,
    pub listen_addr: SocketAddr,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().expect("Failed to load .env file");

        let discord_token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");

        let bad_data_channel_id_str: String =
            env::var("BAD_DATA_CHANNEL_ID").expect("Error reading BAD_DATA_CHANNEL_ID from .env");
        let bad_data_channel_id: ChannelId = ChannelId::new(
            bad_data_channel_id_str
                .parse::<u64>()
                .expect("BAD_DATA_CHANNEL_ID must be an integer"),
        );

        let listen_addr = env::var("HTTP_LISTEN_ADDR")
            .expect("Missing HTTP_LISTEN_ADDR")
            .parse::<SocketAddr>()
            .expect("HTTP_LISTEN_ADDR must be a valid socket address");

        Config {
            discord_token,
            bad_data_channel_id,
            listen_addr,
        }
    }
}
