use crate::royale_api::RoyaleApi;
use crate::config::Config;
use crate::db:: {MongoDb, Parser};
use std::time::SystemTime;
use mongodb::bson::doc;

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
        self.api.request_player_stats(&player_tag).await
    }

    pub async fn get_current_riverrace(&self) -> String {
        let clan_tag = self.config.read_config("clan_tag");
        let response = self.api.request_current_riverrace(&clan_tag).await;
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32;
        let one_day_ago = &now - 60 * 60 * 24;
        let last_riverrace = self.database.find_one_in_collection("riverrace", doc!{"inserted_date": {"$gt": one_day_ago}}).await.unwrap();
        if last_riverrace.is_none() {
            let a = self.database.insert_into_collection("riverrace", doc!{"inserted_date": &now}).await.unwrap().unwrap();
            println!("inserted {}", a);
        }
        response
    }

}
