#![allow(unused_variables)]
use crate::db;
use crate::errors::Error;
use crate::errors::Error::{SQLITE3Error, SQLInvalidCommand};
use async_trait::async_trait;
use html_escape::encode_text;
use markov::Chain;
use mystem::Case::Nominative;
use mystem::Gender::Feminine;
use mystem::MyStem;
use mystem::Person::First;
use mystem::Tense::{Inpresent, Past};
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;
use silicon;
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
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
pub struct Sql {
    pub data: String,
}

pub struct Code {
    pub data: String,
}

#[async_trait]
pub trait Execute {
    async fn exec(&self, api: &Api, message: &Message) -> Result<(), Error>;
    async fn exec_with_result(&self, api: &Api, message: &Message) -> Result<String, Error>;
    async fn exec_mystem(
        &self,
        api: &Api,
        message: &Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error>;
}

#[async_trait]
impl Execute for Sql {
    async fn exec(&self, api: &Api, message: &Message) -> Result<(), Error> {
        unimplemented!()
    }

    async fn exec_with_result(&self, api: &Api, message: &Message) -> Result<String, Error> {
        let mut sql = self.data.clone();
        let is_head = if sql.starts_with('-') {
            sql = sql.replacen("-", "", 1);
            false
        } else {
            true
        };
        let dialect = GenericDialect {};
        let ast: Vec<Statement> = match Parser::parse_sql(&dialect, &sql) {
            Ok(ast) => ast,
            Err(_) => {
                warn!("Invalid SQL - {}", sql);
                return Err(SQLInvalidCommand);
            }
        };
        match ast.len() {
            l if l > 1 => {
                return Err(Error::SQLBannedCommand(
                    "🚫 One statement per message allowed 🚫".into(),
                ))
            }
            _ => (),
        }
        match ast[0] {
            sqlparser::ast::Statement::Query { .. } => {}
            _ => {
                return Err(Error::SQLBannedCommand(
                    "🚫 SELECT requests allowed only 🚫".into(),
                ))
            }
        }

        let conn = db::open()?;
        let mut stmt = conn.prepare_cached(&sql)?;

        let mut rows = match stmt.query(rusqlite::NO_PARAMS) {
            Err(e) => return Err(SQLITE3Error(e)),
            Ok(rows) => rows,
        };

        let mut res: Vec<Vec<String>> = match rows.column_names() {
            Some(n) => vec![n
                .into_iter()
                .map(|s| {
                    let t = String::from(s);
                    if t.len() > 10 {
                        "EMSGSIZE".to_string()
                    } else {
                        t
                    }
                })
                .collect()],
            None => return Err(SQLInvalidCommand),
        };

        let index_count = match rows.column_count() {
            Some(c) => c,
            None => return Err(SQLInvalidCommand),
        };
        while let Some(row) = rows.next().unwrap() {
            let mut tmp: Vec<String> = Vec::new();
            for i in 0..index_count {
                match row.get(i).unwrap_or(None) {
                    Some(rusqlite::types::Value::Text(t)) => tmp.push(t),
                    Some(rusqlite::types::Value::Integer(t)) => tmp.push(t.to_string()),
                    Some(rusqlite::types::Value::Blob(_)) => tmp.push("Binary".to_string()),
                    Some(rusqlite::types::Value::Real(t)) => tmp.push(t.to_string()),
                    Some(rusqlite::types::Value::Null) => tmp.push("Null".to_string()),
                    None => tmp.push("Null".to_string()),
                };
            }
            res.push(tmp);
        }
        if res.len() > 100 {
            return Err(Error::SQLResultTooLong(
                "SQL result too long. Lines limit is 100. Use LIMIT".to_string(),
            ));
        }
        // add Header
        let mut msg = if is_head {
            let mut x = String::from("<b>");
            for head in res[0].iter() {
                x = format!("{} {}", x, head);
            }
            format!("{}{}", x, "</b>\n")
        } else {
            String::new()
        };

        // remove header
        res.remove(0);

        msg = format!("{}{}", msg, "<pre>");
        for line in res.iter() {
            for field in line.iter() {
                msg = format!("{}{}", msg, format!("{} ", field));
            }
            msg = format!("{}{}", msg, "\n");
        }
        msg = format!("{}{}", msg, "</pre>");
        msg = if msg.len() > 4096 {
            "🚫 Result is too big. Use LIMIT 🚫".into()
        } else {
            msg
        };
        Ok(msg)
    }

    #[allow(unused_variables)]
    async fn exec_mystem(
        &self,
        api: &Api,
        message: &Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for Here {
    async fn exec(&self, api: &Api, message: &Message) -> Result<(), Error> {
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
        Ok(())
    }

    async fn exec_with_result(&self, api: &Api, message: &Message) -> Result<String, Error> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn exec_mystem(
        &self,
        api: &Api,
        message: &Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for Top {
    async fn exec(&self, api: &Api, message: &Message) -> Result<(), Error> {
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
        Ok(())
    }

    async fn exec_with_result(&self, api: &Api, message: &Message) -> Result<String, Error> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn exec_mystem(
        &self,
        api: &Api,
        message: &Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for MarkovAll {
    async fn exec(&self, api: &Api, message: &Message) -> Result<(), Error> {
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

    async fn exec_with_result(&self, api: &Api, message: &Message) -> Result<String, Error> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn exec_mystem(
        &self,
        api: &Api,
        message: &Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for Markov {
    async fn exec(&self, api: &Api, message: &Message) -> Result<(), Error> {
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

    async fn exec_with_result(&self, api: &Api, message: &Message) -> Result<String, Error> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn exec_mystem(
        &self,
        api: &Api,
        message: &Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Execute for Omedeto {
    #[allow(unused_variables)]
    async fn exec(&self, api: &Api, message: &Message) -> Result<(), Error> {
        unimplemented!()
    }

    async fn exec_with_result(&self, api: &Api, message: &Message) -> Result<String, Error> {
        unimplemented!()
    }

    #[warn(unused_must_use)]
    async fn exec_mystem(
        &self,
        api: &Api,
        message: &Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        let all_msg = db::get_messages_user_all(&message).await?;
        let re = Regex::new(r"^[яЯ] [а-яА-Я]+(-[а-яА-Я]+(_[а-яА-Я]+)*)*").unwrap();
        let mut nouns: Vec<String> = all_msg
            .iter()
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
            .iter()
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
            .iter()
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
                .for_each(drop);
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

#[async_trait]
impl Execute for Code {
    async fn exec(&self, api: &Api, message: &Message) -> Result<(), Error> {
        let lang = "Rust";
        let theme = "Dracula";
        let code = &self.data;
        let (ps, ts) = silicon::utils::init_syntect();
        let syntax = ps
            .find_syntax_by_token(lang)
            .ok_or_else(|| ps.find_syntax_by_token("js"));
        let theme = ts.themes.get(theme).ok_or_else(|| ts.themes.get("1337"));
        debug!("{:?}", code);
        Ok(())
    }

    async fn exec_with_result(&self, api: &Api, message: &Message) -> Result<String, Error> {
        unimplemented!()
    }

    async fn exec_mystem(
        &self,
        api: &Api,
        message: &Message,
        mystem: &mut MyStem,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}
