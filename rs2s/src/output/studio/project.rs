use serde::Deserialize;
use super::{Api, utils::RequestBuilderUtils};
use crate::{cursor::Cursor, user};

#[derive(Deserialize, Debug)]
pub struct Value {
    pub id: u64,
    pub title: String,
    pub image: String,
    #[serde( rename = "username" )]
    pub name: String,
    pub avatar: user::ProfileImages,
    pub actor_id: u64,
    pub creator_id: u64,
}

impl Api {
    pub async fn studio_projects(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<StudioProject>> {
        let response = self.get(&format!["studios/{id}/projects"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }
}