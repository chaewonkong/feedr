pub struct FeedItem {
    pub id: String,
    pub title: String,
    pub link: String,
    pub summary: String,
    pub published: String,
}

pub async fn fetch_feed(url: &str) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let feed = feed_rs::parser::parse(body.as_bytes())?;

    let mut items = Vec::new();
    for entry in &feed.entries {
        let item = FeedItem {
            id: entry.id.clone(),
            title: entry
                .title
                .as_ref()
                .map(|t| t.content.clone())
                .unwrap_or_default(),
            link: entry
                .links
                .first()
                .map(|l| l.href.clone())
                .unwrap_or_default(),
            summary: entry
                .summary
                .as_ref()
                .map(|s| s.content.clone())
                .unwrap_or_default(),
            published: entry.published.map(|p| p.to_rfc3339()).unwrap_or_default(),
        };
        items.push(item);
    }

    Ok(items)
}
