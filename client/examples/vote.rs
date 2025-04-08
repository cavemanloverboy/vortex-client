use vortex_client::VxClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = VxClient::new();

    let voting_leaderboard = client.fetch_voting_leaderboard().await?;

    for record in voting_leaderboard.records {
        println!("{record:?}");
    }

    Ok(())
}
