const FEED_URL: &str = "https://feeds.bbci.co.uk/news/rss.xml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(FEED_URL).await?;
    let body = response.text().await?;
    println!("가져온 바이트 수: {}", body.len());

    Ok(())
}
