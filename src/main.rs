use std::{error::Error, path::Path, str::FromStr};
mod bot;
use chrono::NaiveDate;
use rand::Rng;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serenity::{
    all::{Context, EventHandler, GatewayIntents, Message},
    async_trait, Client,
};
use sillybot::ErrorResult;
use tokio::fs;


#[derive(Debug, Serialize, Deserialize)]
struct Config {
    token: String,
}
struct Book {
    rowid: i32,
    id: u32,
    name: String,
    author: String,
    publish_date: NaiveDate,
}

async fn read_config(path_ref: impl AsRef<Path>) -> ErrorResult<Config> {
    let path = path_ref.as_ref();
    Ok(if !path.is_file() {
        let config = Config {
            token: "".to_string(),
        };
        let ser = toml::to_string(&config)?;
        fs::write(path, ser).await?;
        config
    } else {
        let deser = fs::read_to_string(path).await?;
        toml::from_str(&deser)?
    })
}

async fn sql() -> ErrorResult {
    let conn = Connection::open_in_memory()?;
    let mut rnd = rand::thread_rng();
    conn.execute(
        "create table if not exists Books (
        ID int unsigned primary key unique default 0,
        NAME text not null,
        AUTHOR text not null,...
        PUBLISH_DATE date not null
    )",
        (),
    )?;
    let name = "test";
    let publish_date = NaiveDate::from_ymd_opt(2024, 10, 7).unwrap().to_string();
    let author = "test";
    let id = rnd.gen::<u32>();

    conn.execute(
        "insert into Books (ID,NAME,AUTHOR,PUBLISH_DATE) values (?4,?1,?2,?3)",
        params![name, author, publish_date, id],
    )?;
    let name = "test1";
    let publish_date = NaiveDate::from_ymd_opt(2023, 10, 7).unwrap().to_string();
    let author = "test1";
    let id = rnd.gen::<u32>();
    conn.execute(
        "insert into Books (ID,NAME,AUTHOR,PUBLISH_DATE) values (?4,?1,?2,?3)",
        params![name, author, publish_date, id],
    )?;

    let mut query = conn.prepare("select ID,NAME,AUTHOR,PUBLISH_DATE,ROWID from Books")?;
    let vec: Vec<Book> = query
        .query_map((), |row| {
            let date: String = row.get_unwrap(3);
            Ok(Book {
                rowid: row.get(4)?,
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                publish_date: NaiveDate::from_str(date.as_str()).unwrap(),
            })
        })
        .unwrap()
        .map(|b| b.unwrap())
        .collect();
    for book in vec {
        println!(
            "ID: {} | NAME: {} AUTHOR: {} | PUBLISH_DATE: {} | ROWID: {}",
            book.id, book.name, book.author, book.publish_date, book.rowid
        )
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ErrorResult {
    bot::run_bot().await?;
    sql().await?;
    Ok(())
}
