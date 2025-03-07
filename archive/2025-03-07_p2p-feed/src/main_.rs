use crate::{item::Item, railway::IteratorExt};

pub async fn main() -> anyhow::Result<()> {
    let feed = reqwest::get("https://www.hannobraun.com/rss/")
        .await?
        .bytes()
        .await?;
    let feed = feed_rs::parser::parse(&*feed)?;

    feed.entries
        .into_iter()
        .map(Item::from_entry)
        .switch(Item::store)
        .track(Item::print)
        .collect::<Result<_, _>>()?;

    Ok(())
}
