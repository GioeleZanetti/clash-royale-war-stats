mod royale_api;
mod config;
mod handler;
mod db;
mod models;
mod date_calculator;
use handler::Handler;
use config::Config;
use royale_api::RoyaleApi;
use rocket::State;
use db::{MongoClient, MongoDb};
#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let json = Config::from_str(include_str!("config.json"));
    let config = Config::new(json);
    let key = config.read_config(&"key");
    let api = RoyaleApi::new(key);
    let connection = config.read_config(&"connection");
    let client = MongoClient::from_string(connection).await;
    let dbname = config.read_config(&"dbname");
    let database = MongoDb::new(client, &dbname);
    let handler = Handler::new(api, config, database);

    rocket::build()
        .manage(handler)
        .mount("/api", routes![player])
        .mount("/api", routes![warlog])
        .launch()
        .await
}


#[get("/player")]
async fn player(handler: &State<Handler>) -> String {
    handler.get_player_stats().await
}

#[get("/riverrace")]
async fn warlog(handler: &State<Handler>) -> String {
    handler.get_current_riverrace().await
}