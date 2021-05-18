extern crate dotenv;
use dotenv::dotenv;
use std::{env, path::Path};

use rand::prelude::*;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    http::AttachmentType
};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use sqlx::mysql::MySqlPool;

mod cache;
use cache::{IdxCaches, get_caches};

mod names;
use names::{Names, get_names};

mod faces;
use faces::{Faces, get_faces};

mod generators;
use generators::*;

pub struct ConnectionPool;
impl TypeMapKey for ConnectionPool {
    type Value = MySqlPool;
}

pub struct Caches;
impl TypeMapKey for Caches {
    type Value = IdxCaches;
}

pub struct Namez;
impl TypeMapKey for Namez {
    type Value = Names;
}

pub struct Facez;
impl TypeMapKey for Facez {
    type Value = Faces;
}

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let caches = data.get::<Caches>().unwrap();

        let result = match msg.content.as_str() {
            "!effect" => Some(generate_effect(&pool, &caches).await),
            "!item" => Some(generate_item(&pool, &caches).await),
            "!npc" => Some(generate_npc(&pool, &caches).await),
            "!rich" => Some(generate_npc(&pool, &caches).await),
            _ => None
        };


        match result {
            None => {},
            Some((phrase, gender)) => {
                print!("Trying to send: {}... ", phrase);
                let phrase = match msg.content.as_str() {
                    "!effect" => Some(format!("Эффект: {}", phrase)),
                    "!item" => Some(format!("Предмет: {}", phrase)),
                    "!npc" => Some(phrase),
                    _ => None
                };
                match msg.content.as_str() {
                    "!npc" => {
                        let names = data.get::<Namez>().unwrap();
                        let faces = data.get::<Facez>().unwrap();

                        let face = match gender.as_str() {
                            "муж" => faces.get_male(),
                            _ => faces.get_female(),
                        };
                        let face_path = format!("./faces/UTKFace-Sad/{}", face.path.to_string());

                        let msg = msg.channel_id.send_message(&ctx.http, |m| {
                            // m.content("Hello, World!");
                            m.embed(|e| {
                                e.title(
                                    match gender.as_str() {
                                        "муж" => names.get_male(),
                                        "жен" => names.get_female(),
                                        _ => String::from("Безымянный объект")
                                    },
                                );
                                e.description(phrase.unwrap());
                                e.thumbnail(format!("attachment://{}", face.path));
                                e.field(
                                    format!("Возраст: {}", face.age),
                                    match gender.as_str() {
                                        "муж" => "Пол: мужской",
                                        "жен" => "Пол: женский",
                                        _ => "Пол: неизвестен"
                                    },
                                    false
                                );
                                // e.footer(|f| {
                                //     f.text("This is a footer");

                                //     f
                                // });

                                e
                            });
                            m.add_file(
                                AttachmentType::Path(
                                    Path::new(&face_path)
                                )
                            );
                            m
                        }).await;
                        if let Err(why) = msg {
                            println!("Error sending message: {:?}", why);
                        }
                    },
                    _ => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, phrase.unwrap()).await {
                            println!("Error sending message: {:?}", why);
                        } else {
                            println!("OK!");
                        }
                    }
                };
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

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

async fn run() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    let database_url = env::var("DATABASE_URL")
        .expect("Expected a database URL in the environment");

    let pool = MySqlPool::connect(&database_url).await?;

    let caches = get_caches(&pool).await;

    println!("Loading names...");
    let names = get_names();

    println!("Loading faces...");
    let faces = get_faces();

    {
        let mut data = client.data.write().await;
        data.insert::<ConnectionPool>(pool);
        data.insert::<Caches>(caches);
        data.insert::<Namez>(names);
        data.insert::<Facez>(faces);
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}