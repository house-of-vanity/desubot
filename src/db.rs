#[allow(unused_mut)]
#[allow(dead_code)]
use crate::errors;
use crate::utils;
use rusqlite::{named_params, params, Connection, Error, Result};
use std::time::SystemTime;
use telegram_bot::*;

#[derive(Debug, Clone)]
pub struct Conf {
    id: telegram_bot::ChatId,
    title: String,
    date: i32,
}
#[derive(Debug, Clone)]
pub struct TopWord {
    pub word: String,
    pub count: i32,
}

pub(crate) fn open() -> Result<Connection> {
    let path = "./memory.sqlite3";
    let db = Connection::open(&path)?;
    Ok(db)
}

pub(crate) fn update_scheme() -> Result<()> {
    let conn = open()?;
    for table in include_str!("../assets/scheme.sql").split(';').into_iter() {
        let t = table.trim();
        if t != "" {
            conn.execute(t, params![])?;
        }
    }
    info!("Database schema updated.");
    Ok(())
}

pub(crate) fn load_stopwords() -> Result<()> {
    info!("Populating stop words wait please.");
    let conn = open()?;
    for table in include_str!("../assets/stop-words.txt")
        .split('\n')
        .into_iter()
    {
        let word = table.trim();
        if word != "" {
            let mut _stmt = conn
                .prepare_cached(
                    "
                    INSERT OR IGNORE INTO
                                    stop_words('word')
                                    VALUES (:word)
                    ",
                )?
                .insert(params![word]);
        }
    }
    info!("Stop words updated.");
    Ok(())
}

pub(crate) fn get_user(id: telegram_bot::UserId) -> Result<telegram_bot::User, errors::Error> {
    let conn = open()?;
    let mut stmt = conn.prepare_cached(
        "SELECT id, username, first_name, last_name, date FROM user WHERE id = :id",
    )?;

    let mut rows = stmt.query_named(&[(":id", &id.to_string())])?;
    let mut users = Vec::new();

    while let Some(row) = rows.next()? {
        users.push(telegram_bot::User {
            id: UserId::new(row.get(0)?),
            username: row.get(1)?,
            is_bot: false,
            first_name: row.get(2)?,
            last_name: row.get(3)?,
            language_code: None,
        })
    }

    if users.is_empty() {
        Err(errors::Error::UserNotFound)
    } else {
        Ok(users[0].clone())
    }
}

pub(crate) fn get_conf(id: telegram_bot::ChatId) -> Result<Conf, errors::Error> {
    let conn = open()?;
    let mut stmt = conn.prepare_cached("SELECT id, title, date FROM conf WHERE id = :id")?;

    let mut rows = stmt.query_named(&[(":id", &id.to_string())])?;
    let mut confs = Vec::new();
    while let Some(row) = rows.next()? {
        confs.push(Conf {
            id: telegram_bot::ChatId::new(row.get(0)?),
            title: row.get(1)?,
            date: row.get(2)?,
        })
    }
    if confs.is_empty() {
        Err(errors::Error::ConfNotFound)
    } else {
        Ok(confs[0].clone())
    }
}

#[allow(dead_code)]
pub(crate) fn get_confs() -> Result<Vec<Conf>> {
    let conn = open()?;
    let mut stmt = conn.prepare("SELECT id, title, date FROM conf")?;

    let mut rows = stmt.query(params![])?;
    let mut confs = Vec::new();

    while let Some(row) = rows.next()? {
        confs.push(Conf {
            id: telegram_bot::ChatId::new(row.get(0)?),
            title: row.get(1)?,
            date: row.get(2)?,
        })
    }
    //println!("Confs: {:?}", confs);

    Ok(confs)
}

pub(crate) async fn get_messages_random_all() -> Result<Vec<String>, Error> {
    let conn = open()?;
    let mut stmt = conn.prepare_cached("SELECT text FROM messages ORDER BY RANDOM() LIMIT 50")?;
    let mut rows = stmt.query_named(named_params![])?;
    let mut messages = Vec::new();

    while let Some(row) = rows.next()? {
        messages.push(row.get(0)?)
    }
    Ok(messages)
}

