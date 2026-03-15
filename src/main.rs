const FEED_URL: &str = "https://feeds.bbci.co.uk/news/rss.xml";

mod db;
mod feed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::PgPool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("DB 연결 및 마이그레이션 완료");

    let items = feed::fetch_feed(FEED_URL).await?;

    println!("아이템 수: {}", items.len());
    for item in &items {
        println!("- {} ({})", item.title, item.published);
    }
    let feed_id = db::save_feed(&pool, FEED_URL, "BBC News").await?;
    for item in &items {
        db::save_feed_item(&pool, feed_id, item).await?;
    }

    println!("저장 완료: {}건", items.len());

    Ok(())
}
