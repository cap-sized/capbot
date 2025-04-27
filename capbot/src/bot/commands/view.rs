// Since there's a whole HTTP server set up, might as well try this lol
// GET requests to the NHL API

use reqwest::Client as ReqwestClient;
use serenity::Result as SerenityResult;
use serenity::all::{Context, Message};

const NHL_API_BASE: &str = "https://api-web.nhle.com/v1/meta";

pub async fn handle(
    ctx: &Context,
    msg: &Message,
    args: &[&str],
    http_client: &ReqwestClient,
) -> SerenityResult<()> {
    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "Please pass in some arguments for this command. (See `.help view` for more info)").await?;
        return Ok(());
    }

    // args (something like ["players=123", "teams=ABC,DEF"])
    let mut query_params: Vec<String> = vec![];
    for arg in args {
        if let Some((key, value)) = arg.split_once("=") {
            // Should probably add some sort of checker but idk NHL API
            query_params.push(format!("{}={}", key.trim(), value.trim()));
        } else {
            msg.reply(
                &ctx.http,
                format!("Invalid argument: {}\n Please us type=val1,val2,...", arg),
            )
            .await
            .ok();
        }
    }

    if query_params.is_empty() {
            msg.reply(
                &ctx.http,
                format!("No valid arguments passed for API. Ignoring request"),
            )
            .await
            .ok();
        return Ok(());
    }

    let query_string = query_params.join("&");
    let request_url = format!("{}?{}", NHL_API_BASE, query_string);

    println!("Making NHL API request to: {}", request_url);

    match http_client.get(&request_url).send().await {
        Ok(response) => {
            match response.text().await {
                Ok(body) => {
                    let mut formatted_body = format!("```json\n{}\n```", body);
                    if formatted_body.len() > 2000 {
                        formatted_body.truncate(2000);
                        msg.channel_id.say(&ctx.http, "NHL API Response too long, truncated").await?;
                    }

                    msg.channel_id.say(&ctx.http, formatted_body).await?;
                }
                Err(e) => {
                    eprintln!("Failed to read NHL API response body: {}", e);
                    msg.reply(
                        &ctx.http,
                        format!("Failed to read NHL API response body: {}", e),
                    )
                    .await
                    .ok();
                }
            }
        }
        Err(e) => {
            eprintln!("Error with GET request to NHL API: {}\nURL={}", e, request_url);
            msg.reply(
                &ctx.http,
                format!("Error with GET request to NHL API: {}\nURL={}", e, request_url),
            )
            .await
            .ok();
        }
    }

    Ok(())
}