pub(crate) async fn get_messages_random_group(
    message: &telegram_bot::Message,
) -> Result<Vec<String>, Error> {
    let conf_id = i64::from(message.chat.id());
    let conn = open()?;
    let mut stmt = conn.prepare_cached(
        "
    SELECT m.text FROM messages m
    LEFT JOIN relations r ON r.msg_id = m.id
    WHERE r.conf_id = :conf_id
    ORDER BY RANDOM() LIMIT 50
    ",
    )?;
    let mut rows = stmt.query_named(named_params! {":conf_id": conf_id})?;
    let mut messages = Vec::new();

    while let Some(row) = rows.next()? {
        messages.push(row.get(0)?)
    }
    Ok(messages)
}

#[allow(dead_code)]
pub(crate) async fn get_messages_user_group(
    message: &telegram_bot::Message,
) -> Result<Vec<String>, Error> {
    let conf_id = i64::from(message.chat.id());
    let user_id = i64::from(message.from.id);
    let conn = open()?;
    let mut stmt = conn.prepare_cached(
        "
    SELECT m.text FROM messages m
    LEFT JOIN relations r ON r.msg_id = m.id
    WHERE r.conf_id = :conf_id
    AND r.user_id = :user_id
    ",
    )?;
    let mut rows = stmt.query_named(named_params! {":conf_id": conf_id, ":user_id": user_id})?;
    let mut messages = Vec::new();

    while let Some(row) = rows.next()? {
        messages.push(row.get(0)?)
    }
    Ok(messages)
}

pub(crate) async fn get_messages_user_all(
    message: &telegram_bot::Message,
) -> Result<Vec<String>, Error> {
    let user_id = i64::from(message.from.id);
    let conn = open()?;
    let mut stmt = conn.prepare_cached(
        "
    SELECT m.text FROM messages m
    LEFT JOIN relations r ON r.msg_id = m.id
    WHERE r.user_id = :user_id
    ",
    )?;
    let mut rows = stmt.query_named(named_params! {":user_id": user_id})?;
    let mut messages = Vec::new();

    while let Some(row) = rows.next()? {
        messages.push(row.get(0)?)
    }
    Ok(messages)
}

pub(crate) fn get_members(id: telegram_bot::ChatId, limit: u32) -> Result<Vec<telegram_bot::User>> {
    let where_statement = if limit > 0 {
        format!("and days_seen <= {}", limit)
    } else {
        "".into()
    };
    debug!("{}", where_statement);
    let conn = open()?;
    let mut stmt = conn.prepare_cached(&format!(
        "
        SELECT DISTINCT(u.username), u.id, u.first_name, u.last_name, u.date,
        (strftime('%s','now')-r.date)/60/60/24 as days_seen
        FROM relations r
        JOIN user u
        ON u.id = r.user_id
        LEFT JOIN conf c
        ON r.conf_id = c.id
        WHERE c.id = :id
        {}
        GROUP BY u.id
        ORDER BY r.date DESC",
        where_statement
    ))?;
    let mut rows = stmt.query_named(&[(":id", &id.to_string())])?;
    let mut users = Vec::new();

    while let Some(row) = rows.next()? {
        users.push(telegram_bot::User {
            id: UserId::new(row.get(1)?),
            username: row.get(0)?,
            is_bot: false,
            first_name: row.get(2)?,
            last_name: row.get(3)?,
            language_code: None,
        })
    }
    Ok(users)
}

