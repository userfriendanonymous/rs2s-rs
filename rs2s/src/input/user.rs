use http::Method;
use serde::{Deserialize, Serialize};
use crate::output::{self, NotFoundError};
use crate::{domain, Username as Name};
use http_input::{Bytes, Instance as HttpInput};
use super::{JsonOrNotFoundError, ItemsRange};

#[derive(Clone, Debug)]
pub struct Value<'a>(pub Name<'a>);

impl<'a> HttpInput for Value<'a> {
    type Output = Result<output::User, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::get(format!("https://{}/users/{}", domain::API, &self.0))
            .body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

#[derive(Clone, Debug)]
pub struct MessagesCount<'a>(pub Name<'a>);

impl<'a> HttpInput for MessagesCount<'a> {
    type Output = Result<u32, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::builder()
            .uri(format!("https://{}/users/{}/messages/count", domain::API, &self.0))
            .body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new::<output::user::MessagesCount>(head.status, body).map(|s| s.0)
    }
}

#[derive(Clone, Debug)]
pub struct Followers<'a>(pub Name<'a>, pub ItemsRange);

impl<'a> HttpInput for Followers<'a> {
    type Output = Result<Vec<output::User>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::builder()
            .uri(format!(
                "https://{}/users/{}/followers/?{}",
                domain::API, &self.0, self.1.as_query_string()
            ))
            .body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

#[derive(Clone, Debug)]
pub struct Following<'a>(pub Name<'a>, pub ItemsRange);

impl<'a> HttpInput for Following<'a> {
    type Output = Result<Vec<output::User>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::builder()
            .uri(format!(
                "https://{}/users/{}/following/?{}",
                domain::API, &self.0, self.1.as_query_string()
            ))
            .body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

#[derive(Clone, Debug)]
pub struct SetFollow<'a> {
    pub to: Name<'a>,
    pub by: Name<'a>,
    pub state: bool,
}

impl<'a> HttpInput for SetFollow<'a> {
    type Output = Result<Vec<output::User>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::builder()
            .uri(format!(
                "https://{}/users/followers/{}/{}/?usernames={}",
                domain::API, &self.to,
                if self.state { "add" } else { "remove" },
                &self.by
            ))
            .method(Method::PUT)
            .body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

// #[derive(Clone, Debug)]
// pub struct SendComment<'a>(Name<'a>, super::Comment<'a>);

// impl<'a> HttpInput for SendComment<'a> {
//     type Body = String;
//     type Output = Result<(), JsonOrNotFoundError>;
//     fn into_request(self) -> http::Result<http::Request<Self::Body>> {
//         http::Request::post(format!(
//             "https://{}/comments/user/{}/add/",
//             domain::SITE_API, &self.0,
//         ))
//             .body(serde_json::to_string(&self.1).unwrap())
//     }
//     fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
//         JsonOrNotFoundError::new(head.status, body)
//     }
// }

#[derive(Clone, Debug)]
pub struct ToggleCommenting<'a>(Name<'a>);

impl<'a> HttpInput for ToggleCommenting<'a> {
    type Output = Result<(), JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::post(format!(
            "https://{}/comments/user/{}/toggle-comments/",
            domain::SITE_API, &self.0,
        ))
            .body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

// #[derive(Clone, Debug)]
// pub struct ProjectComments<'a>(pub Name<'a>, pub u64, pub ItemsRange);

// impl<'a> HttpInput for ProjectComments<'a> {
//     type Body = ();
//     type Output = Result<Vec<output::Comment>, JsonOrNotFoundError>;
//     fn into_request(self) -> http::Result<http::Request<Self::Body>> {
//         http::Request::get(format!(
//             "https://{}/users/{}/projects/{}/comments/?{}",
//             domain::API, &self.0, self.1, self.2.as_query_string()
//         )).body(())
//     }
//     fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
//         JsonOrNotFoundError::new(head.status, body)
//     }
// }

// #[derive(Clone, Debug)]
// pub struct ProjectCommentReplies<'a> {
//     pub name: Name<'a>, 
//     pub project_id: u64,
//     pub comment_id: u64,
//     pub range: ItemsRange
// }

// impl<'a> HttpInput for ProjectCommentReplies<'a> {
//     type Body = ();
//     type Output = Result<Vec<output::Comment>, JsonOrNotFoundError>;
//     fn into_request(self) -> http::Result<http::Request<Self::Body>> {
//         http::Request::get(format!(
//             "https://{}/users/{}/projects/{}/comments/{}/replies/?{}",
//             domain::API, &self.name, self.project_id, self.comment_id, self.range.as_query_string()
//         )).body(())
//     }
//     fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
//         JsonOrNotFoundError::new(head.status, body)
//     }
// }

#[derive(Clone, Debug)]
pub struct Projects<'a>(pub Name<'a>, pub ItemsRange);

impl<'a> HttpInput for Projects<'a> {
    type Output = Result<Vec<output::Project3>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::get(format!(
            "https://{}/users/{}/projects/?{}",
            domain::API, &self.0, self.1.as_query_string()
        )).body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

#[derive(Clone, Debug)]
pub struct FavoriteProjects<'a>(pub Name<'a>, pub ItemsRange);

impl<'a> HttpInput for FavoriteProjects<'a> {
    type Output = Result<Vec<output::Project3>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::get(format!(
            "https://{}/users/{}/favorites/?{}",
            domain::API, &self.0, self.1.as_query_string()
        )).body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

#[derive(Clone, Debug)]
pub struct ViewedProjects<'a>(pub Name<'a>, pub ItemsRange);

impl<'a> HttpInput for ViewedProjects<'a> {
    type Output = Result<Vec<output::Project2>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::get(format!(
            "https://{}/users/{}/projects/recentlyviewed/?{}",
            domain::API, &self.0, self.1.as_query_string()
        )).body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

#[derive(Clone, Debug)]
pub struct ProjectsLovedByFollowing<'a>(pub Name<'a>, pub ItemsRange);

impl<'a> HttpInput for ProjectsLovedByFollowing<'a> {
    type Output = Result<Vec<output::Project2>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::get(format!(
            "https://{}/users/{}/following/users/loves/?{}",
            domain::API, &self.0, self.1.as_query_string()
        )).body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}

#[derive(Clone, Debug)]
pub struct ProjectsSharedByFollowing<'a>(pub Name<'a>, pub ItemsRange);

impl<'a> HttpInput for ProjectsSharedByFollowing<'a> {
    type Output = Result<Vec<output::Project2>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Bytes>> {
        http::Request::get(format!(
            "https://{}/users/{}/following/users/projects/?{}",
            domain::API, &self.0, self.1.as_query_string()
        )).body(Bytes::new())
    }
    fn output(head: &http::response::Parts, body: &[u8]) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body)
    }
}
