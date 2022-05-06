use serde::{Serialize, Deserialize};
use crate::models::current_riverrace::item::Item;

#[derive(Serialize, Deserialize)]
pub struct PeriodLog{
    #[serde(rename = "periodIndex")]
    pub period_index: i64,
    pub items: Vec<Item>
}