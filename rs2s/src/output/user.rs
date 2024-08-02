use serde::{Serialize, Deserialize};
use crate::Username as Name;

// pub mod comment;
// pub mod featured_project;

// region: User
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value {
    pub id: u64,
    #[serde( rename = "username" )]
    pub name: String,
    #[serde( rename = "scratchteam" )]
    pub scratch_team: bool,
    pub history: History,
    pub profile: Profile,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub id: u64,
    pub images: ProfileImages,
    pub status: String,
    pub bio: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProfileImages {
    #[serde( rename = "90x90" )]
    pub x90: String,
    #[serde( rename = "60x60" )]
    pub x60: String,
    #[serde( rename = "55x55" )]
    pub x55: String,
    #[serde( rename = "50x50" )]
    pub x50: String,
    #[serde( rename = "32x32" )]
    pub x32: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct History {
    pub joined: String
}

// region: NameCheck
// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum NameCheck {
//     Valid,
//     Invalid,
//     Bad,
//     Taken,
// }

// impl<'de> Deserialize<'de> for NameCheck {
//     fn deserialize<D>(d: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
//         use serde::de::Error;
//         Ok(match Value::deserialize(d)?.as_object().ok_or(D::Error::custom("expected object"))?
//         .get("msg").ok_or(D::Error::custom("'msg' field not found"))?
//         .as_str().ok_or(D::Error::custom("'msg' field must be string"))? {
//             "invalid username" => Self::Invalid,
//             "valid username" => Self::Valid,
//             "username exists" => Self::Taken,
//             "bad username" => Self::Bad,
//             msg => Err(D::Error::custom(format!["invalid 'msg' field value: `{msg}`"]))?
//         })
//     }
// }
// endregion: NameCheck

// #[derive(Debug, Default, Serialize, Clone)]
// pub struct Info {
//     #[serde( skip_serializing_if = "Option::is_none" )]
//     pub bio: Option<String>,
//     #[serde( skip_serializing_if = "Option::is_none" )]
//     pub status: Option<String>,
//     #[serde( rename = "featured_project", skip_serializing_if = "Option::is_none" )]
//     pub featured_id: Option<u64>,
//     #[serde( rename = "featured_project_label", skip_serializing_if = "Option::is_none" )]
//     pub featured_label: Option<FeaturedLabel>
// }

#[derive(Debug, Clone, Copy)]
pub enum SetUserIconError {
    TooLarge, // thumbnail-too-large
    Invalid // image-invalid
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MessagesCount(#[serde(rename = "count")] pub u32);