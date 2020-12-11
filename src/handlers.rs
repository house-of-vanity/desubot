use telegram_bot::*;
use crate::mystem::MyStem;
use crate::errors;
use crate::db;
use crate::commands;
use crate::utils;


pub async fn handler(
    api: Api,
    message: Message,
    token: String,
    mystem: &mut MyStem,
    me: User,
) -> Result<(), errors::Error> {

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
            db::add_sentence(&message, mystem).await?;
            match data.as_str() {
                "/here" => commands::here(api, message).await?,
                "/top" => commands::top(api, message).await?,
                "/stat" => commands::top(api, message).await?,
                "/markov_all" => commands::markov_all(api, message).await?,
                "/markov" => commands::markov(api, message).await?,
                _ => (),
            }
        }
        MessageKind::Photo { ref caption, .. } => {
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

        MessageKind::Sticker { .. } => {
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
