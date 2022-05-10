use serde::{Serialize, Deserialize};
use crate::models::past_riverrace::riverrace_log::RiverRaceLog;

#[derive(Serialize, Deserialize)]
pub struct LastRiverRaces{
    #[serde(rename = "pastWars")]
    pub past_wars: Vec<RiverRaceLog>
}

impl LastRiverRaces{

    pub fn new() -> Self {
        Self{past_wars: Vec::new()}
    }

}