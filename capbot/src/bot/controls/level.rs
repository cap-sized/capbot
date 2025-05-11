use std::fmt;
use std::sync::{Arc, RwLock};

use axum::extract::{Query, State};
use reqwest::StatusCode;
use serde::Deserialize;
use serenity::all::{Context, ResolvedOption, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

use crate::server::SharedBotState;

#[derive(Deserialize, Debug)]
pub struct LevelParams {
    filter: Option<LevelFilters>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelFilters {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LevelFilters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)] // Clone is important if SharedBotState is Clone (common for Axum state)
pub struct LogLevel {
    level: Arc<RwLock<LevelFilters>>,
}

impl LogLevel {
    pub fn new() -> Self {
        Self {
            level: Arc::new(RwLock::new(LevelFilters::Warn)),
        }
    }

    pub fn set_level(&self, level: LevelFilters) {
        let mut lock = self.level.write().expect("Level RwLock poisoned");
        *lock = level;
    }

    pub fn get_level(&self) -> LevelFilters {
        let lock = self.level.read().expect("Level RwLock poisoned");
        *lock
    }
}

pub async fn handle_level_post(
    State(state): State<SharedBotState>,
    Query(params): Query<LevelParams>,
) -> Result<StatusCode, (StatusCode, String)> {
    let (filter_string, reply);

    match params.filter {
        Some(level) => {
            filter_string = level.to_string();
            state.log_level.set_level(level);
            reply = format!("Log level filter set to: {filter_string} via HTTP");
            println!("{reply}");
        }
        None => {
            reply = format!("Did not receive filter");
            println!("{reply}");
        }
    };

    state
        .config
        .logging_channel_id
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

pub async fn run(ctx: &Context, options: &[ResolvedOption<'_>]) -> String {
    let data_read = ctx.data.read().await;
    let shared_state = data_read.get::<SharedBotState>().unwrap();

    if let Some(ResolvedOption {
        value: ResolvedValue::String(selected_level),
        ..
    }) = options.first()
    {
        // Value will always be one of the 4 log levels
        let log_level = match *selected_level {
            "Debug" => Some(LevelFilters::Debug),
            "Info" => Some(LevelFilters::Info),
            "Warn" => Some(LevelFilters::Warn),
            "Error" => Some(LevelFilters::Error),
            _ => None,
        };
        shared_state.log_level.set_level(log_level.unwrap());

        format!("You have selected a log level: {}", selected_level)
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
