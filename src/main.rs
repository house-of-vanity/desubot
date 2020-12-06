use std::{env, process};

use futures::StreamExt;
use reqwest;
use telegram_bot::types::chat::MessageChat;
use telegram_bot::*;
#[macro_use]
extern crate log;
use env_logger::Env;

mod commands;
mod db;
mod errors;
mod utils;

async fn handler(api: Api, message: Message, token: String) -> Result<(), errors::Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => {
            let title = utils::get_title(&message);
            info!(
                "<{}({})>[{}({})]: {}",
                &message.chat.id(),
                title,
                &message.from.id,
                &message.from.first_name,
                data
            );
            db::add_sentence(&message).await?;
            match data.as_str() {
                "/here" => commands::here(api, message).await?,
                _ => (),
            }
        }
        MessageKind::Photo {
            ref caption,
            ref data,
            ..
        } => {
            let title = utils::get_title(&message);
            info!(
                "<{}({})>[{}({})]: *PHOTO* {}",
                &message.chat.id(),
                title,
                &message.from.id,
                &message.from.first_name,
                caption.clone().unwrap_or("NO_TITLE".to_string())
            );
            utils::get_files(api, message, token).await?;
        }

        MessageKind::Document { ref caption, .. } => {
            let title = utils::get_title(&message);
            info!(
                "<{}({})>[{}({})]: *DOCUMENT* {}",
                &message.chat.id(),
                title,
                &message.from.id,
                &message.from.first_name,
                caption.clone().unwrap_or("NO_TITLE".to_string())
            );
            utils::get_files(api, message, token).await?;
        }

        MessageKind::Sticker { ref data, .. } => {
            let title = utils::get_title(&message);
            info!(
                "<{}({})>[{}({})]: *STICKER*",
                &message.chat.id(),
                title,
                &message.from.id,
                &message.from.first_name,
            );
            utils::get_files(api, message, token).await?;
        }

        MessageKind::Voice { .. } => {
            let title = utils::get_title(&message);
            info!(
                "<{}({})>[{}({})]: *VOICE*",
                &message.chat.id(),
                title,
                &message.from.id,
                &message.from.first_name,
            );
            utils::get_files(api, message, token).await?;
        }

        MessageKind::Video { .. } => {
            let title = utils::get_title(&message);
            info!(
                "<{}({})>[{}({})]: *VIDEO*",
                &message.chat.id(),
                title,
                &message.from.id,
                &message.from.first_name,
            );
            utils::get_files(api, message, token).await?;
        }

        MessageKind::VideoNote { .. } => {
            let title = utils::get_title(&message);
            info!(
                "<{}({})>[{}({})]: *VIDEO_NOTE*",
                &message.chat.id(),
                title,
                &message.from.id,
                &message.from.first_name,
            );
            utils::get_files(api, message, token).await?;
        }
        _ => (),
    };
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    env_logger::from_env(Env::default().default_filter_or("debug")).init();
    db::update_scheme();
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            error!("TELEGRAM_BOT_TOKEN not set");
            process::exit(0x0001);
        }
    };
    let api = Api::new(token.clone());

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            db::add_user(message.clone()).await?;
            db::add_conf(message.clone()).await?;

            handler(api.clone(), message, token.clone()).await?;
        }
    }
    Ok(())
}
