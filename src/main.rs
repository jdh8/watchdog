use dotenv::var;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let ping = || async {
        const ENDPOINT: &str = "https://natsuki-oehk.shuttle.app/";
        anyhow::ensure!(client.head(ENDPOINT).send().await?.status().is_success());
        Ok(())
    };
    if let Err(error) = ping().await {
        client
            .post(var("WEBHOOK")?)
            .json(&json!({ "content": format!("{error:#?}") }))
            .send()
            .await?;
    }
    Ok(())
}
