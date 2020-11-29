use crate::db;
use html_escape::encode_text;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, ParseMode, UpdateKind};
use tokio::time::delay_for;

pub(crate) async fn here(api: Api, message: Message) -> Result<(), Error> {
    let members: Vec<telegram_bot::User> = db::get_members(message.chat.id()).unwrap();
    for u in &members {
        println!("Found user {:?}", u);
    }
    let mut msg = "<b>I summon you</b>, ".to_string();
    for user in members {
        let mention = match user.username {
            Some(username) => format!("@{}", username),
            _ => format!(
                "<a href=\"tg://user?id={}\">{}</a>",
                encode_text(&user.id.to_string()),
                encode_text(&user.first_name)
            ),
        };
        msg = format!("{} {}", msg, mention);
    }
    println!("Message: {:?}", msg);

    api.send(message.text_reply(msg).parse_mode(ParseMode::Html))
        .await?;
    //api.send(message.chat.text("Text to message chat")).await?;
    //api.send(message.from.text("Private text")).await?;
    Ok(())
}
