//use crate::commands::Command;
use crate::commands::{Execute, Here, Markov, MarkovAll, Omedeto, Sql, Top};
use crate::db;
use crate::errors;
use crate::utils;
use mystem::MyStem;
use telegram_bot::*;

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
            let cleaned_message = data.replace(&format!("@{}", me.clone().username.unwrap()), "");
            match cleaned_message.as_str() {
                s if s.contains("/here") => {
                    Here {
                        data: "".to_string(),
                    }
                    .run(api, message)
                    .await?
                }
                s if s.to_string().starts_with("/sql") => {
                    Sql {
                        data: s.replace("/sql ", ""),
                    }
                    .run(api, message)
                    .await?
                }
                "/top" => {
                    Top {
                        data: "".to_string(),
                    }
                    .run(api, message)
                    .await?
                }
                "/stat" => {
                    Top {
                        data: "".to_string(),
                    }
                    .run(api, message)
                    .await?
                }
                "/markov_all" => {
                    MarkovAll {
                        data: "".to_string(),
                    }
                    .run(api, message)
                    .await?
                }
                "/markov" => {
                    Markov {
                        data: "".to_string(),
                    }
                    .run(api, message)
                    .await?
                }
                "/omedeto" => {
                    Omedeto {
                        data: "".to_string(),
                    }
                    .run_mystem(api, message, mystem)
                    .await?
                }
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
