use crate::bot::controls;
use crate::server::SharedBotState;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(controls::ping::run()),
                "status" => Some(controls::bot_status::run(&ctx).await),
                "level" => Some(controls::level::run(&ctx, &command.data.options()).await),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let data_read = ctx.data.read().await;
        let shared_state = data_read.get::<SharedBotState>().unwrap().clone();

        let commands = shared_state
            .config
            .guild_id
            .set_commands(
                &ctx.http,
                vec![
                    controls::ping::register(),
                    controls::bot_status::register(),
                    controls::level::register(),
                ],
            )
            .await;

        // Update this when more stuff is added / when I find a better way
        let ready_message = format!(
"# Hello, I am CapBot! :wave: 
_This start-up message is only sent here_
## Getting started
### Main bot commands:
`/level` -> Sets a minimum logging level which an `alert` must have before being channeled here (set to **Info** by default)
`/status` -> Check the current bot status.");

        if let Err(err) = shared_state
            .config
            .logging_channel_id
            .say(&ctx.http, ready_message)
            .await
        {
            println!("Failed to send ready message: {}", err);
        }

        println!("I now have the following guild slash commands: {commands:?}");
    }
}
