use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub worker: String,
    pub db_uri: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            worker: String::from("anonymous"),
            db_uri: String::from("sqlite:data?mode=rwc"),
        }
    }
}
