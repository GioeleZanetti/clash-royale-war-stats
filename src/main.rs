mod royale_api;
mod config;
mod handler;
use handler::Handler;
use config::Config;
use royale_api::RoyaleApi;
use rocket::State;
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let json = Config::from_str(include_str!("config.json"));
    let config = Config::new(json);
    let key = config.read_config(&"key");
    let api = RoyaleApi::new(key);
    let handler = Handler::new(api, config);

    rocket::build()
        .manage(handler)
        .mount("/", routes![player])
        .mount("/", routes![warlog])
}


#[get("/player")]
async fn player(handler: &State<Handler>) -> String {
    handler.get_player_stats().await
}

#[get("/riverrace")]
async fn warlog(handler: &State<Handler>) -> String {
    handler.get_current_riverrace().await
}