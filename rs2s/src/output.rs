use serde::{Deserialize, Serialize};
pub use user::Value as User;
// pub use studio::{Value as Studio, Value2 as Studio2};
// pub use message::Value as Message;
pub use project::{Value as Project, Value2 as Project2, Value3 as Project3};
// pub use comment::Value as Comment;

pub mod user;
// pub mod my_stuff;
// pub mod front_page;
// pub mod studio;
// pub mod search;
// pub mod message;
pub mod project;
// pub mod comment;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotFoundError {
    pub code: String,
    pub message: String,
}