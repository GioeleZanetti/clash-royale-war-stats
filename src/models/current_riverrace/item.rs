use serde::{Serialize, Deserialize};
use crate::models::current_riverrace::mini_clan::MiniClan;

#[derive(Serialize, Deserialize)]
pub struct Item{
    pub clan: MiniClan,
    #[serde(rename = "pointsEarned")]
    pub points_earned: i64,
    #[serde(rename = "progressStartOfDay")]
    pub progress_start_of_day: i64,
    #[serde(rename = "progressEndOfDay")]
    pub progress_end_of_day: i64,
    #[serde(rename = "endOfDayRank")]
    pub end_of_day_rank: i64,
    #[serde(rename = "progressEarned")]
    pub progress_earned: i64,
    #[serde(rename = "numOfDefensesRemaining")]
    pub num_of_defenses_remaining: i64,
    #[serde(rename = "progressEarnedFromDefenses")]
    pub progress_earned_from_defenses: i64,
}