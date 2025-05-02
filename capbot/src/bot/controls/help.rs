// testing
use serenity::all::{Context, Message};

pub async fn handle(ctx: &Context, msg: &Message, args: &[&str]) -> serenity::Result<()> {
    let _ = args; // TODO .help view .help server

    msg.channel_id.say(&ctx.http,"Hi this is Cap Bot. How may I help you?").await?;

    Ok(())
}
