use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VotingLeaderboard {
    pub epoch: u64,
    pub records: Vec<VotingRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VotingRecord {
    pub node_name: String,
    pub node_address: String,
    pub vote_address: String,
    pub voted_slots: u64,
    pub earned_credits: u64,
    pub total_latency: u64,
    pub datacenter: Option<String>,
    pub continent: Option<String>,
    pub country: Option<String>,
}
