use crate::errors;
use crate::utils;
use rusqlite::{named_params, params, Connection, Error, Result};
use sha1::{Digest, Sha1};
use std::time::SystemTime;
use telegram_bot::*;

#[derive(Debug, Clone)]
pub struct Conf {
    id: telegram_bot::ChatId,
    title: String,
    date: i32,
}

pub(crate) fn open() -> Result<Connection> {
    let path = "./memory.sqlite3";
    let db = Connection::open(&path)?;
    Ok(db)
}

pub(crate) fn get_user(id: telegram_bot::UserId) -> Result<telegram_bot::User, errors::Error> {
    let conn = open()?;
    let mut stmt =
        conn.prepare("SELECT id, username, first_name, last_name, date FROM user WHERE id = :id")?;

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

    if users.len() == 0 {
        Err(errors::Error::UserNotFound)
    } else {
        Ok(users[0].clone())
    }
}

pub(crate) fn get_conf(id: telegram_bot::ChatId) -> Result<Conf, errors::Error> {
    let conn = open()?;
    let mut stmt = conn.prepare("SELECT id, title, date FROM conf WHERE id = :id")?;

    let mut rows = stmt.query_named(&[(":id", &id.to_string())])?;
    let mut confs = Vec::new();

    while let Some(row) = rows.next()? {
        confs.push(Conf {
            id: telegram_bot::ChatId::new(row.get(0)?),
            title: row.get(1)?,
            date: row.get(2)?,
        })
    }
    //println!("Confs: {:?}", confs);

    if confs.len() == 0 {
        Err(errors::Error::ConfNotFound)
    } else {
        Ok(confs[0].clone())
    }
}

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

pub(crate) fn get_members(id: telegram_bot::ChatId) -> Result<Vec<telegram_bot::User>> {
    let conn = open()?;
    let mut stmt = conn.prepare(
        "
        SELECT DISTINCT(u.username), u.id, u.first_name, u.last_name, u.date
        FROM relations r
        JOIN user u
        ON u.id = r.user_id
        LEFT JOIN conf c
        ON r.conf_id = c.id
        WHERE c.id = :id",
    )?;
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
        Ok(conf) => {
            let update = Conf {
                id: message.chat.id(),
                title,
                date: 0,
            };
            let mut stmt = conn.prepare(
                "UPDATE conf
                SET
                    title = :title
                WHERE
                id = :id",
            )?;
            stmt.execute_named(&[(":id", &update.id.to_string()), (":title", &update.title)])?;
            //println!("Conf {:?} updated: {:?}", update.title, get_conf(update.id));
        }
        Err(e) => {
            let update = Conf {
                id: message.chat.id(),
                title,
                date: 0,
            };
            let unix_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let mut stmt = conn.prepare(
                "UPDATE conf
                SET
                    title = :title,
                    date = :date
                WHERE
                id = :id",
            )?;
            stmt.execute_named(&[
                (":id", &update.id.to_string()),
                (":title", &update.title),
                (":date", &unix_time),
            ])?;
            //println!("Conf {:?} added: {:?}", update.title, get_conf(update.id));
        }
        _ => {}
    }
    Ok(())
}

pub(crate) async fn add_user(message: Message) -> Result<(), Error> {
    let conn = open()?;
    match get_user(message.from.id) {
        Ok(user) => {
            let update = telegram_bot::User {
                id: message.from.id,
                first_name: message.from.first_name,
                last_name: message.from.last_name,
                username: message.from.username,
                is_bot: false,
                language_code: None,
            };
            let mut stmt = conn.prepare(
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
            //println!("User {} updated: {:?}", update.first_name, get_user(user.id));
        }
        Err(e) => {
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
            let mut stmt = conn.prepare(
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
            //println!("User added: {:?}", user);
        }
        _ => {}
    }
    Ok(())
}

pub(crate) async fn add_file(
    message: &Message,
    path: String,
    file_id: String,
) -> Result<(), Error> {
    let conn = open()?;
    let mut stmt = conn.prepare(
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

pub(crate) async fn get_file(file_id: String) -> Result<(), errors::Error> {
    let conn = open()?;
    let mut stmt = conn.prepare("SELECT path FROM file WHERE file_id = :file_id")?;
    let mut rows = stmt.query_named(&[(":file_id", &file_id)])?;
    let mut files = Vec::new();

    while let Some(row) = rows.next()? {
        files.push("should be rewritten");
    }
    if files.len() > 0 {
        Ok(())
    } else {
        Err(errors::Error::FileNotFound)
    }
}
