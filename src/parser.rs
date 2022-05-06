use serde_json::Value;
use mongodb::bson::*;

pub fn value_to_document(value: &Value) -> Document{
    let bson = ser::to_bson(&value).unwrap();
    mongodb::bson::to_document(&bson).unwrap()
}