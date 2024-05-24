use serde_derive::Deserialize;

pub mod config;

#[derive(Debug, Deserialize)]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub properties: Vec<ProfileProperty>,
    pub profileActions: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}
