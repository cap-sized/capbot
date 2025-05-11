use serenity::all::ChannelId;
use serenity::prelude::TypeMapKey;
use std::env;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Config {
    pub discord_token: String,
    pub logging_channel_id: ChannelId,
    pub listen_addr: SocketAddr,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().expect("Failed to load .env file");

        let discord_token= env::var("CAPBOT_TOKEN").expect("Missing CAPBOT_TOKEN");
        let bad_data_channel_id_str= env::var("CAPSIZED_CHANNEL_ID").expect("Error reading CAPSIZED_CHANNEL_ID from .env");

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
            logging_channel_id: bad_data_channel_id,
            listen_addr,
        }
    }
}
