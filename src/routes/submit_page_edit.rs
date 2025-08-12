use crate::routes::auth::can_user_edit;
use axum::extract::Path;
use axum::extract::State;
use axum::{extract::Form, response::Html, routing::get, Router};
use axum_cookie::prelude::*;
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Postgres;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PageEditInput {
    current_url: Option<String>,
    authors: String,
    article_type: String,
    title: String,
    description: Option<String>,
    image_slug: Option<String>,
    article_contents: String,
}

pub async fn submit_page_edit(
    cookie: CookieManager,
    State(pool): State<Pool<Postgres>>,
    Form(input): Form<PageEditInput>,
) -> Html<String> {
    if !can_user_edit(cookie, &pool).await {
        return Html("<h2>Authentication Error</h2>".to_string());
    }
    let id = Uuid::new_v4();
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let authors = input.authors;
    let title = input.title;
    let description = input.description.unwrap_or_else(|| "".to_string());
    let image_slug = input.image_slug.unwrap_or_else(|| "".to_string());
    let article_contents = input.article_contents;
    let article_type = input.article_type;
    let q = sqlx::query("INSERT INTO articles (article_id, time_created_unix, time_updated_unix, authors, article_type, title, description, thumbnail_slug, article_contents, views) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)").bind(id.to_string()).bind(current_time as i32).bind(current_time as i64).bind(authors).bind(article_type).bind(title).bind(description).bind(image_slug).bind(article_contents).bind(0);
    q.execute(&pool).await.expect("woops");
    return Html("<h2>Form Submitted Successfully (Go <a href='/'>Home</a>)</h2>".to_string());
}

// id, current_time, current_time, authors, article_type, title, description, thumbnail_slug, article_contents, 0
