use feed_rs::model::Entry;
use feed_rs::parser;
use std::thread::sleep;
use std::{fs, time};

use serde_derive::{Deserialize, Serialize};
use toml::value::Array;
use toml::Value;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    feeds: Vec<Feed>,
    cache: Cache,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Cache {
    cache: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Feed {
    url: String,
    pfp: String,
    webhook: String,
}

struct WebhookEmbed {
    content: String,
    username: String,
    pfp: String,
}

fn main() {
    for i in load_feeds() {
        let mut entries = parse_feed(fetch(i.url));
        if entries.len() > 5 {
            entries.drain(5..entries.len());
        }

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
                dbg!(&j.authors);

                add_cache(link.as_str());
                let webhook = WebhookEmbed {
                    content: link,
                    username: j.authors[0].name.clone().to_string(),
                    pfp: i.pfp.clone(),
                };

                sleep(time::Duration::from_secs(1));
                hook(i.webhook.clone(), webhook)
            }
        }
    }
}

fn parse_feed(url: String) -> Vec<Entry> {
    let a = parser::parse(url.as_bytes());
    a.unwrap().entries
}

fn load_feeds() -> Vec<Feed> {
    let config = match fs::read_to_string("../feeds.toml") {
        Ok(value) => value,
        Err(_) => unimplemented!(),
    };
    let config: Config = toml::from_str(config.as_str()).unwrap();

    config.feeds
}

fn fetch(url: String) -> String {
    let body: String = ureq::get(url.as_str())
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    body
}

fn hook(url: String, webhook: WebhookEmbed) {
    ureq::post(&url)
        .send_json(ureq::json!({
              "avatar_url": webhook.pfp,
              "username": webhook.username,
              "content": format!("🚨 {}", webhook.content)
        }))
        .unwrap()
        .into_string()
        .unwrap();
}

fn is_cached(value: &str) -> bool {
    let config = match fs::read_to_string("../feeds.toml") {
        Ok(value) => value,
        Err(_) => unimplemented!(),
    };
    let config: Config = toml::from_str(config.as_str()).unwrap();

    config.cache.cache.to_vec().contains(&value.to_string())
}
fn add_cache(value: &str) {
    let config = match fs::read_to_string("../feeds.toml") {
        Ok(value) => value,
        Err(_) => unimplemented!(),
    };

    let mut config: Config = toml::from_str(config.as_str()).unwrap();

    config.cache.cache.push(value.parse().unwrap());

    let string = toml::to_string(&config).unwrap();

    fs::write("../feeds.toml", string.as_bytes()).expect("TODO: panic message");
}
