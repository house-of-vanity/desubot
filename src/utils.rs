use telegram_bot::*;

pub(crate) fn get_title(message: &telegram_bot::Message) -> String {
    match &message.chat {
        MessageChat::Supergroup(chat) => chat.title.clone(),
        MessageChat::Group(chat) => chat.title.clone(),
        MessageChat::Private(chat) => "PRIVATE".to_string(),
        _ => "PRIVATE".to_string()
    }
}