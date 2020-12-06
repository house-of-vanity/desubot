use reqwest::Client;
use sha1::Sha1;
use std::fs::{create_dir as fs_create_dir, File};
use std::io::prelude::*;
use std::path::Path;
use std::{env, io};
use telegram_bot::*;
use uuid::Uuid;

use crate::db;
use crate::errors;
extern crate reqwest;
use ascii::AsciiChar::{LineFeed, EOT};
use subprocess::{Exec, Popen, PopenConfig, Redirection};
use serde_json::Value;
//use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize)]
struct StemWord {

}


pub(crate) fn get_title(message: &Message) -> String {
    match &message.chat {
        MessageChat::Supergroup(chat) => chat.title.clone(),
        MessageChat::Group(chat) => chat.title.clone(),
        MessageChat::Private(chat) => "PRIVATE".to_string(),
        _ => "PRIVATE".to_string(),
    }
}

pub(crate) async fn create_dir(dir: &String) -> () {
    match fs_create_dir(dir) {
        Ok(_) => info!("Dir {} created.", dir),
        Err(_) => (),
    }
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
                    let mut file_response = reqwest::get(&url).await?;
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
                        Ok(_) => {
                            debug!("File {} exist", file_hash);
                        }
                        Err(_) => {
                            let mut dest = File::create(path.clone())?;
                            dest.write(&content);
                        }
                    };
                    db::add_file(&message, path, file_hash).await?;
                }
                Err(e) => warn!("Couldn't get file: {}", e),
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
            let mut v: Vec<Value> = serde_json::from_str(line.as_str())?;
            for i in v {
                words.push(i["analysis"][0]["lex"].to_string().replace("\"", ""));
            }
            words.retain(|x| x != "null");
            info!("{:?}", words);
        }
        None => return Ok(vec![]),
    };

    Ok(words)
}
