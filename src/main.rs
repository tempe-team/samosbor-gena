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

struct Handler;

pub struct ConnectionPool;
impl TypeMapKey for ConnectionPool {
    type Value = MySqlPool;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();

        if msg.content == "!item" {
            let nouns = sqlx::query!(
                "SELECT * FROM nouns WHERE soul = 0 AND gender != 'ср' AND gender != 'общ' ORDER BY RAND() LIMIT 1"
            ).fetch_all(pool).await.unwrap();

            let noun = &nouns[0].word;
            let gender = &nouns[0].gender;

            let adjectives = sqlx::query!(
                "SELECT * FROM adjectives_morf WHERE gender = ? AND wcase = 'им' AND plural = 0 ORDER BY RAND() LIMIT 1",
                gender
            ).fetch_all(pool).await.unwrap();

            let adjective = &adjectives[0].word;

            let phrase = format!("Предмет: **{} {}**", adjective, noun);

            if let Err(why) = msg.channel_id.say(&ctx.http, phrase).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!effect" {
            let nouns = sqlx::query!(
                "SELECT * FROM nouns WHERE soul = 0 AND gender = 'ср' OR gender = 'общ' ORDER BY RAND() LIMIT 1"
            ).fetch_all(pool).await.unwrap();

            let noun = &nouns[0].word;
            let gender = &nouns[0].gender;

            let phrase;

            if rand::random() {
                let nouns_2 = sqlx::query!(
                    "SELECT * FROM nouns_morf WHERE wcase = 'род' ORDER BY RAND() LIMIT 1",
                ).fetch_all(pool).await.unwrap();

                let noun_2 = &nouns_2[0].word;

                phrase = format!("Эффект: **{} {}**", noun, noun_2);
            } else {
                let adjectives = sqlx::query!(
                    "SELECT * FROM adjectives_morf WHERE gender = ? AND wcase = 'им' AND plural = 0 ORDER BY RAND() LIMIT 1",
                    gender
                ).fetch_all(pool).await.unwrap();

                let adjective = &adjectives[0].word;

                phrase = format!("Эффект: **{} {}**", adjective, noun);
            }

            if let Err(why) = msg.channel_id.say(&ctx.http, phrase).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!npc" {
            let nouns = sqlx::query!(
                "SELECT * FROM nouns WHERE soul = 1 AND gender != 'общ' ORDER BY RAND() LIMIT 1"
            ).fetch_all(pool).await.unwrap();

            let noun = &nouns[0].word;
            let gender = &nouns[0].gender;

            let adjectives = sqlx::query!(
                "SELECT * FROM adjectives_morf WHERE gender = ? AND wcase = 'им' AND plural = 0 ORDER BY RAND() LIMIT 1",
                gender
            ).fetch_all(pool).await.unwrap();

            let adjective = &adjectives[0].word;

            let phrase = format!("NPC: **{} {}**", adjective, noun);

            if let Err(why) = msg.channel_id.say(&ctx.http, phrase).await {
                println!("Error sending message: {:?}", why);
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

    {
        let mut data = client.data.write().await;
        data.insert::<ConnectionPool>(pool);
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}