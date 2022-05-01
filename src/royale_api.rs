use reqwest;
use reqwest::header::*;
use serde_json::{Value};


pub struct RoyaleApi{
    key: String
}

impl RoyaleApi{

    pub fn new(key: String) -> Self{
        Self{key}
    }

    async fn request(&self, url: &str) -> Value {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.key))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
            
        let json: Value = serde_json::from_str(&response).unwrap();
        json
    }

    pub async fn request_player_stats(&self, player_tag: &str) -> Value {
        let url = format!("https://api.clashroyale.com/v1/players/{}", &player_tag);
        self.request(&url).await
    }

    pub async fn request_current_riverrace(&self, clan_tag: &str) -> Value {
        let url = format!("https://api.clashroyale.com/v1/clans/{}/currentriverrace", &clan_tag);
        self.request(&url).await
    }

}