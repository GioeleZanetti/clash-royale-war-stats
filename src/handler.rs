use crate::royale_api::RoyaleApi;
use crate::config::Config;
use crate::db::{MongoDb};
use std::time::SystemTime;
use mongodb::{bson::*};
use chrono::prelude::*;
use crate::date_calculator;
use crate::models::river_race::RiverRace;

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
        //build options for query, in this case sort
        let options = mongodb::options::FindOptions::builder().sort(doc!{"insertedDate": -1}).build();
        //get clan tag from config file
        let clan_tag = self.config.read_config("clan_tag");
        //get api response
        let response = self.api.request_current_riverrace(&clan_tag).await;
        //parse string into bson
        let bson = ser::to_bson(&response).unwrap();
        //parse bson to document
        let doc = mongodb::bson::to_document(&bson).unwrap();
        //get cursor with latest riverrace
        let last_riverrace = self.database.find_in_collection::<RiverRace>("riverrace", Document::new(), options).await.unwrap();
        //deserialize into riverrace
        //TODO: what if cursor is empty?
        let deserialized: RiverRace = last_riverrace.deserialize_current().unwrap();
        //get inserted_date of latest riverrace
        let last_riverrace_date = NaiveDateTime::from_timestamp(deserialized.inserted_date, 0);
        //convert into seconds from 1970-01-01 into datetime
        let datetime: chrono::prelude::DateTime<Utc> = chrono::prelude::DateTime::from_utc(last_riverrace_date, Utc);
        //get seconds form 1970-01-01
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32;
        //check if today is inside of race period of latest race
        let date_inside = date_calculator::is_in_race_range(&datetime);
        println!("{}", datetime);
        println!("{}", date_inside);
        if date_inside {
            //delete latest race from db
            let deleted_rows = self.database.delete_from_collection("riverrace", doc!{"_id": &deserialized.id}).await.unwrap();
            //insert new race in db
            let inserted_id = self.database.insert_into_collection("riverrace", doc).await.unwrap().unwrap();
            //add insertedDate field to new race
            let updated_rows = self.database.update_from_collection("riverrace", doc!{"_id": &inserted_id}, doc!{"$set": {"insertedDate": &now}}).await.unwrap();
            //debug
            println!("deleted rows: {}", deleted_rows);
            println!("inserted id: {}", inserted_id);
            println!("updated rows: {}", updated_rows);
        }else{
            //insert new race in db
            let inserted_id = self.database.insert_into_collection("riverrace", doc).await.unwrap().unwrap();
            //add insertedDate field to new race
            let updated_rows = self.database.update_from_collection("riverrace", doc!{"_id": &inserted_id}, doc!{"$set": {"insertedDate": &now}}).await.unwrap();
            //debug
            println!("inserted id: {}", inserted_id);
            println!("updated rows: {}", updated_rows);
        }
        serde_json::to_string(&response).unwrap()
    }

}