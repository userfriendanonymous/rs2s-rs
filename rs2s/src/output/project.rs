use serde::{Deserialize, Serialize};
use serde_json::json;

// region: Project
#[derive(Deserialize, Debug)]
pub struct Value2 {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub author: Author,
    pub image: String,
    pub images: Images,
    pub stats: Stats,
    pub remix: Remix,
    pub history: History,
}

#[derive(Deserialize, Debug)]
pub struct Value3 {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub instructions: String, 
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub author: Value3Author,
    pub image: String,
    pub images: Images,
    pub stats: Stats,
    pub remix: Remix,
    pub history: History,
}

#[derive(Deserialize, Debug)]
pub struct Value {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub author: Author,
    pub image: String,
    pub images: Images,
    pub stats: Stats,
    pub remix: Remix,
    pub history: History,
    #[serde( rename = "project_token" )]
    pub token: String
}
// endregion: Project

// region: Author
#[derive(Deserialize, Debug)]
pub struct Author {
    pub id: u64,
    #[serde( rename = "username" )]
    pub name: String,
    #[serde( rename = "scratchteam" )]
    pub scratch_team: bool,
    pub history: super::user::History,
    pub profile: AuthorProfile,
}

#[derive(Deserialize, Debug)]
pub struct Value3Author {
    pub id: u64,
    #[serde( rename = "scratchteam" )]
    pub scratch_team: bool,
    pub history: super::user::History,
    pub profile: AuthorProfile,
}

#[derive(Deserialize, Debug)]
pub struct AuthorProfile {
    pub id: (),
    pub images: super::user::ProfileImages,
}
// endregion: Author

// region: Project extra
#[derive(Deserialize, Debug)]
pub struct Images {
    #[serde( rename = "282x218" )]
    pub x282_218: String,
    #[serde( rename = "216x163" )]
    pub x216_163: String,
    #[serde( rename = "200x200" )]
    pub x200_200: String,
    #[serde( rename = "144x108" )]
    pub x144_108: String,
    #[serde( rename = "135x102" )]
    pub x135_102: String,
    #[serde( rename = "100x80" )]
    pub x100_80: String,
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub views: u32,
    pub loves: u32,
    pub favorites: u32,
    pub remixes: u32,
}

#[derive(Deserialize, Debug)]
pub struct History {
    pub created: String,
    pub modified: String,
    pub shared: String,
}

#[derive(Deserialize, Debug)]
pub struct Remix {
    pub parent: Option<u64>,
    pub root: Option<u64>,
}
// endregion: Project extra


#[derive(Debug, Clone, Serialize, Default)]
pub struct ProjectInfo {
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub title: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub instructions: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub description: Option<String>,
}
