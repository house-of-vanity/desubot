use std::env;

use futures::StreamExt;
use telegram_bot::types::chat::MessageChat;
use telegram_bot::*;

mod commands;
mod db;

async fn handler(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => {
            let title = match &message.chat {
                MessageChat::Supergroup(chat) => &chat.title,
                _ => "test",
            };

            println!(
                "<{}>[{}::{}({})]: {}",
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
async fn main() -> Result<(), Error> {
    /*
    println!("get_user: {:?}", db::get_user(1228494339));
    println!("get_confs: {:?}", db::get_confs());
    println!("get_members: {:?}", db::get_members(-1001233797421));

     */
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            handler(api.clone(), message).await?;
        }
    }
    Ok(())
}
/*
{ id: MessageId(94793), from:
    User { id: UserId(124317807), first_name: "Холм", last_name: Some("Вечный"), username: Some("ultradesu"), is_bot: false, language_code: Some("en") },
    date: 1606564847,
    chat: Supergroup(Supergroup { id: SupergroupId(-1001233797421), title: "Квантовый Аллах", username: None, invite_link: None }),
    forward: None, reply_to_message: None, edit_date: None, kind: Text { data: "пук", entities: [] }

 */
