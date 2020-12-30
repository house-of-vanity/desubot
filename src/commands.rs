use crate::db;
use crate::errors::Error;
use async_trait::async_trait;
use html_escape::encode_text;
use markov::Chain;
use mystem::Case::Nominative;
use mystem::Gender::Feminine;
use mystem::Tense::{Inpresent, Past};
use mystem::VerbPerson::First;
use mystem::{MyStem, VerbPerson};
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Message, ParseMode};

pub struct Here {
    pub data: String,
}
pub struct Top {
    pub data: String,
}
pub struct MarkovAll {
    pub data: String,
}
pub struct Markov {
    pub data: String,
}
pub struct Omedeto {
    pub data: String,
}

// pub enum Command {
//     Here(Here),
//     Top { data: String },
//     MarkovAll { data: String },
//     Markov { data: String },
//     Omedeto { data: String },
// }

#[async_trait]
pub trait Execute {
    async fn run(&self, api: Api, message: Message) -> Result<(), Error>;
    async fn run_mystem(
        &self,
        api: Api,
        message: Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error>;
}

#[async_trait]
impl Execute for Here {
    async fn run(&self, api: Api, message: Message) -> Result<(), Error> {
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

    async fn run_mystem(
        &self,
        api: Api,
        message: Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for Top {
    async fn run(&self, api: Api, message: Message) -> Result<(), Error> {
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

    async fn run_mystem(
        &self,
        api: Api,
        message: Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for MarkovAll {
    async fn run(&self, api: Api, message: Message) -> Result<(), Error> {
        let messages = db::get_messages_random_all().await?;
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

    async fn run_mystem(
        &self,
        api: Api,
        message: Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for Markov {
    async fn run(&self, api: Api, message: Message) -> Result<(), Error> {
        let messages = db::get_messages_random_group(&message).await?;
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

    async fn run_mystem(
        &self,
        api: Api,
        message: Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for Omedeto {
    async fn run(&self, api: Api, message: Message) -> Result<(), Error> {
        unimplemented!()
    }

    async fn run_mystem(
        &self,
        api: Api,
        message: Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        let all_msg = db::get_messages_user_all(&message).await?;
        let re = Regex::new(r"^[яЯ] [а-яА-Я]+(-[а-яА-Я]+(_[а-яА-Я]+)*)*").unwrap();
        let mut nouns: Vec<String> = all_msg
            .clone()
            .into_iter()
            .filter(|m| re.is_match(m))
            .map(|m| m.split(' ').map(|s| s.to_string()).collect::<Vec<String>>()[1].clone())
            .filter(|m| {
                let stem = mystem.stemming(m.clone()).unwrap_or_default();
                if stem.is_empty() {
                    false
                } else if stem[0].lex.is_empty() {
                    false
                } else {
                    match stem[0].lex[0].grammem.part_of_speech {
                        mystem::PartOfSpeech::Noun => stem[0].lex[0]
                            .grammem
                            .facts
                            .contains(&mystem::Fact::Case(Nominative)),
                        _ => false,
                    }
                }
            })
            .map(|w| w.replace(|z| z == '.' || z == ',', ""))
            .collect();
        nouns.sort();
        nouns.dedup();
        nouns.shuffle(&mut rand::thread_rng());
        //debug!("Found {} nouns. {:#?}", nouns.len(), nouns);

        let mut verbs_p: Vec<String> = all_msg
            .clone()
            .into_iter()
            .filter(|m| re.is_match(m))
            .map(|m| m.split(' ').map(|s| s.to_string()).collect::<Vec<String>>()[1].clone())
            .filter(|m| {
                let stem = mystem.stemming(m.clone()).unwrap_or_default();
                if stem.is_empty() {
                    false
                } else if stem[0].lex.is_empty() {
                    false
                } else {
                    match stem[0].lex[0].grammem.part_of_speech {
                        mystem::PartOfSpeech::Verb => stem[0].lex[0]
                            .grammem
                            .facts
                            .contains(&mystem::Fact::Tense(Past)),
                        _ => false,
                    }
                }
            })
            .map(|w| w.replace(|z| z == '.' || z == ',', ""))
            .collect();
        verbs_p.sort();
        verbs_p.dedup();
        verbs_p.shuffle(&mut rand::thread_rng());
        //debug!("Found {} past verbs. {:#?}", verbs_p.len(), verbs_p);

        let mut verbs_i: Vec<String> = all_msg
            .clone()
            .into_iter()
            .filter(|m| re.is_match(m))
            .map(|m| m.split(' ').map(|s| s.to_string()).collect::<Vec<String>>()[1].clone())
            .filter(|m| {
                let stem = mystem.stemming(m.clone()).unwrap_or_default();
                if stem.is_empty() {
                    false
                } else if stem[0].lex.is_empty() {
                    false
                } else {
                    match stem[0].lex[0].grammem.part_of_speech {
                        mystem::PartOfSpeech::Verb => {
                            stem[0].lex[0]
                                .grammem
                                .facts
                                .contains(&mystem::Fact::Tense(Inpresent))
                                && stem[0].lex[0]
                                    .grammem
                                    .facts
                                    .contains(&mystem::Fact::Person(First))
                        }
                        _ => false,
                    }
                }
            })
            .map(|w| w.replace(|z| z == '.' || z == ',', ""))
            .collect();
        verbs_i.sort();
        verbs_i.dedup();
        verbs_i.shuffle(&mut rand::thread_rng());
        //debug!("Found {} inpresent verbs. {:#?}", verbs_i.len(), verbs_i);

        if nouns.is_empty() {
            nouns.push(message.from.first_name.to_string());
        }
        let start: Vec<String> = vec![
            "С новым годом".into(),
            "С НГ тебя".into(),
            "Поздравляю".into(),
            "Поздравляю с НГ".into(),
        ];
        let placeholders: Vec<String> = vec![
            "[ДАННЫЕ УДАЛЕНЫ]".into(),
            "[СЕКРЕТНО]".into(),
            "[НЕТ ДАННЫХ]".into(),
            "[ОШИБКА ДОСТУПА]".into(),
        ];
        //debug!("Nouns: {:#?}", nouns);
        //debug!("Verbs: {:#?}", verbs);

        let fem = {
            let mut fm = 0;
            let mut mu = 0;
            all_msg
                .clone()
                .into_iter()
                .filter(|m| re.is_match(m))
                .map(|m| m.split(' ').map(|s| s.to_string()).collect::<Vec<String>>()[1].clone())
                .map(|m| {
                    let stem = mystem.stemming(m.clone()).unwrap_or_default();
                    if stem.is_empty() {
                        ()
                    } else if stem[0].lex.is_empty() {
                        ()
                    } else {
                        match stem[0].lex[0].grammem.part_of_speech {
                            mystem::PartOfSpeech::Verb => {
                                match stem[0].lex[0]
                                    .grammem
                                    .facts
                                    .contains(&mystem::Fact::Tense(Past))
                                {
                                    true => {
                                        if stem[0].lex[0]
                                            .grammem
                                            .facts
                                            .contains(&mystem::Fact::Gender(Feminine))
                                        {
                                            fm = fm + 1;
                                        } else {
                                            mu = mu + 1;
                                        }
                                    }
                                    false => (),
                                }
                            }
                            _ => (),
                        }
                    }
                })
                .collect::<()>();
            //debug!("fm - {}, mu - {}", fm, mu);
            if fm >= mu {
                true
            } else {
                false
            }
        };
        //debug!("Is Feminine - {}", fem);
        let result = format!(
            "{} {} известн{} как {}, {}, а так же конечно {}. В прошедшем году ты часто давал{} нам знать, что ты {}, {} и {}. Нередко ты говорил{} я {}, я {} или даже я {}. =*",
            start.choose(&mut rand::thread_rng()).unwrap(),
            message.from.first_name.to_string(),
            { if fem { "ая" } else { "ый" } },
            nouns.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
            nouns.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
            nouns.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
            { if fem { "а" } else { "" } },
            verbs_p.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
            verbs_p.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
            verbs_p.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
            { if fem { "а" } else { "" } },
            verbs_i.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
            verbs_i.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
            verbs_i.pop().unwrap_or(placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()),
        );
        //debug!("{:?}", result);
        match api
            .send(
                message
                    .text_reply(result.trim())
                    .parse_mode(ParseMode::Html),
            )
            .await
        {
            Ok(_) => debug!("/omedeto command sent to {}", message.chat.id()),
            Err(_) => warn!("/omedeto command sent failed to {}", message.chat.id()),
        }
        Ok(())
    }
}
