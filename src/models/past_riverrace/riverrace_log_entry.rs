use serde::{Serialize, Deserialize};
use crate::models::past_riverrace::standing::Standing;

#[derive(Serialize, Deserialize)]
pub struct RiverraceLogEntry{
    #[serde(rename = "seasonId")]
    pub season_id: u64,
    #[serde(rename = "sectionIndex")]
    pub section_index: u64,
    #[serde(rename = "createdDate")]
    created_date: String,
    pub standings: Vec<Standing>
}