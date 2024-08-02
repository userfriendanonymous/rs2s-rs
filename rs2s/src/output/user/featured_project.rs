use serde::Deserialize;
use crate::Api;
use super::utils::RequestBuilderUtils;

// region: Label
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Label {
    FeaturedProject,
    FeaturedTutorial,
    WorkInProgress,
    RemixThis,
    MyFavoriteThings,
    WhyIScratch,
}

impl Serialize for FeaturedLabel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_id())
    }
}

impl Label {
    pub fn from_label_str(data: &str) -> Option<Self> {
        Some(match data {
            "Featured Project" => Self::FeaturedProject,
            "Featured Tutorial" => Self::FeaturedTutorial,
            "Work In Progress" => Self::WorkInProgress,
            "Remix This!" => Self::RemixThis,
            "My Favorite Things" => Self::MyFavoriteThings,
            "Why I Scratch" => Self::WhyIScratch,
            _ => None?
        })
    }

    pub fn as_id(&self) -> &str {
        match self {
            Self::FeaturedProject => "",
            Self::FeaturedTutorial => "0",
            Self::WorkInProgress => "1",
            Self::RemixThis => "2",
            Self::MyFavoriteThings => "3",
            Self::WhyIScratch => "4"
        }
    }

    pub fn deserialize<'de, D: serde::de::Deserializer<'de>>(d: D) -> Result<Label, D::Error> {
        use serde::de::Error;
        let label = serde_json::Value::deserialize(d)?;
        Self::from_label_str(label.as_str().ok_or(D::Error::custom("expected string"))?)
        .ok_or(D::Error::custom("invalid value"))
    }
}
// endregion: Label

#[derive(Deserialize, Debug, Clone)]
pub struct Value {
    pub id: u64, // not sure what this field is for
    #[serde( rename = "featured_project_label_name", deserialize_with = "Label::deserialize" )]
    pub label: Label,
    #[serde( rename = "featured_project_label_id" )]
    pub label_id: Option<u64>,
    #[serde( rename = "featured_project_data" )]
    pub data: Data,
    #[serde( rename = "thumbnail_url" )]
    pub profile_image_url: String,
    #[serde( rename = "user" )]
    pub user: User,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    #[serde( rename = "creator" )]
    pub author_name: super::Name,
    pub thumbnail_url: String,
    pub id: u64,
    pub title: String,
    #[serde( rename = "datetime_modified" )]
    pub modified_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    #[serde( rename = "username" )]
    pub name: super::Name,
    #[serde( rename = "pk" )]
    pub id: u64,
}
