use rusqlite::{named_params, params, Connection, Result};
use telegram_bot::*;

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    username: Option<String>,
    first_name: String,
    last_name: Option<String>,
    date: i32,
}

#[derive(Debug, Clone)]
pub struct Conf {
    id: i64,
    title: Option<String>,
    date: i32,
}

pub(crate) fn open() -> Result<Connection> {
    let path = "./memory.sqlite3";
    let db = Connection::open(&path)?;
    Ok(db)
}

pub(crate) fn get_user(id: i64) -> Result<telegram_bot::User> {
    let conn = open()?;
    let mut stmt =
        conn.prepare("SELECT id, username, first_name, last_name, date FROM user WHERE id = :id")?;

    let mut rows = stmt.query_named(&[(":id", &id)])?;
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

    Ok(users[0].clone())
}

pub(crate) fn get_confs() -> Result<Vec<Conf>> {
    let conn = open()?;
    let mut stmt = conn.prepare("SELECT id, title, date FROM conf")?;

    let mut rows = stmt.query(params![])?;
    let mut confs = Vec::new();

    while let Some(row) = rows.next()? {
        confs.push(Conf {
            id: row.get(0)?,
            title: row.get(1)?,
            date: row.get(2)?,
        })
    }
    println!("Confs: {:?}", confs);

    Ok(confs)
}

pub(crate) fn get_members(id: &telegram_bot::ChatId) -> Result<Vec<telegram_bot::User>> {
    let conn = open()?;
    let str_id = id.to_string();
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
    let mut rows = stmt.query_named(&[(":id", &str_id)])?;
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
