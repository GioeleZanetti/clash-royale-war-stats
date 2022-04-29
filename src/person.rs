use serde::{Deserialize};
use mongodb::{bson::oid::ObjectId};

#[derive(Debug, Deserialize)]
pub struct Person{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    nome: String,
    anni: i32,
    adasd: String
}