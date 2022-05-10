mod royale_api;
mod config;
mod handler;
mod db;
mod models;
mod date_calculator;
mod parser;
mod responder;
use handler::Handler;
use config::Config;
use royale_api::RoyaleApi;
use responder::JsonResponder;
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
        .mount("/api", routes![riverrace])
        .mount("/api", routes![clan])
        .mount("/api", routes![past_riverrace])
        .mount("/api", routes![past_riverraces])
        .mount("/setup", routes![insert_riverrace])
        .launch()
        .await
}


#[get("/player")]
async fn player(handler: &State<Handler>) -> JsonResponder {
    let call = handler.get_player_stats().await;
    JsonResponder::new(call)
}

#[get("/riverrace")]
async fn riverrace(handler: &State<Handler>) -> JsonResponder {
    let call = handler.get_current_riverrace().await;
    JsonResponder::new(call)
}

#[get("/clan/<tag>")]
async fn clan(tag: String, handler: &State<Handler>) -> JsonResponder {
    let call = handler.get_clan_info(tag).await;
    JsonResponder::new(call)
}

#[get("/pastriverrace")]
async fn past_riverrace(handler: &State<Handler>) -> JsonResponder {
    let call = handler.get_past_riverrace().await;
    JsonResponder::new(call)
}

#[get("/pastriverraces")]
async fn past_riverraces(handler: &State<Handler>) -> JsonResponder {
    let call = handler.get_past_five_riverraces().await;
    JsonResponder::new(call)
}

#[get("/riverrace")]
async fn insert_riverrace(handler: &State<Handler>){
    handler.setup().await
}