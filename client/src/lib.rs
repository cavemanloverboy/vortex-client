use reqwest::Client as HttpClient;
use vortex_types::{income::VotingIncome, voting::VotingLeaderboard};

#[derive(Debug, Clone)]
pub struct VxClient {
    http: HttpClient,
    base_url: String,
}

impl VxClient {
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
            base_url: "https://api.vx.tools".to_string(),
        }
    }

    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            http: HttpClient::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn fetch_voting_leaderboard(&self) -> Result<VotingLeaderboard, reqwest::Error> {
        let url = format!("{}/epochs/leaderboard/voting", self.base_url);
        let res = self
            .http
            .post(url)
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?
            .error_for_status()?;

        let records = res.json::<VotingLeaderboard>().await?;
        Ok(records)
    }

    pub async fn fetch_income_leaderboard(&self) -> Result<VotingIncome, reqwest::Error> {
        let url = format!("{}/epochs/leaderboard/income", self.base_url);
        let res = self
            .http
            .post(url)
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?
            .error_for_status()?;

        let income = res.json::<VotingIncome>().await?;
        Ok(income)
    }
}
