extern crate core;

use std::{fs, time};
use std::thread::sleep;
use feed_rs::model::Entry;
use feed_rs::parser;

use serde_derive::{Deserialize, Serialize};
use toml::Value;
use toml::value::Array;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    feeds: Vec<Feed>,
    cache: Cache,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Cache {
    cache: Array,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Feed {
    url: Value,
    pfp: Value,
    webhook: Value,
}

struct WebhookEmbed {
    content: String,
    username: String,
    pfp: String,
}

fn main() {
    //println!("Hello, world!");
    //let a = fetch(String::from("https://feed.eugenemolotov.ru/?action=display&bridge=Twitter&context=By+username&u=energeticbark&format=Atom")).unwrap();
    //let a = parser::parse(a.as_bytes()).unwrap();
    //println!("{:#?}", a.entries[0]);

    for i in load_feeds() {
        let mut entries = parse_feed( fetch(i.url.as_str().unwrap().to_string()) );
        entries.drain(5..entries.len());

        for j in entries {
            if !is_cached(j.id.as_str()) {

                add_cache(j.id.as_str());
                let webhook = WebhookEmbed {
                    content: j.id.to_string(),
                    username: j.authors[0].name.clone().to_string(),
                    pfp: i.pfp.as_str().unwrap().to_string()
                };

                hook(i.webhook.as_str().unwrap(), webhook)
            }
            // DISCORD WILL KILL ME IF I DONT DO THIS
            sleep(time::Duration::from_secs(1));
        }






    }

    // hook(
    //     "https://discord.com/api/webhooks/976890175365988484/c0W89PbhrwafGsOHJYrbPOKyp31q_5h8E34jAnPEyJtgXW1iUW71Leh99V34aSoULg96",
    //     "its not actually a bridge its just im testing stuff :) im writing an rss thingy"
    // );

}

fn parse_feed(url: String) -> Vec<Entry> {
    let a = parser::parse(url.as_bytes());
    a.unwrap().entries
}

fn load_feeds() -> Vec<Feed> {
    let config = match fs::read_to_string("feeds.toml") {
        Ok(value) => value,
        Err(_) => unimplemented!(),
    };
    let config : Config = toml::from_str(config.as_str() ).unwrap();

    config.feeds.clone().to_vec()

    // Convert feeds to a vector, then replace it's contents from Values to Strings
    //config.feeds.to_vec().iter().map(|s| s.as_str().unwrap().to_string() ).collect()
}

fn fetch(url: String) -> String {
    let body: String = ureq::get(url.as_str())
        .call().unwrap()
        .into_string().unwrap();

    body
}

#[warn(dead_code)]
fn hook(url: &str, webhook: WebhookEmbed) {
    ureq::post(url)
        .send_json(ureq::json!({
            "avatar_url": webhook.pfp,
            "username": webhook.username,
            "content": format!("🚨 IM DEBUGGING PLEASE IGNORE LIBERAL {}", webhook.content)
      })).unwrap()
        .into_string().unwrap();

}

fn is_cached(value: &str) -> bool {
    let config = match fs::read_to_string("feeds.toml") {
        Ok(value) => value,
        Err(_) => unimplemented!(),
    };
    let config: Config = toml::from_str(config.as_str() ).unwrap();

    config.cache.cache.to_vec().contains(&toml::Value::from(value))
}
fn add_cache(value: &str) {
    let config = match fs::read_to_string("feeds.toml") {
        Ok(value) => value,
        Err(_) => unimplemented!(),
    };
    dbg!(&config);
    let mut config: Config = toml::from_str(config.as_str() ).unwrap();

    config.cache.cache.push(toml::Value::from(value));

    dbg!(&config);



    let string = toml::to_string(&config).unwrap();

    dbg!(&string);

    fs::write("feeds.toml", string.as_bytes()).expect("TODO: panic message");
}