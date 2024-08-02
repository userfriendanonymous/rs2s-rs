use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value {
    pub id: u64,
    pub author: Author,
    pub parent_id: Option<u64>,
    #[serde( rename = "commentee_id" )]
    pub to_user_id: Option<u64>,
    pub content: String,
    #[serde( rename = "datetime_created" )]
    pub created_at: String,
    #[serde( rename = "datetime_modified" )]
    pub modified_at: String,
    pub reply_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    pub id: u64,
    #[serde( rename = "username" )]
    pub name: String,
    #[serde( rename = "scratchteam" )]
    pub scratch_team: bool,
    pub image: String
}
