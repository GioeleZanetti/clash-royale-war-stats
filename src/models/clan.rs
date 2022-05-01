use serde::{Serialize, Deserialize};
use crate::models::participants::Participants;

#[derive(Serialize, Deserialize)]
pub struct Clan{
    pub tag: String,
    pub name: String,
    #[serde(rename = "badgeId")]
    pub badge_id: i64,
    pub fame: i64,
    #[serde(rename = "repairPoints")]
    pub repair_points: i64,
    pub participants: Vec<Participants>,
    #[serde(rename = "periodPoints")]
    pub period_points: i64,
    #[serde(rename = "clanScore")]
    pub clan_score: i64
}