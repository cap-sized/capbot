use std::sync::Arc;

use crate::server::SharedBotState;
use serenity::all::Context;
use serenity::builder::CreateCommand;

pub fn generate_status_string(shared_state: &Arc<SharedBotState>) -> String {
    let log_level = shared_state.log_level.get_level();
    let listening_addr = shared_state.config.listen_addr;

    format!(
"## Current configuration variables:
```
Logging level: {log_level}
Listening Address: {listening_addr}
```"
    )
    .to_string()
}

pub async fn run(ctx: &Context) -> String {
    let data_read = ctx.data.read().await;
    let shared_state = data_read.get::<SharedBotState>().unwrap().clone();

    generate_status_string(&shared_state)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("status").description("Current Bot Status")
}
