use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct After{
    pub after: String
}