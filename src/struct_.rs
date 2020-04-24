use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub id: String,
    pub access_token: String,
    pub files: Files,
}

#[derive(Deserialize)]
pub struct Files {
    pub vaultd: String,
    pub unique: String,
}
