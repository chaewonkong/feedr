const FEED_URL: &str = "https://feeds.bbci.co.uk/news/rss.xml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(FEED_URL).await?;
    let body = response.text().await?;
    println!("가져온 바이트 수: {}", body.len());

    let feed = feed_rs::parser::parse(body.as_bytes())?;

    if let Some(title) = &feed.title {
        println!("피드 제목: {}", title.content);
    }

    println!("아이템 수: {}", feed.entries.len());

    for entry in &feed.entries {
        if let Some(title) = &entry.title {
            println!("- {}", title.content)
        }
    }

    Ok(())
}
