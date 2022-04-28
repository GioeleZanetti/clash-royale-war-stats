use crate::royale_api::RoyaleApi;
use crate::config::Config;

pub struct Handler{
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

    pub async fn get_current_riverrace(&self) -> String {
        let clan_tag = self.config.read_config("clan_tag");
        self.api.request_current_riverrace(&clan_tag).await
    }

}