pub(crate) async fn add_conf(message: Message) -> Result<(), Error> {
    let conn = open()?;
    let title = utils::get_title(&message);

    match get_conf(message.chat.id()) {
        Ok(_) => {
            debug!("Group found: {:?}", message.chat.id());
            let update = Conf {
                id: message.chat.id(),
                title,
                date: 0,
            };
            let mut stmt = conn.prepare_cached(
                "UPDATE conf
                SET
                    title = :title
                WHERE
                id = :id",
            )?;
            stmt.execute_named(&[(":id", &update.id.to_string()), (":title", &update.title)])?;
            debug!("Conf {:?} updated: {:?}", update.title, get_conf(update.id));
        }
        Err(_) => {
            debug!("Group didn't found: {:?}", message.chat.id());

            let update = Conf {
                id: message.chat.id(),
                title,
                date: 0,
            };
            let unix_time = utils::unixtime().await;

            let mut stmt = conn.prepare_cached(
                "INSERT OR IGNORE INTO
                    conf('id', 'title', 'date')
                    VALUES (:id, :title, :date)",
            )?;
            stmt.execute_named(&[
                (":id", &update.id.to_string()),
                (":title", &update.title),
                (":date", &unix_time),
            ])?;
            debug!("Conf {:?} added: {:?}", update.title, get_conf(update.id));
        }
    }
    Ok(())
}

pub(crate) async fn add_user(message: Message) -> Result<(), Error> {
    let conn = open()?;
    match get_user(message.from.id) {
        Ok(_) => {
            let update = telegram_bot::User {
                id: message.from.id,
                first_name: message.from.first_name,
                last_name: message.from.last_name,
                username: message.from.username,
                is_bot: false,
                language_code: None,
            };
            let mut stmt = conn.prepare_cached(
                "UPDATE user
                SET
                    username = :username,
                    first_name = :first_name,
                    last_name = :last_name
                WHERE
                id = :id",
            )?;
            stmt.execute_named(&[
                (":id", &update.id.to_string()),
                (":username", &update.username),
                (":first_name", &update.first_name),
                (":last_name", &update.last_name),
            ])?;
            debug!(
                "User {} updated: {:?}",
                update.first_name,
                get_user(update.id)
            );
        }
        Err(_) => {
            let unix_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            let user = telegram_bot::User {
                id: message.from.id,
                first_name: message.from.first_name,
                last_name: message.from.last_name,
                username: message.from.username,
                is_bot: false,
                language_code: None,
            };
            let mut stmt = conn.prepare_cached(
                "INSERT OR IGNORE INTO
                user('id', 'username', 'first_name', 'last_name', 'date')
                VALUES (:id, :username, :first_name, :last_name, :date)",
            )?;
            stmt.execute_named(&[
                (":id", &user.id.to_string()),
                (":username", &user.username),
                (":first_name", &user.first_name),
                (":last_name", &user.last_name),
                (":date", &unix_time),
            ])?;
            debug!("User added: {:?}", user);
        }
    }
    Ok(())
}

pub(crate) async fn add_file(
    message: &Message,
    path: String,
    file_id: String,
) -> Result<(), Error> {
    let conn = open()?;
    let mut stmt = conn.prepare_cached(
        "INSERT OR IGNORE INTO
                file('path', 'user_id', 'conf_id', 'file_id')
                VALUES (:path, :user_id, :conf_id, :file_id)",
    )?;
    stmt.execute_named(&[
        (":path", &path),
        (":user_id", &message.from.id.to_string()),
        (":conf_id", &message.chat.id().to_string()),
        (":file_id", &file_id),
    ])?;
    Ok(())
}

pub(crate) async fn get_file(file_id: String) -> Result<i64, errors::Error> {
    let conn = open()?;
    let file_rowid =
        match { conn.prepare_cached("SELECT rowid FROM file WHERE file_id = :file_id")? }
            .query_row(params![file_id], |row| row.get(0))
        {
            Ok(id) => Ok(id),
            Err(_) => Err(errors::Error::FileNotFound),
        };
    file_rowid
}

async fn add_word(word: &str) -> Result<i64, errors::Error> {
    if get_stop_word(&word).await.is_err() {
        return Err(errors::Error::WordInStopList);
    }
    let conn = open()?;
    let word_rowid =
        match { conn.prepare_cached("INSERT OR IGNORE INTO word('word') VALUES (:word)")? }
            .insert(params![word])
        {
            Ok(id) => id,
            Err(_) => { conn.prepare_cached("SELECT rowid FROM word WHERE word = (:word)")? }
                .query_row(params![word], |row| row.get(0))?,
        };
    Ok(word_rowid)
}

