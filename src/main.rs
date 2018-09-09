extern crate serenity;


extern crate reqwest;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate html2runes;
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::env;
use std::iter::Map;

use std::ptr::null;
use std::io::{self, Write};

use serde_json::{Value, Error};


use std::collections::HashMap;

mod commands;

struct Handler;

static GLOBSE_URL: &'static str = "https://glosbe.com/gapi/translate?from=fra&dest=bre&format=json&phrase=%term%";
static LT_URL: &'static str = "https://languagetool.org/api/v2/check";
#[derive(Deserialize)]
struct ResultGlosbe {
    result: String,
    tuc: Vec<Value>
}
#[derive(Deserialize)]
struct ReplacementLT {
    value: String
}
#[derive(Deserialize)]
struct MatchLT {
    message: String,
    offset: i32,
    length: i32,
    replacements: Vec<ReplacementLT>

}
#[derive(Deserialize)]
struct ResultLT {
    matches: Vec<MatchLT>
}

impl EventHandler for Handler {


    fn message(&self, _: Context, msg: Message)  {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with("!termofis") {

            let term = str::replace(&msg.content, "!termofis ", "");
            let message = commands::termofis_run(&term).unwrap();

            if let Err(why) = msg.channel_id.say(message) {
                println!("Error sending message: {:?}", why);
            }
       } else if msg.content.starts_with("!difazi") {
            let mut term = str::replace(&msg.content, "!difazi ", "");
            let mut language = String::from("br");
            let mut res = reqwest::Client::new()
                .post(LT_URL)
                .form(&[("text", &term), ("language", &language)])
                .send()
                .unwrap();
            let mut json_data: ResultLT= res.json().unwrap();
            let mut offset = 0;

            let mut phrase = term.clone();
            let mut iterable_phrase = phrase.clone();
            let mut iter = UnicodeSegmentation::graphemes(iterable_phrase.as_str(), true).collect::<Vec<&str>>();
            let mut message: String = String::new();
            for i in &json_data.matches {

                phrase.insert_str((i.offset + offset) as usize, "~~");
                offset = offset+2;
                println!("{}", &phrase);
                println!("size of phrase: {}, offset: {}", phrase.len().to_string(), ((i.offset + offset + i.length) as usize).to_string());
                phrase.insert_str((i.offset + offset + i.length) as usize, "~~");
                offset = offset+2;
                let mut is_replacement = false;
                if &i.replacements.len() > &0 {
                    is_replacement = true;
                }
                if is_replacement {
                    phrase.insert_str((i.offset + i.length + offset) as usize, &format!("**{}**", &i.replacements.first().unwrap().value));
                    offset = offset + i.replacements.first().unwrap().value.len() as i32;
                }
            }
            println!("{}", &phrase);
            msg.channel_id.say(&phrase);

            for i in &json_data.matches {
                msg.channel_id.say(&i.message);
            }
       } else if msg.content.starts_with("!glosbe") {

            let term = str::replace(&msg.content, "!glosbe ", "");
            let message = commands::glosbe_run(&term).unwrap();

            msg.channel_id.say(&message);
            //println!("{}", jsonData);


       }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
