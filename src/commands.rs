use crate::db;
use crate::errors::Error;
use html_escape::encode_text;
use markov::Chain;
use rand::Rng;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Message, ParseMode};

pub(crate) async fn here(api: Api, message: Message) -> Result<(), Error> {
    let members: Vec<telegram_bot::User> = db::get_members(message.chat.id()).unwrap();
    for u in &members {
        debug!("Found user {:?} in chat {}", u, message.chat.id());
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

    match api
        .send(message.text_reply(msg).parse_mode(ParseMode::Html))
        .await
    {
        Ok(_) => debug!("/here command sent to {}", message.chat.id()),
        Err(_) => warn!("/here command sent failed to {}", message.chat.id()),
    }
    //api.send(message.chat.text("Text to message chat")).await?;
    //api.send(message.from.text("Private text")).await?;
    Ok(())
}

pub(crate) async fn top(api: Api, message: Message) -> Result<(), Error> {
    let top = db::get_top(&message).await?;
    let mut msg = "<b>Your top using words:</b>\n<pre>".to_string();
    let mut counter = 1;
    for word in top.iter() {
        msg = format!(
            "{} <b>{}</b> {} - {}\n",
            msg, counter, word.word, word.count
        );
        counter += 1;
    }
    msg = format!("{}{}", msg, "</pre>");
    match api
        .send(message.text_reply(msg).parse_mode(ParseMode::Html))
        .await
    {
        Ok(_) => debug!("/top command sent to {}", message.chat.id()),
        Err(_) => warn!("/top command sent failed to {}", message.chat.id()),
    }
    //api.send(message.chat.text("Text to message chat")).await?;
    //api.send(message.from.text("Private text")).await?;
    Ok(())
}

pub(crate) async fn markov_all(api: Api, message: Message) -> Result<(), Error> {
    let messages = db::get_random_messages().await?;
    let mut chain = Chain::new();
    chain.feed(messages);
    let mut sentences = chain.generate();
    let mut msg = String::new();
    for _ in 1..rand::thread_rng().gen_range(2, 10) {
        msg = format!("{} {}", msg, sentences.pop().unwrap());
    }
    match api
        .send(message.text_reply(msg.trim()).parse_mode(ParseMode::Html))
        .await
    {
        Ok(_) => debug!("/markov_all command sent to {}", message.chat.id()),
        Err(_) => warn!("/markov_all command sent failed to {}", message.chat.id()),
    }
    //api.send(message.chat.text("Text to message chat")).await?;
    //api.send(message.from.text("Private text")).await?;
    Ok(())
}

pub(crate) async fn markov(api: Api, message: Message) -> Result<(), Error> {
    let messages = db::get_random_messages_group().await?;
    let mut chain = Chain::new();
    chain.feed(messages);
    let mut sentences = chain.generate();
    let mut msg = String::new();
    for _ in 1..rand::thread_rng().gen_range(2, 10) {
        msg = format!("{} {}", msg, sentences.pop().unwrap());
    }
    match api
        .send(message.text_reply(msg.trim()).parse_mode(ParseMode::Html))
        .await
    {
        Ok(_) => debug!("/markov command sent to {}", message.chat.id()),
        Err(_) => warn!("/markov command sent failed to {}", message.chat.id()),
    }
    //api.send(message.chat.text("Text to message chat")).await?;
    //api.send(message.from.text("Private text")).await?;
    Ok(())
}