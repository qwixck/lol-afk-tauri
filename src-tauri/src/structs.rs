use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub pick: Pick,
    pub ban: Ban
}

#[derive(Deserialize, Serialize)]
pub struct Pick {
    pub drafts: Drafts,
    pub blind: Blind
}

#[derive(Deserialize, Serialize)]
pub struct Drafts {
    pub top: Vec<String>,
    pub jungle: Vec<String>,
    pub middle: Vec<String>,
    pub bottom: Vec<String>,
    pub utility: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct Blind {
    pub middle: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct Ban {
    pub drafts: Drafts
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub type_: String,
    pub mode: String,
    pub position: String
}