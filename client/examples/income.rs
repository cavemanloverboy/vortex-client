use vortex_client::VxClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = VxClient::new();

    let income_leaderboard = client.fetch_income_leaderboard().await?;

    for record in income_leaderboard.records {
        println!("{record:?}");
    }

    Ok(())
}
