use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Article {
    pub title: String,
    pub article_id: String,
    pub time_created_unix: i64,
    pub time_updated_unix: i64,
    pub authors: String,
    pub article_type: String, // TODO: make enum
    pub description: String,
    pub thumbnail_slug: String,
    pub article_contents: String,
    pub views: i64,
    pub formatted_date: String,
}
