use crate::bot::controls;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use std::env;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(controls::ping::run(&command.data.options())),
                "level" => Some(controls::level::run(&command.data.options())),
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

        let guild_id = GuildId::new(
            env::var("CAPSIZED_GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                controls::ping::register(),
                controls::level::register(),
                // controls::welcome::register(),
                // controls::numberinput::register(),
                // controls::attachmentinput::register(),
                // controls::modal::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:?}");

        // let guild_command =
        //     Command::create_global_command(&ctx.http, controls::wonderful_command::register())
        //         .await;

        // println!("I created the following global slash command: {guild_command:#?}");
    }
}