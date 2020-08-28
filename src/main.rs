#[macro_use]
extern crate log;

use anyhow::anyhow;
use jfs::Store;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    info!("Starting up");
    let mut count = 0u32;
    let client = reqwest::blocking::Client::new();
    let uri = "https://cat-fact.herokuapp.com/facts/random";
    let db: Store = Store::new("data")?;
    loop {
        count += 1;
        let response = client.get(uri).send()?;
        if response.status().is_client_error() || response.status().is_server_error() {
            return Err(anyhow!("Server responded with: {}", response.status()));
        }
        let string: CatFact = serde_json::from_str(&response.text()?)?;
        let key = db.save(&string)?;
        info!("Written one file with key: {}", key);
        thread::sleep(Duration::from_millis(5000));
        if count == 5 {
            break;
        } else {
            continue;
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct CatFact {
    used: bool,
    source: String,
    r#type: String,
    deleted: bool,
    _id: String,
    __v: i32,
    text: String,
    updatedAt: String,
    createdAt: String,
    status: Status,
    user: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    verified: bool,
    sentCount: i32
}