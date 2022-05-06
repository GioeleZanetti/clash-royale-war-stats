use serde::{Serialize, Deserialize};
use crate::models::past_riverrace::riverrace_log_entry::RiverraceLogEntry;
use crate::models::past_riverrace::cursor::Cursor;
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
pub struct RiverRaceLog{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub items: Vec<RiverraceLogEntry>,
    pub paging: Cursor
}