use std::env;

use futures::StreamExt;
use telegram_bot::types::chat::MessageChat;
use telegram_bot::*;

mod commands;
mod db;
mod errors;
mod utils;

async fn handler(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => {
            let title = utils::get_title(&message);

            println!(
                "<{}({})>[{}({})]: {}",
                &message.chat.id(),
                title,
                &message.from.id,
                &message.from.first_name,
                data
            );
            match data.as_str() {
                "/here" => commands::here(api, message).await?,
                _ => (),
            }
        }
        _ => (),
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            db::add_user(api.clone(), message.clone()).await?;
            db::add_conf(api.clone(), message.clone()).await?;

            handler(api.clone(), message).await?;
        }
    }
    Ok(())
}
