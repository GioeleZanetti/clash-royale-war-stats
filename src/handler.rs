use crate::royale_api::RoyaleApi;
use crate::config::Config;
use crate::db::{MongoDb};
use std::time::SystemTime;
use mongodb::{bson::*};
use chrono::prelude::*;
use crate::date_calculator;
use crate::models::current_riverrace::river_race::RiverRace;
use crate::parser;
use crate::models::past_riverrace::riverrace_log::RiverRaceLog;
use crate::models::past_riverrace::last_riverraces::LastRiverRaces;

pub struct Handler{
    api: RoyaleApi,
    config: Config,
    database: MongoDb
}

impl Handler{

    pub fn new(api: RoyaleApi, config: Config, database: MongoDb) -> Self {
        Self{api, config, database}
    }

    pub async fn get_player_stats(&self) -> String {
        let player_tag = self.config.read_config("player_tag");
        let response = self.api.request_player_stats(&player_tag).await;
        serde_json::to_string(&response).unwrap()
    }

    pub async fn get_current_riverrace(&self) -> String {
        let clan_tag = self.config.read_config("clan_tag");
        let response = self.api.request_current_riverrace(&clan_tag).await;
        let doc = parser::value_to_document(&response);
        let options = mongodb::options::FindOptions::builder().sort(doc!{"insertedDate": -1}).build();
        let last_riverrace = self.database.find_in_collection::<RiverRace>("riverrace", Document::new(), options).await.unwrap();
        //TODO: what if cursor is empty?
        let deserialized: RiverRace = last_riverrace.deserialize_current().unwrap();
        let last_riverrace_date = NaiveDateTime::from_timestamp(deserialized.inserted_date, 0);
        let datetime: chrono::prelude::DateTime<Utc> = chrono::prelude::DateTime::from_utc(last_riverrace_date, Utc);
        let date_inside = date_calculator::is_in_race_range(datetime);
        if date_inside {
            self.update_current_riverrace(deserialized, doc).await;
        }else{
            self.insert_current_and_past_riverrace(deserialized, doc).await;
        }
        serde_json::to_string(&response).unwrap()
    }

    pub async fn get_clan_info(&self, clan_tag: String) -> String {
        let response = self.api.request_clan_info(&clan_tag).await;
        serde_json::to_string(&response).unwrap()
    }

    pub async fn setup(&self){
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32;
        let clan_tag = self.config.read_config("clan_tag");
        let response = self.api.request_current_riverrace(&clan_tag).await;
        let doc = parser::value_to_document(&response);
        let inserted_id = self.database.insert_into_collection("riverrace", doc).await.unwrap().unwrap();
        let updated_rows = self.database.update_from_collection("riverrace", doc!{"_id": &inserted_id}, doc!{"$set": {"insertedDate": &now}}).await.unwrap();
        println!("inserted id in riverrace: {}", inserted_id); 
        println!("updated rows in riverrace: {}", updated_rows);
    }

    async fn insert_current_and_past_riverrace(&self, riverrace_to_delete: RiverRace, riverrace_to_insert: Document){
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32;
        let clan_tag = self.config.read_config("clan_tag");
        let response = self.api.request_past_riverrace(&clan_tag).await;
        let doc = parser::value_to_document(&response);
        let inserted_id = self.database.insert_into_collection("pastriverrace", doc).await.unwrap().unwrap();
        let deleted_rows = self.database.delete_from_collection("riverrace", doc!{"_id": &riverrace_to_delete.id}).await.unwrap();
        let inserted_id_2 = self.database.insert_into_collection("riverrace", riverrace_to_insert).await.unwrap().unwrap();
        let _updated_rows = self.database.update_from_collection("riverrace", doc!{"_id": &inserted_id_2}, doc!{"$set": {"insertedDate": &now}}).await.unwrap();
        println!("deleted rows in riverrace: {}", deleted_rows);
        println!("inserted id in riverrace: {}", inserted_id_2);
        println!("inserted id in pastriverrace: {}", inserted_id);
    }

    async fn update_current_riverrace(&self, riverrace: RiverRace, document: Document){
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32;
        let deleted_rows = self.database.delete_from_collection("riverrace", doc!{"_id": &riverrace.id}).await.unwrap();
        let inserted_id = self.database.insert_into_collection("riverrace", document).await.unwrap().unwrap();
        let updated_rows = self.database.update_from_collection("riverrace", doc!{"_id": &inserted_id}, doc!{"$set": {"insertedDate": &now}}).await.unwrap();
        println!("deleted rows in riverrace: {}", deleted_rows);
        println!("inserted id in riverrace: {}", inserted_id);
        println!("updated rows in riverrace: {}", updated_rows);
    }

    pub async fn get_past_riverrace(&self) -> String {
        let options = mongodb::options::FindOptions::builder().sort(doc!{"seasonId": -1}).build();
        let last_riverrace = self.database.find_in_collection::<RiverRaceLog>("pastriverrace", Document::new(), options).await.unwrap();
        //TODO: what if cursor is empty?
        let deserialized: RiverRaceLog = last_riverrace.deserialize_current().unwrap();
        println!("Got last riverrace from database");
        serde_json::to_string(&deserialized).unwrap()
    }

    pub async fn get_past_five_riverraces(&self) -> String {
        let options = mongodb::options::FindOptions::builder().sort(doc!{"seasonId": -1}).limit(5).build();
        let mut last_riverraces = self.database.find_in_collection::<RiverRaceLog>("pastriverrace", Document::new(), options).await.unwrap();
        let mut deserialized = LastRiverRaces::new();
        //TODO: what if cursor is empty?
        while last_riverraces.advance().await.unwrap() {
            deserialized.past_wars.push(last_riverraces.deserialize_current().unwrap());
        }
        println!("Got last riverrace from database");
        serde_json::to_string(&deserialized).unwrap()
    }

}