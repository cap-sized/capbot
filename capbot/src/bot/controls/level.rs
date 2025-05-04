use std::fmt;

use axum::extract::{Query, State};
use reqwest::StatusCode;
use serde::Deserialize;
use serenity::all::{ResolvedOption, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

use crate::server::SharedBotState;

#[derive(Deserialize, Debug)]
pub struct LevelParams {
    filter: Option<LevelFilters>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum LevelFilters {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LevelFilters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // Outputs "Debug", "Info", etc.
    }
}

pub async fn handle_level_post(
    State(state): State<SharedBotState>,
    Query(params): Query<LevelParams>
) -> Result<StatusCode, (StatusCode, String)> {
    let (filter_string, reply);

    match params.filter {
        Some(level) => {
            filter_string = level.to_string();
            println!("Somehow storing {filter_string:?} *globally* (IDK BRUH)...");
            // TODO: Implement the aforementioned
            // https://github.com/serenity-rs/serenity/tree/current/examples/e12_global_data
            reply = format!("Invalid params for /level: {filter_string:?}")
        }
        None => {
            filter_string = "None".to_string();
            reply = format!("Received POST for /level: {filter_string:?}")
        }
    };
    
    state
        .devlog_channel_id
        .say(&state.discord_http, reply)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to send message: {}", e),
            )
        })
}

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(level),
        ..
    }) = options.first()
    {
        format!("You have selected a log level: {}", level)
    } else {
        "Please provide a valid log level".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("level")
        .description("Set a logging level")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "loglevel", "Min loging level")
                .add_string_choice("Debug", "Debug")
                .add_string_choice("Info", "Info")
                .add_string_choice("Warn", "Warn")
                .add_string_choice("Error", "Error")
                .required(true),
        )
}
