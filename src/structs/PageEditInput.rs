use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct PageEditInput {
    pub article_id: String,
    pub authors: String,
    pub article_type: String,
    pub title: String,
    pub description: Option<String>,
    pub image_slug: Option<String>,
    pub article_contents: String,
}
