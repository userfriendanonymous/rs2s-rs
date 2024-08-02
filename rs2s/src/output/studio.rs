use serde::{Deserialize, Serialize};
use super::{Api, utils::RequestBuilderUtils, SendComment};
use crate::cursor::Cursor;

pub mod project;
pub mod action;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value {
    pub id: u64,
    pub title: String,
    pub host: u64,
    pub description: String,
    pub visibility: String,
    pub public: bool,
    pub open_to_all: bool,
    pub comments_allowed: bool,
    pub image: String,
    pub history: History,
    pub stats: Stats
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value2 {
    pub id: u64,
    pub title: String,
    pub host: u64,
    pub description: String,
    pub visibility: String,
    pub public: bool,
    pub open_to_all: bool,
    pub comments_allowed: bool,
    pub image: String,
    pub history: History,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct History {
    pub created: String,
    pub modified: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stats {
    pub comments: u64,
    pub followers: u64,
    pub managers: u64,
    pub projects: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddProject {
    #[serde( rename = "actorId" )]
    pub actor_id: u64,
    #[serde( rename = "datetimeCreated" )]
    pub created_at: String,
    #[serde( rename = "projectId", deserialize_with = "crate::utils::serde::de::string_to_u64" )]
    pub project_id: u64,
    #[serde( rename = "studioId", deserialize_with = "crate::utils::serde::de::string_to_u64" )]
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Info {
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub title: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub description: Option<String>
}
