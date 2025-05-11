// src/bot/controls/bot_status.rs
use serenity::all::Context; // Renamed for clarity
use serenity::builder::CreateCommand;
// We will get LevelGlobal via SharedBotState
use crate::server::SharedBotState; // Import your SharedBotState

pub async fn run(ctx: &Context) -> String {
    let mut status_messages: Vec<String> = Vec::new();

    let data_read = ctx.data.read().await;
    let shared_state = data_read.get::<SharedBotState>().unwrap();
    
    let current_level = shared_state.log_level.get_level();
    status_messages.push(format!("Log Level: {}", current_level));

    status_messages.join("\n")

}

pub fn register() -> CreateCommand {
    CreateCommand::new("status").description("Current Bot Status")
}