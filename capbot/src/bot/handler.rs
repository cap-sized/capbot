use serenity::all::{ChannelId, Context, EventHandler, Message, Ready};
use serenity::async_trait;

use std::env;
use super::controls;

// Shared client
pub struct Handler;

const COMMAND_PREFIX: &str = ".";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if let Some(content) = msg.content.strip_prefix(COMMAND_PREFIX) {
            let parts: Vec<&str> = content.split_whitespace().collect();
            let cmd: &str = parts.get(0).copied().unwrap_or("");
            let args: &[&str] = parts.get(1..).unwrap_or(&[]);

            println!("Received command: {} with args: {:?}", cmd, args);

            // Command dispatcher
            let command_result: Result<(), serenity::Error> = match cmd.to_lowercase().as_str() {
                "help" => controls::help::handle(&ctx, &msg, &args).await,
                _ => {
                    // DEBUG
                    msg.reply(&ctx.http, format!("Unknown command: `{}`", cmd))
                        .await
                        .ok();
                    Ok(())
                }
            };

            // Log any errors that occurred when running a command
            if let Err(why) = command_result {
                eprintln!("Error with command '{}': {:?}", cmd, why);
                let _ = msg
                    .reply(&ctx.http, format!("Invalid command: {}", msg.content))
                    .await
                    .ok();
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let bad_data_channel_id_str: String =
            env::var("BAD_DATA_CHANNEL_ID").expect("Error reading BAD_DATA_CHANNEL_ID from .env");
        let bad_data_channel_id: ChannelId = ChannelId::new(
            bad_data_channel_id_str
                .parse::<u64>()
                .expect("BAD_DATA_CHANNEL_ID must be an integer"),
        );
        bad_data_channel_id
            .say(&ctx.http, "Hi I am capbot. Use `.` for command prefixes")
            .await
            .ok();
        println!("{} is connected!", ready.user.name);
    }
}
