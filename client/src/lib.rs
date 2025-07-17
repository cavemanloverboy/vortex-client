use reqwest::Client as HttpClient;
use vortex_types::{income::IncomeLeaderboard, voting::VotingLeaderboard};

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

    pub async fn fetch_voting_leaderboard(
        &self,
        epoch: Option<u64>,
    ) -> Result<VotingLeaderboard, reqwest::Error> {
        let url = format!("{}/epochs/leaderboard/voting", self.base_url);

        let body = match epoch {
            Some(e) => format!(r#"{{"epoch":{}}}"#, e),
            None => "{}".to_string(),
        };

        let res = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .error_for_status()?;

        let mut voting_leaderboard = res.json::<VotingLeaderboard>().await?;
        voting_leaderboard
            .records
            .sort_by_key(|r| std::cmp::Reverse(r.earned_credits));

        for (i, record) in voting_leaderboard.records.iter_mut().enumerate() {
            record.rank = i as u64;
        }

        Ok(voting_leaderboard)
    }

    pub async fn fetch_income_leaderboard(
        &self,
        epoch: Option<u64>,
    ) -> Result<IncomeLeaderboard, reqwest::Error> {
        let url = format!("{}/epochs/leaderboard/income", self.base_url);

        let body = match epoch {
            Some(e) => format!(r#"{{"epoch":{}}}"#, e),
            None => "{}".to_string(),
        };

        let res = self
            .http
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .error_for_status()?;

        let mut income_leaderboard = res.json::<IncomeLeaderboard>().await?;
        income_leaderboard.records.sort_by_key(|record| {
            std::cmp::Reverse(
                (record.total_income.base_fees + record.total_income.priority_fees)
                    .checked_div(record.total_slots)
                    .unwrap_or(0),
            )
        });

        for (i, record) in income_leaderboard.records.iter_mut().enumerate() {
            record.rank = i as u64;
        }

        Ok(income_leaderboard)
    }
}