async fn get_stop_word(stop_word: &str) -> Result<(), errors::Error> {
    let conn = open()?;
    match conn.execute_named(
        "SELECT rowid FROM stop_words WHERE word = (:stop_word)",
        &[(":stop_word", &stop_word)],
    ) {
        Ok(i) => match i {
            0 => Ok(()),
            _ => Err(errors::Error::WordNotFound),
        },
        Err(e) => Err(errors::Error::SQLITE3Error(e)),
    }
}

async fn add_relation(word_id: i64, msg_id: i64, message: &Message) -> Result<i64, errors::Error> {
    let user_id = i64::from(message.from.id);
    let conf_id = i64::from(message.chat.id());
    let unix_time = utils::unixtime().await;
    let conn = open()?;
    let rowid = match {
        conn.prepare_cached(
            "INSERT OR IGNORE INTO
        relations('word_id', 'user_id', 'conf_id', 'msg_id', 'date')
        VALUES (:word_id, :user_id, :conf_id, :msg_id, :date)",
        )?
    }
    .insert(params![word_id, user_id, conf_id, msg_id, unix_time])
    {
        Ok(id) => id,
        Err(e) => return Err(errors::Error::SQLITE3Error(e)),
    };
    Ok(rowid)
}

#[allow(unused_must_use)]
pub(crate) async fn add_sentence(
    message: &telegram_bot::Message,
    mystem: &mut mystem::MyStem,
) -> Result<(), errors::Error> {
    let text = message.text().unwrap();
    let conn = open()?;

    // Save sentence
    let msg_rowid =
        match { conn.prepare_cached("INSERT OR IGNORE INTO messages('text') VALUES (:text)")? }
            .insert(params![text])
        {
            Ok(id) => id,
            Err(_) => { conn.prepare_cached("SELECT rowid FROM messages WHERE text = (:text)")? }
                .query_row(params![text], |row| row.get(0))?,
        };

    // Save stemmed words
    debug!("Going to stem: {}", text);
    let words = mystem.stemming(text)?;
    conn.execute("BEGIN TRANSACTION", params![]);
    for word in words {
        if word.lex.is_empty() {
            continue;
        }
        match add_word(&word.lex[0].lex).await {
            Ok(id) => {
                debug!("Added {}: rowid: {}", &word.lex[0].lex, id);
                match add_relation(id, msg_rowid, message).await {
                    Ok(_) => {}
                    Err(e) => panic!("SQLITE3 Error: Relations failed: {:?}", e),
                }
            }
            Err(_) => debug!("Word {} is in a stop list.", &word.lex[0].lex),
        }
    }
    conn.execute("END TRANSACTION", params![]);
    Ok(())
}

pub(crate) async fn get_top(
    message: &telegram_bot::Message,
) -> Result<Vec<TopWord>, errors::Error> {
    let user_id = i64::from(message.from.id);
    let conf_id = i64::from(message.chat.id());

    let conn = open()?;
    let mut stmt = conn.prepare_cached("
        SELECT w.word, COUNT(*) as count FROM relations r
        LEFT JOIN word w ON w.id = r.word_id
        LEFT JOIN `user` u ON u.id = r.user_id
        WHERE u.id = :user_id AND
        r.conf_id = :conf_id AND
        r.id > (
            SELECT IFNULL(MAX(relation_id), 0) FROM reset WHERE user_id = :user_id AND conf_id = :conf_id
        )
        GROUP BY w.word
        ORDER BY count DESC
        LIMIT 10
    ")?;

    let mut rows = stmt.query_named(named_params! {":user_id": user_id, ":conf_id": conf_id})?;
    let mut top = Vec::new();

    while let Some(row) = rows.next()? {
        top.push(TopWord {
            word: row.get(0)?,
            count: row.get(1)?,
        })
    }
    Ok(top)
}
