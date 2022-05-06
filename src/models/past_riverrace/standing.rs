use serde::{Serialize, Deserialize};
use crate::models::past_riverrace::clan::Clan;

#[derive(Serialize, Deserialize)]
pub struct Standing{
    rank: u8,
    #[serde(rename = "trophyChange")]
    pub trophy_change: i8,
    clan: Clan
}