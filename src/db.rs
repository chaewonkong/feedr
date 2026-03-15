use crate::feed::FeedItem;

pub async fn save_feed(
    pool: &sqlx::PgPool,
    url: &str,
    title: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO feed (url, title) VALUES ($1, $2)
                ON CONFLICT (url) DO UPDATE SET title = $2, updated_at = NOW()
                RETURNING id
                ",
    )
    .bind(url)
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn save_feed_item(
    pool: &sqlx::PgPool,
    feed_id: i32,
    item: &FeedItem,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query(
        "INSERT INTO feed_item (feed_id, guid, title, link, summary)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (feed_id, guid) DO NOTHING",
    )
    .bind(feed_id)
    .bind(&item.id)
    .bind(&item.title)
    .bind(&item.link)
    .bind(&item.summary)
    .execute(pool)
    .await?;

    Ok(())
}
