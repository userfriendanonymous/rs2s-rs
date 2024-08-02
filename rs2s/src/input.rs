use http::StatusCode;
use serde::Deserialize;
pub use user::{Value as User};
// pub use comment::Value as Comment;

use crate::output::NotFoundError;

pub mod user;
// pub mod studio;
// pub mod comment;
// pub mod project;

pub mod header {
    pub const REQUESTED_WITH: (&'static str, &'static str) = ("X-Requested-With", "XMLHttpRequest");
    pub const CSRF_TOKEN: &'static str = "X-CSRFToken";
    pub const TOKEN: &'static str = "X-Token";
}

pub mod cookie {
    pub const CSRF_TOKEN: &'static str = "scratchcsrftoken";
    pub const SESSION_SID: &'static str = "scratchsessionsid";
    pub const LANGUAGE: &'static str = "scratchlanguage";
}

#[derive(Clone, Copy, Debug)]
pub struct ItemsRange {
    pub offset: u32,
    pub limit: u32,
}

impl ItemsRange {
    pub fn as_query_string(&self) -> String {
        format!("limit={}&offset={}", self.limit, self.offset)
    }
}

#[derive(Debug)]
pub enum JsonOrNotFoundError {
    NotFound(NotFoundError),
    Json(serde_json::Error)
}

impl JsonOrNotFoundError {
    fn new<'d, T: Deserialize<'d>>(status: StatusCode, body: &'d [u8]) -> Result<T, Self> {
        match status.as_u16() {
            x @ 200 ..= 299 => Ok(
                serde_json::from_slice::<T>(&body)
                    .map_err(Self::Json)?
            ),
            _ => Err(Self::NotFound(serde_json::from_slice(&body).map_err(Self::Json)?))
        }
    }
}