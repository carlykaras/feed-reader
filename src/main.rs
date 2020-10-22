use std::error::Error;
use rss::Channel;

#[tokio::main]
async fn list() -> Result<Channel, Box<dyn Error>> {
    let res = reqwest::get("https://www.wowhead.com/news/rss/retail")
        .await?
        .bytes()
        .await?;

    println!("Status: {:#?}", res);
    let channel = Channel::read_from(&res[..])?;
    Ok(channel)
}

fn main() {
    println!("{:#?}", list());
}