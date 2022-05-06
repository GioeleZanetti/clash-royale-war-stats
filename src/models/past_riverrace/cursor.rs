use serde::{Serialize, Deserialize};
use crate::models::past_riverrace::after::After;

#[derive(Serialize, Deserialize)]
pub struct Cursor{
    pub cursors: After
}