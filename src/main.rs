mod royale_api;
mod config;
use config::Config;
use royale_api::RoyaleApi;
use rocket::State;
#[macro_use] extern crate rocket;

/*fn main() -> _ {
    /*let json = Config::from_str(include_str!("config.json"));
    let config = Config::new(&json);
    let key = config.read_config(&"key");
    let player_tag = config.read_config(&"player_tag");
    let api = RoyaleApi::new(&key);
    let player_stats = api.request_player_stats(&player_tag).await;
    println!("{}", player_stats);
    */
    rocket::build().mount("/", routes![player])
}*/

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
        .mount("/", routes![clanwar])
        .mount("/", routes![warlog])
}


#[get("/player")]
async fn player(handler: &State<Handler>) -> String {
    handler.get_player_stats().await
}

#[get("/clanwar")]
async fn clanwar(handler: &State<Handler>) -> String {
    handler.get_war_stats().await
}

#[get("/riverrace")]
async fn warlog(handler: &State<Handler>) -> String {
    handler.get_current_riverrace().await
}

struct Handler{
    api: RoyaleApi,
    config: Config
}

impl Handler{

    pub fn new(api: RoyaleApi, config: Config) -> Self {
        Self{api, config}
    }

    pub async fn get_player_stats(&self) -> String {
        let player_tag = self.config.read_config("player_tag");
        self.api.request_player_stats(&player_tag).await
    }

    pub async fn get_war_stats(&self) -> String {
        let clan_tag = self.config.read_config("clan_tag");
        self.api.request_war_stats(&clan_tag).await
    }

    pub async fn get_current_riverrace(&self) -> String {
        let clan_tag = self.config.read_config("clan_tag");
        self.api.request_current_riverrace(&clan_tag).await
    }

}