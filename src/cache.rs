use sqlx::mysql::MySqlPool;

pub struct IdxCaches {
    pub item_noun_ids: Vec<u32>,
    pub effect_noun_ids: Vec<u32>,
    pub male_adjective_ids: Vec<u32>,
    pub female_adjective_ids: Vec<u32>,
    pub neuter_adjective_ids: Vec<u32>,
    pub genitive_noun_ids: Vec<u32>,
    pub npc_noun_ids: Vec<u32>,
}

pub async fn get_caches(pool: &MySqlPool) -> IdxCaches {
    println!("Caching item_noun_ids...");
    let item_noun_ids: Vec<u32> = sqlx::query!(
        "SELECT (IID) FROM nouns WHERE soul = 0 AND gender != 'ср' AND gender != 'общ'"
    ).fetch_all(pool).await.unwrap().iter().map(|row| row.IID as u32).collect();

    println!("Caching effect_noun_ids...");
    let effect_noun_ids: Vec<u32> = sqlx::query!(
        "SELECT (IID) FROM nouns WHERE soul = 0 AND gender = 'ср'"
    ).fetch_all(pool).await.unwrap().iter().map(|row| row.IID as u32).collect();

    println!("Caching male_adjective_ids...");
    let male_adjective_ids: Vec<u32> = sqlx::query!(
        "SELECT (IID) FROM adjectives_morf WHERE gender = 'муж' AND wcase = 'им' AND plural = 0"
    ).fetch_all(pool).await.unwrap().iter().map(|row| row.IID as u32).collect();

    println!("Caching female_adjective_ids...");
    let female_adjective_ids: Vec<u32> = sqlx::query!(
        "SELECT (IID) FROM adjectives_morf WHERE gender = 'жен' AND wcase = 'им' AND plural = 0"
    ).fetch_all(pool).await.unwrap().iter().map(|row| row.IID as u32).collect();

    println!("Caching neuter_adjective_ids...");
    let neuter_adjective_ids: Vec<u32> = sqlx::query!(
        "SELECT (IID) FROM adjectives_morf WHERE gender = 'ср' AND wcase = 'им' AND plural = 0"
    ).fetch_all(pool).await.unwrap().iter().map(|row| row.IID as u32).collect();

    println!("Caching genitive_noun_ids...");
    let genitive_noun_ids: Vec<u32> = sqlx::query!(
        "SELECT (IID) FROM nouns_morf WHERE wcase = 'род'"
    ).fetch_all(pool).await.unwrap().iter().map(|row| row.IID as u32).collect();

    println!("Caching npc_noun_ids...");
    let npc_noun_ids: Vec<u32> = sqlx::query!(
        "SELECT (IID) FROM nouns WHERE soul = 1 AND gender != 'общ'"
    ).fetch_all(pool).await.unwrap().iter().map(|row| row.IID as u32).collect();

    println!("Done caching.");

    IdxCaches {
        item_noun_ids,
        effect_noun_ids,
        male_adjective_ids,
        female_adjective_ids,
        neuter_adjective_ids,
        genitive_noun_ids,
        npc_noun_ids
    }
}