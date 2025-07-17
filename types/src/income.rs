use serde::{Deserialize, Serialize};

use crate::format_node_name;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VotingIncomeSummary {
    pub base_fees: u64,
    pub priority_fees: u64,
    pub mev_tips: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IncomeRecord {
    #[serde(default)]
    pub rank: u64,
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
pub struct IncomeLeaderboard {
    pub epoch: u64,
    pub records: Vec<IncomeRecord>,
}

impl IncomeLeaderboard {
    pub fn print_pretty(&self, median_fees_per_slot: f64, median_tips_per_slot: f64) {
        println!("Epoch {}\n", self.epoch);

        println!(
            "{:<4} {:<25} {:<44} {:>12} {:>14} {:>14} {:>14} {:>14} {:>14}",
            "#",
            "Node Name",
            "Node Address",
            "Stake",
            "Total Slots",
            "Confirmed",
            "Skipped",
            "Block Rewards",
            "MEV Tips",
        );

        println!("{}", "-".repeat(200));

        for record in self.records.iter() {
            println!(
                "{:<4} {:<25} {:<44} {:>12.2} {:>14} {:>14} {:>14} {:>14} ({:.2}%) {:>14} ({:.2}%)",
                format!("{}", record.rank),
                format_node_name(&record.node_name, 25),
                record.node_address,
                record.stake as f64 / 1e9,
                record.total_slots,
                record.confirmed_slots,
                record.skipped_slots,
                format!(
                    "{:.4}",
                    (record.total_income.base_fees / 2 + record.total_income.priority_fees) as f64
                        / record.confirmed_slots as f64
                        / 1e9
                ),
                100.0
                    * (record.total_income.base_fees / 2 + record.total_income.priority_fees)
                        as f64
                    / record.confirmed_slots as f64
                    / 1e9
                    / median_fees_per_slot,
                format!(
                    "{:.4}",
                    record.total_income.mev_tips as f64 / record.confirmed_slots as f64 / 1e9,
                ),
                100.0 * record.total_income.mev_tips as f64
                    / record.confirmed_slots as f64
                    / 1e9
                    / median_tips_per_slot
            );
        }
    }
}
