use serde::Deserialize;
use crate::Api;
use super::utils::RequestBuilderUtils;

#[derive(Deserialize, Clone, Debug)]
pub struct Author {
    pub admin: bool,
    #[serde( rename = "pk" )]
    pub id: u64,
    #[serde( rename = "thumbnail_url" )]
    pub icon_url: String,
    #[serde( rename = "username" )]
    pub name: String,
}

// region: project
#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub fields: ProjectFields,
    #[serde( rename = "pk" )]
    pub id: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectFields {
    #[serde( rename = "creator" )]
    pub author: Author,
    #[serde( rename = "datetime_created" )]
    pub created_at: String,
    #[serde( rename = "datetime_modified" )]
    pub modified_at: String,
    #[serde( rename = "datetime_shared" )]
    pub shared_at: Option<String>,
    pub favorite_count: u32,
    #[serde( rename = "isPublished" )]
    pub public: bool,
    pub love_count: u32,
    #[serde( rename = "remixers_count" )]
    pub remix_count: u32,
    pub thumbnail_url: String,
    pub title: String,
    pub view_count: u32,
    pub visibility: String,
    pub commenters_count: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SharedProject {
    pub fields: SharedProjectFields,
    #[serde( rename = "pk" )]
    pub id: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SharedProjectFields {
    #[serde( rename = "creator" )]
    pub author: Author,
    #[serde( rename = "datetime_created" )]
    pub created_at: String,
    #[serde( rename = "datetime_modified" )]
    pub modified_at: String,
    #[serde( rename = "datetime_shared" )]
    pub shared_at: String,
    pub favorite_count: u32,
    #[serde( rename = "isPublished" )]
    pub public: bool,
    pub love_count: u32,
    #[serde( rename = "remixers_count" )]
    pub remix_count: u32,
    pub thumbnail_url: String,
    pub title: String,
    pub view_count: u32,
    pub visibility: String,
    pub commenters_count: u64,
}
// endregion: project

// region: studio
#[derive(Deserialize, Debug, Clone)]
pub struct Studio {
    pub fields: StudioFields,
    #[serde( rename = "pk" )]
    pub id: u64
}

#[derive(Deserialize, Debug, Clone)]
pub struct StudioFields {
    pub commenters_count: u64,
    pub curator_count: u32,
    #[serde( rename = "datetime_created" )]
    pub created_at: String,
    #[serde( rename = "datetime_modified" )]
    pub modified_at: String,
    pub owner: Author,
    #[serde( rename = "projecters_count" )]
    pub projects_count: u32,
    pub thumbnail_url: String,
    pub title: String,
}
// endregion: studio

impl Api {
    pub async fn stuff_all(&self, page: u32, sort_by: &str) -> super::Result<Vec<Project>> {
        let response = self.get_site_api("projects/all/").query(&[("page", page)])
        .query(&[("ascsort", ""), ("descsort", sort_by)]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn stuff_shared(&self, page: u32, sort_by: &str) -> super::Result<Vec<SharedProject>> {
        let response = self.get_site_api("projects/shared/").query(&[("page", page)])
        .query(&[("ascsort", ""), ("descsort", sort_by)]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn stuff_unshared(&self, page: u32, sort_by: &str) -> super::Result<Vec<Project>> {
        let response = self.get_site_api("projects/notshared/").query(&[("page", page)])
        .query(&[("ascsort", ""), ("descsort", sort_by)]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn stuff_trashed(&self, page: u32, sort_by: &str) -> super::Result<Vec<Project>> {
        let response = self.get_site_api("projects/trashed/").query(&[("page", page)])
        .query(&[("ascsort", ""), ("descsort", sort_by)]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn stuff_studios(&self, page: u32, sort_by: &str) -> super::Result<Vec<Author>> {
        let response = self.get_site_api("galleries/all/").query(&[("page", page)])
        .query(&[("ascsort", ""), ("descsort", sort_by)]).send_success().await?;
        Ok(response.json().await?)
    }
}