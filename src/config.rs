pub mod conf {
  use serde_derive::Deserialize;
  use std::fs;
  use toml;

  #[derive(Deserialize)]
  pub struct Config {
    pub service: Service,
    pub webhook: Webhook,
    pub slack_webhook: SlackWebhook,
  }

  #[derive(Deserialize)]
  pub struct Service {
    pub host: String,
    pub port: u16,
  }

  #[derive(Deserialize)]
  pub struct Webhook {
    pub url: String,
  }

  #[derive(Deserialize)]
  pub struct SlackWebhook {
    pub url: String,
  }

  pub fn get_config(file_path: &String) -> Config {
    let file_contents = fs::read_to_string(file_path).unwrap();

    toml::from_str(file_contents.as_str()).unwrap()
  }
}
