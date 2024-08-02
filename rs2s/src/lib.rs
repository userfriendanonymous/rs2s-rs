pub use username::Value as Username;

pub mod output;
pub mod input;
pub mod username;

mod utils;

pub mod domain {
    pub const API: &str = "api.scratch.mit.edu";
    pub const PROJECTS: &str = "projects.scratch.mit.edu";
    pub const BASE: &str = "scratch.mit.edu";
    pub const CLOUD: &str = "clouddata.scratch.mit.edu";
    pub const UPLOADS: &str = "uploads.scratch.mit.edu";
    pub const ASSETS: &str = "assets.scratch.mit.edu";
    pub const SITE_API: &str = "scratch.mit.edu/site-api";
}
