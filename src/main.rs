extern crate dotenv;
use dotenv::dotenv;
use std::env;

use rand::prelude::*;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use sqlx::mysql::MySqlPool;

mod cache;
use cache::{IdxCaches, get_caches};

mod generators;
use generators::*;

struct Handler;

pub struct ConnectionPool;
impl TypeMapKey for ConnectionPool {
    type Value = MySqlPool;
}

pub struct Caches;
impl TypeMapKey for Caches {
    type Value = IdxCaches;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        let _ = msg.channel_id.broadcast_typing(&ctx.http).await;

        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let caches = data.get::<Caches>().unwrap();

        let phrase = match msg.content.as_str() {
            "!effect" => Some(generate_effect(&pool, &caches).await),
            "!item" => Some(generate_item(&pool, &caches).await),
            "!npc" => Some(generate_npc(&pool, &caches).await),
            _ => None
        };

        match phrase {
            None => {},
            Some(phrase) => {
                print!("Trying to send: {}... ", phrase);
                if let Err(why) = msg.channel_id.say(&ctx.http, phrase).await {
                    println!("Error sending message: {:?}", why);
                } else {
                    println!("OK!");
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    std::process::exit(match run().await {
        Ok(_) => 0,
        Err(_) => 1,
    });
}

async fn run() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let database_url = env::var("DATABASE_URL")
        .expect("Expected a database URL in the environment");

    let pool = MySqlPool::connect(&database_url).await?;

    let caches = get_caches(&pool).await;

    {
        let mut data = client.data.write().await;
        data.insert::<ConnectionPool>(pool);
        data.insert::<Caches>(caches);
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}