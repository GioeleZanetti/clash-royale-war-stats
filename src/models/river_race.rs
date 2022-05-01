use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use crate::models::clan::Clan;
use crate::models::period_log::PeriodLog;

#[derive(Serialize, Deserialize)]
pub struct RiverRace{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub state: String,
    pub clan: Clan,
    pub clans: Vec<Clan>,
    #[serde(rename = "sectionIndex")]
    pub section_index: i64,
    #[serde(rename = "periodIndex")]
    pub period_index: i64,
    #[serde(rename = "periodType")]
    pub period_type: String,
    #[serde(rename = "periodLogs")]
    pub period_logs: Vec<PeriodLog>,
    #[serde(rename = "insertedDate")]
    pub inserted_date: i64 
}