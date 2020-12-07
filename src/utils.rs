use sha1::Sha1;
use std::fs::{create_dir as fs_create_dir, File};
use std::io::prelude::*;
use std::time::SystemTime;
use telegram_bot::*;
use uuid::Uuid;

use crate::db;
use crate::errors;
extern crate reqwest;
use serde_json::Value;
use subprocess::Exec;

pub(crate) fn get_title(message: &Message) -> String {
    match &message.chat {
        MessageChat::Supergroup(chat) => chat.title.clone(),
        MessageChat::Group(chat) => chat.title.clone(),
        MessageChat::Private(_) => format!("PRIVATE"),
        _ => "PRIVATE".to_string(),
    }
}

pub(crate) async fn create_dir(dir: &String) -> () {
    match fs_create_dir(dir) {
        Ok(_) => info!("Dir {} created.", dir),
        Err(_) => (),
    }
}

pub(crate) async fn unixtime() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub(crate) async fn get_files(
    api: Api,
    message: Message,
    token: String,
) -> Result<i32, errors::Error> {
    let mut file_count = 0;
    let file_type = match message.kind {
        MessageKind::Photo { .. } => "photo".to_string(),
        MessageKind::Document { .. } => "doc".to_string(),
        MessageKind::Voice { .. } => "voice".to_string(),
        MessageKind::Video { .. } => "video".to_string(),
        MessageKind::VideoNote { .. } => "video".to_string(),
        MessageKind::Sticker { .. } => "sticker".to_string(),
        _ => "docs".to_string(),
    };
    create_dir(&file_type).await;
    if let Some(files) = message.get_files() {
        let group_title = get_title(&message);
        let author = message.from.id;
        for file in files {
            file_count += 1;
            let uuid = Uuid::new_v4();
            match api.send(&file).await {
                Ok(api_response) => {
                    let url = format!(
                        "https://api.telegram.org/file/bot{}/{}",
                        token,
                        api_response.file_path.unwrap()
                    );
                    let file_response = reqwest::get(&url).await?;
                    let ext = {
                        file_response
                            .url()
                            .path_segments()
                            .and_then(|segments| segments.last())
                            .and_then(|name| if name.is_empty() { None } else { Some(name) })
                            .unwrap_or("tmp.bin")
                            .split('.')
                            .last()
                            .unwrap()
                    };
                    let path = format!("{}/{}_{}_{}.{}", file_type, group_title, author, uuid, ext);
                    let mut hasher = Sha1::new();
                    let content = file_response.bytes().await?;
                    hasher.update(&content);
                    let file_hash = hasher.digest().to_string();
                    match db::get_file(file_hash.clone()).await {
                        Ok(_) => {}
                        Err(_) => {
                            let mut dest = File::create(path.clone())?;
                            match dest.write(&content) {
                                Ok(_) => {}
                                Err(e) => panic!("IO Error: Couldn't save file: {:?}", e),
                            }
                        }
                    };
                    db::add_file(&message, path, file_hash).await?;
                }
                Err(e) => error!("Couldn't get file: {}", e),
            }
        }
    };
    Ok(file_count)
}

pub(crate) async fn stemming(message: &Message) -> Result<Vec<String>, errors::Error> {
    let mut words: Vec<String> = vec![];
    let proc = Exec::shell("mystem -d --format json -l");
    match proc
        .stdin(message.text().unwrap().as_str())
        .communicate()
        .unwrap()
        .read_string()
        .unwrap()
        .0
    {
        Some(line) => {
            let v: Vec<Value> = match serde_json::from_str(line.as_str()) {
                Ok(val) => val,
                Err(_) => return Ok(vec![]),
            };
            for i in v {
                words.push(i["analysis"][0]["lex"].to_string().replace("\"", ""));
            }
            words.retain(|x| x != "null");
            //debug!("Parsed words: {}.", words.join(", "));
        }
        None => return Ok(vec![]),
    };

    Ok(words)
}
