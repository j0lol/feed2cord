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
    loop {

        for i in load_feeds() {
            let mut entries = parse_feed( fetch(i.url.as_str().unwrap().to_string()) );
            entries.drain(5..entries.len());

            for j in entries {
                let mut link = String::from("Placeholder");
                for k in j.links {
                    if k.rel == Some("alternate".to_string()) {
                        link = k.href.as_str().to_string()
                    }
                }
                if link == String::from("Placeholder") {
                    unimplemented!("Weird atom feed?")
                }

                                println!("Processing {}", link);

                if !is_cached(link.as_str()) {

                    add_cache(link.as_str());
                    let webhook = WebhookEmbed {
                        content: link,
                        username: j.authors[0].name.clone().to_string(),
                        pfp: i.pfp.as_str().unwrap().to_string()
                    };

                    sleep(time::Duration::from_secs(1));
                    hook(i.webhook.as_str().unwrap(), webhook)
                }

            }
        }

    sleep(time::Duration::from_secs(300));


    }
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
}

fn fetch(url: String) -> String {
    let body: String = ureq::get(url.as_str())
        .call().unwrap()
        .into_string().unwrap();

    body
}

fn hook(url: &str, webhook: WebhookEmbed) {
    ureq::post(url)
        .send_json(ureq::json!({
            "avatar_url": webhook.pfp,
            "username": webhook.username,
            "content": format!("🚨 {}", webhook.content)
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

    let mut config: Config = toml::from_str(config.as_str() ).unwrap();

    config.cache.cache.push(toml::Value::from(value));

    let string = toml::to_string(&config).unwrap();

    fs::write("feeds.toml", string.as_bytes()).expect("TODO: panic message");
}
