use http::Method;
use serde::{Deserialize, Serialize};
use crate::output::{self, NotFoundError};
use crate::{domain, Username};
use http_input::{Bytes, Instance as HttpInput};
use super::{JsonOrNotFoundError, ItemsRange};

#[derive(Clone, Debug)]
pub struct Value(pub u64);

impl HttpInput for Value {
    type Body = ();
    type Output = Result<output::Project, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Self::Body>> {
        http::Request::get(format!(
            "https://{}/projects/{}", domain::API, self.0
        ))
            .body(())
    }
    fn output(head: &http::response::Parts, body: Bytes) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body.as_ref())
    }
}

#[derive(Clone, Debug)]
pub struct Remixes(pub u64, ItemsRange);

impl HttpInput for Remixes {
    type Body = ();
    type Output = Result<Vec<output::Project3>, JsonOrNotFoundError>;
    fn into_request(self) -> http::Result<http::Request<Self::Body>> {
        http::Request::get(format!(
            "https://{}/projects/{}/remixes/?{}",
            domain::API, self.0, self.1.as_query_string()
        ))
            .body(())
    }
    fn output(head: &http::response::Parts, body: Bytes) -> Self::Output {
        JsonOrNotFoundError::new(head.status, body.as_ref())
    }
}
