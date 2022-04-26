use reqwest;
use reqwest::header::*;
use serde_json::{to_string_pretty, Value};

#[tokio::main]
async fn main() {
    let config: Value = serde_json::from_str(include_str!("config.json")).unwrap();
    
    let key = read_config(&config, &"key");
    let player_tag = read_config(&config, &"player_tag");
    let clan_tag = read_config(&config, &"clan_tag");

    let url = format!("https://api.clashroyale.com/v1/players/{}", player_tag);
    //let url = format!("https://api.clashroyale.com/v1/clans/{}/currentwar", clan_tag);


    let response = request(&url, &key).await;
    let pretty = serde_json::to_string_pretty(&response).unwrap();

    println!("{}", pretty);
}

async fn request(url: &str, key: &str) -> Value {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
        
    let json: Value = serde_json::from_str(&response).unwrap();

    json
}

fn read_config(config: &Value, field: &str) -> String{
    match config[field] {
        Value::String(ref v) => v.to_string(),
        _ => panic!("")
    }
}
