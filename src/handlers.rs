//use crate::commands::Command;
use crate::commands::{Code, Execute, Here, Markov, MarkovAll, Omedeto, Scheme, Sql, Top};
use crate::db;
use crate::errors;
use crate::utils;
use mystem::MyStem;
use telegram_bot::*;

include!("../assets/help_text.rs");

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
                data.replace("\n", " ")
            );

            let cleaned_message = data.replace(&format!("@{}", me.clone().username.unwrap()), "");
            match cleaned_message.as_str() {
                s if s.to_string().starts_with("/code") => {
                    match {
                        Code {
                            data: s.replace("/code", ""),
                        }
                        .exec_with_result(&api, &message)
                        .await
                    } {
                        Ok(path) => {
                            let mut cnt_lines = 0;
                            for _ in s.lines() {
                                cnt_lines = cnt_lines + 1;
                            }
                            let mut cnt_chars = 0;
                            for _ in s.chars() {
                                cnt_chars = cnt_chars + 1;
                            }
                            let file = InputFileUpload::with_path(path.clone());
                            info!("lines: {}, chars: {}", cnt_lines, cnt_chars);
                            // api.send(message.chat.document(&file)).await?;
                            //
                            // // Send an image from disk
                            if cnt_chars > 4000 {
                                let _ = api
                                    .send(message.text_reply(CODE_HELP).parse_mode(ParseMode::Html))
                                    .await?;
                                return Ok(());
                            }
                            if cnt_lines < 81 {
                                api.send(message.chat.photo(&file)).await?;
                            } else {
                                api.send(message.chat.document(&file)).await?;
                            }
                            //debug!("{:#?}", formatter);
                            let _ = std::fs::remove_file(&path);
                        }
                        Err(_) => {
                            let _ = api
                                .send(message.text_reply(CODE_HELP).parse_mode(ParseMode::Html))
                                .await?;
                        }
                    }
                }
                s if s.contains("/here")
                    || s.contains("@here")
                    || s.contains("/хере")
                    || s.contains("@хере")
                    || s.contains("\"хере") =>
                {
                    db::add_sentence(&message, mystem).await?;
                    Here {
                        data: "".to_string(),
                    }
                    .exec(&api, &message)
                    .await?
                }
                s if s.to_string().starts_with("/sql") => match {
                    Sql {
                        data: s.replace("/sql ", ""),
                    }
                    .exec_with_result(&api, &message)
                    .await
                } {
                    Ok(msg) => {
                        let _ = api
                            .send(message.text_reply(msg).parse_mode(ParseMode::Html))
                            .await?;
                    }
                    Err(e) => {
                        let _ = api
                            .send(
                                message
                                    .text_reply(format!("Error: {:#?}", e))
                                    .parse_mode(ParseMode::Html),
                            )
                            .await?;
                    }
                },
                "/top" => {
                    Top {
                        data: "".to_string(),
                    }
                    .exec(&api, &message)
                    .await?
                }
                "/stat" => {
                    Top {
                        data: "".to_string(),
                    }
                    .exec(&api, &message)
                    .await?
                }
                "/markov_all" => {
                    MarkovAll {
                        data: "".to_string(),
                    }
                    .exec(&api, &message)
                    .await?
                }
                "/markov" => {
                    Markov {
                        data: "".to_string(),
                    }
                    .exec(&api, &message)
                    .await?
                }
                s if s =="/scheme" || s == "/schema" => {
                    Scheme {
                        data: "".to_string(),
                    }
                    .exec(&api, &message)
                    .await?
                }
                "/omedeto" => {
                    Omedeto {
                        data: "".to_string(),
                    }
                    .exec_mystem(&api, &message, mystem)
                    .await?
                }
                _ => db::add_sentence(&message, mystem).await?,
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
                caption.clone().unwrap_or_else(|| "NO_TITLE".to_string())
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
                caption.clone().unwrap_or_else(|| "NO_TITLE".to_string())
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
