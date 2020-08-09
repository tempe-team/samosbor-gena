use sqlx::mysql::MySqlPool;

pub async fn generate_item(pool: &MySqlPool) -> String {
    let noun = sqlx::query!(
        "SELECT * FROM nouns WHERE soul = 0 AND gender != 'ср' AND gender != 'общ' ORDER BY RAND() LIMIT 1"
    ).fetch_one(pool).await.unwrap();

    let adjective = sqlx::query!(
        "SELECT * FROM adjectives_morf WHERE gender = ? AND wcase = 'им' AND plural = 0 ORDER BY RAND() LIMIT 1",
        noun.gender
    ).fetch_one(pool).await.unwrap();

    format!("Предмет: **{} {}**", adjective.word, noun.word)
}

pub async fn generate_effect(pool: &MySqlPool) -> String {
    let noun = sqlx::query!(
        "SELECT * FROM nouns WHERE soul = 0 AND gender = 'ср' OR gender = 'общ' ORDER BY RAND() LIMIT 1"
    ).fetch_one(pool).await.unwrap();

    let (word_1, word_2) = match rand::random() {
        true => {
            let noun_2 = sqlx::query!(
                "SELECT * FROM nouns_morf WHERE wcase = 'род' ORDER BY RAND() LIMIT 1",
            ).fetch_one(pool).await.unwrap();
            (noun.word, noun_2.word.clone())
        },
        false => {
            let adjective = sqlx::query!(
                "SELECT * FROM adjectives_morf WHERE gender = ? AND wcase = 'им' AND plural = 0 ORDER BY RAND() LIMIT 1",
                noun.gender
            ).fetch_one(pool).await.unwrap();
            (adjective.word.clone(), noun.word)
        }
    };
    format!("Эффект: **{} {}**", word_1, word_2)
}

pub async fn generate_npc(pool: &MySqlPool) -> String {
    let noun = sqlx::query!(
        "SELECT * FROM nouns WHERE soul = 1 AND gender != 'общ' ORDER BY RAND() LIMIT 1"
    ).fetch_one(pool).await.unwrap();

    let adjective = sqlx::query!(
        "SELECT * FROM adjectives_morf WHERE gender = ? AND wcase = 'им' AND plural = 0 ORDER BY RAND() LIMIT 1",
        noun.gender
    ).fetch_one(pool).await.unwrap();

    format!("NPC: **{} {}**", adjective.word, noun.word)
}