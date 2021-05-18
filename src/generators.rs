use crate::cache::{IdxCaches};
use sqlx::mysql::MySqlPool;

pub async fn generate_item(pool: &MySqlPool, caches: &IdxCaches) -> (String, String) {
    let noun = sqlx::query!(
        "SELECT * FROM nouns WHERE IID = ?",
        caches.item_noun_ids.choose(&mut rand::thread_rng())
    ).fetch_one(pool).await.unwrap();
    let gender = noun.gender.unwrap().clone();

    let adjective = sqlx::query!(
        "SELECT * FROM adjectives_morf WHERE IID = ?",
        match gender.as_str() {
            "жен" => caches.female_adjective_ids.choose(&mut rand::thread_rng()),
            _ => caches.male_adjective_ids.choose(&mut rand::thread_rng())
        }
    ).fetch_one(pool).await.unwrap();

    (format!("{} {}", adjective.word, noun.word), gender)
}

pub async fn generate_effect(pool: &MySqlPool, caches: &IdxCaches) -> (String, String) {
    let noun = sqlx::query!(
        "SELECT * FROM nouns WHERE IID = ?",
        caches.effect_noun_ids.choose(&mut rand::thread_rng())
    ).fetch_one(pool).await.unwrap();
    let gender = noun.gender.unwrap().clone();

    let (word_1, word_2) = match rand::random() {
        true => {
            let noun_2 = sqlx::query!(
                "SELECT * FROM nouns_morf WHERE IID = ?",
                caches.genitive_noun_ids.choose(&mut rand::thread_rng())
            ).fetch_one(pool).await.unwrap();
            (noun.word, noun_2.word.clone())
        },
        false => {
            let adjective = sqlx::query!(
                "SELECT * FROM adjectives_morf WHERE IID = ?",
                match gender.as_str() {
                    "муж" => caches.male_adjective_ids.choose(&mut rand::thread_rng()),
                    "жен" => caches.female_adjective_ids.choose(&mut rand::thread_rng()),
                    _ => caches.neuter_adjective_ids.choose(&mut rand::thread_rng())
                }
            ).fetch_one(pool).await.unwrap();
            (adjective.word.clone(), noun.word)
        }
    };
    (format!("{} {}", word_1, word_2), gender)
}

pub async fn generate_npc(pool: &MySqlPool, caches: &IdxCaches) -> (String, String) {
    let noun = sqlx::query!(
        "SELECT * FROM nouns WHERE IID = ?",
        caches.npc_noun_ids.choose(&mut rand::thread_rng())
    ).fetch_one(pool).await.unwrap();
    let gender = noun.gender.unwrap().clone();

    let adjective = sqlx::query!(
        "SELECT * FROM adjectives_morf WHERE IID = ?",
        match gender.as_str() {
            "жен" => caches.female_adjective_ids.choose(&mut rand::thread_rng()),
            _ => caches.male_adjective_ids.choose(&mut rand::thread_rng())
        }
    ).fetch_one(pool).await.unwrap();

    (format!("{} {}", adjective.word, noun.word), gender)
}
