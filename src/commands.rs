use crate::db;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, ParseMode, UpdateKind};
use tokio::time::delay_for;

pub(crate) async fn here(api: Api, message: Message) -> Result<(), Error> {
    let members: Vec<telegram_bot::User> = db::get_members(&message.chat.id()).unwrap();
    let mut msg = "I summon you, ".to_string();
    for user in members {
        let mention = match user.username {
            Some(username) => format!("@{}", username),
            _ => format!("[{}](tg://user?id={})", user.first_name, user.id),
        };
        msg = format!("{} {}", msg, mention);
    }

    api.send(message.text_reply(msg).parse_mode(ParseMode::MarkdownV2))
        .await?;
    //api.send(message.chat.text("Text to message chat")).await?;
    //api.send(message.from.text("Private text")).await?;
    Ok(())
}
