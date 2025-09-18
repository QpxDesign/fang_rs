use crate::routes::auth::can_user_edit;
use crate::structs::PageEditInput::PageEditInput;
use axum::extract::State;
use axum::{extract::Form, response::Html, routing::get, Router};
use axum_cookie::prelude::*;
use chrono::prelude::*;
use sqlx::Pool;
use sqlx::Postgres;

pub async fn submit_page_edit(
    cookie: CookieManager,
    State(pool): State<Pool<Postgres>>,
    Form(input): Form<PageEditInput>,
) -> Html<String> {
    if !can_user_edit(cookie, &pool).await {
        return Html("<h2>Authentication Error</h2>".to_string());
    }
    // delete existing article
    let id = input.article_id;
    println!("{}", id);
    sqlx::query("DELETE FROM articles WHERE article_id = $1")
        .bind(&id)
        .execute(&pool)
        .await
        .expect("woops");
    let mut d = input.date.clone();
    d.push_str(" 12:00:00");
    println!("{}", d);

    let date: i64 = NaiveDateTime::parse_from_str(&d, "%m/%d/%Y %H:%M:%S")
        .unwrap_or(
            NaiveDateTime::parse_from_str("01/01/2030 12:00:00", "%m/%d/%Y %H:%M:%S").unwrap(),
        )
        .and_utc()
        .timestamp();
    let authors = input.authors;
    let title = input.title;
    let description = input.description.unwrap_or_else(|| "".to_string());
    let image_slug = input.image_slug.unwrap_or_else(|| "".to_string());
    let article_contents = input.article_contents;
    let article_type = input.article_type;
    let q = sqlx::query("INSERT INTO articles (article_id, time_created_unix, time_updated_unix, authors, article_type, title, description, thumbnail_slug, article_contents, views) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)").bind(id).bind(date as i32).bind(date as i64).bind(authors).bind(article_type).bind(title).bind(description).bind(image_slug).bind(article_contents).bind(0);
    q.execute(&pool).await.expect("woops");
    return Html(
        "<h2>Form Submitted Successfully (Go <a href='/' class='link' >Home</a>)</h2>".to_string(),
    );
}

// id, current_time, current_time, authors, article_type, title, description, thumbnail_slug, article_contents, 0
