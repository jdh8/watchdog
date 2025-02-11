use dotenv::var;
use serde_json::json;

async fn ping(client: &reqwest::Client) -> anyhow::Result<()> {
    const ENDPOINT: &str = "https://natsuki-oehk.shuttle.app/";
    anyhow::ensure!(client.head(ENDPOINT).send().await?.status().is_success());
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    if let Err(error) = ping(&client).await {
        client
            .post(var("WEBHOOK")?)
            .json(&json!({ "content": format!("{}: {error:#?}", var("MENTION")?) }))
            .send()
            .await?;
    }

    Ok(())
}
