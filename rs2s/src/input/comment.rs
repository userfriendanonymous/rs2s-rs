use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Value<'a> {
    pub content: &'a str,
    pub parent_id: Option<u64>,
    #[serde(rename = "commentee_id")]
    pub to_id: Option<u64>,
}