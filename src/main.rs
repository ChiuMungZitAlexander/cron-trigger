use reqwest;
use tokio::time::{sleep, Duration};

const EXTERNAL_URL: &str = "https://udw-fe-flooringlab.vercel.app/api/p2p-market-nfts?refreshCache=true";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        make_get_request().await?;
        sleep(Duration::from_secs(12)).await;
    }
}

async fn make_get_request() -> Result<(), reqwest::Error> {
    let response = reqwest::get(EXTERNAL_URL).await?;
    Ok(())
}