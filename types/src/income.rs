use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VotingIncomeSummary {
    pub base_fees: u64,
    pub priority_fees: u64,
    pub mev_tips: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeRecord {
    pub node_name: String,
    pub node_address: String,
    pub stake: u64,
    pub total_slots: u64,
    pub confirmed_slots: u64,
    pub skipped_slots: u64,
    pub total_income: VotingIncomeSummary,
    pub median_income: VotingIncomeSummary,
    pub min_income: VotingIncomeSummary,
    pub max_income: VotingIncomeSummary,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VotingIncome {
    pub epoch: u64,
    pub records: Vec<IncomeRecord>,
}
