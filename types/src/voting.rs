use serde::{Deserialize, Serialize};
use unicode_width::UnicodeWidthStr;

use crate::format_node_name;

#[derive(Debug, Serialize, Deserialize)]
pub struct VotingLeaderboard {
    pub epoch: u64,
    pub records: Vec<VotingRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VotingRecord {
    #[serde(default)]
    pub rank: u64,
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

impl VotingLeaderboard {
    pub fn print_pretty(&self) {
        println!("Epoch {}\n", self.epoch);

        // Print header
        println!(
            "{:<4} {:<4} {:<25} {:<44} {:<44} {:>12} {:>14} {:>12} {:<25} {:<15} {:<15}",
            "#",
            "Rank",
            "Node Name",
            "Node Address",
            "Vote Address",
            "Voted Slots",
            "Earned Credits",
            "Latency",
            "Datacenter",
            "Continent",
            "Country"
        );

        println!("{}", "-".repeat(240));

        for (i, record) in self.records.iter().enumerate() {
            println!(
                "{:<4} {:<4} {:<25} {:<44} {:<44} {:>12} {:>14} {:>9.5} {:<25} {:<15} {:<15}",
                i,
                record.rank,
                format_node_name(&record.node_name, 25),
                record.node_address,
                record.vote_address,
                record.voted_slots,
                record.earned_credits,
                record.total_latency as f64 / record.voted_slots as f64,
                record
                    .datacenter
                    .as_deref()
                    .map(|dc| &dc[..25.min(dc.width())])
                    .unwrap_or("-"),
                record.continent.as_deref().unwrap_or("-"),
                record.country.as_deref().unwrap_or("-"),
            );
        }
    }
}
