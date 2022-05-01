use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Participants{
    pub tag: String,
    pub name: String,
    pub fame: i64,
    #[serde(rename = "repairPoints")]
    pub repair_points: i64,
    #[serde(rename = "boatAttacks")]
    pub boat_attacks: i64,
    #[serde(rename = "decksUsed")]
    pub decks_used: i64,
    #[serde(rename = "decksUsedToday")]
    pub decks_used_today: i64
}