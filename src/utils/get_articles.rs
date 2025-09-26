use crate::structs::article::Article;
use crate::utils::authors_to_string;
use axum::extract::State;
use chrono::prelude::*;
use futures_util::TryStreamExt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use shuffle::shuffler::Shuffler;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;

pub async fn get_articles(State(pool): State<&Pool<Postgres>>) -> Vec<Article> {
    let mut rows = sqlx::query("SELECT * FROM articles").fetch(pool);
    let mut out: Vec<Article> = Vec::new();
    let mut zines: Vec<Article> = Vec::new();
    while let Some(row) = rows.try_next().await.expect("Woops") {
        let mut o = Article {
            article_id: row.try_get("article_id").unwrap_or("".to_string()),
            authors: row.try_get("authors").unwrap_or("".to_string()),
            article_type: row.try_get("article_type").unwrap_or("".to_string()),
            title: row.try_get("title").unwrap_or("".to_string()),
            description: row.try_get("description").unwrap_or("".to_string()),
            thumbnail_slug: row.try_get("thumbnail_slug").unwrap_or("".to_string()),
            time_created_unix: row.try_get("time_created_unix").unwrap_or(0),
            time_updated_unix: row.try_get("time_updated_unix").unwrap_or(0),
            views: row.try_get("thumbnail_slug").unwrap_or(0),
            article_contents: row.try_get("article_contents").unwrap_or("".to_string()),
            formatted_date: "".to_string(),
            formatted_title: "".to_string(),
            formatted_authors: authors_to_string::fancy_html(
                row.try_get("authors").unwrap_or("".to_string()),
            ),
        };
        o.formatted_title = format_url_text(o.title.clone());
        if (o.time_created_unix > 0) {
            let n = NaiveDateTime::from_timestamp(o.time_created_unix, 0);
            let d: DateTime<Utc> = DateTime::from_utc(n, Utc);
            o.formatted_date = d.format("%m/%d/%Y").to_string();
        }

        out.push(o);
    }
    out.sort_by_key(|k| -k.time_created_unix); // SORT BY DATE (NEWEST FIRST)
                                               //out.shuffle(&mut thread_rng()); // SORT RANDOMLY

    return out;
}

fn format_url_text(t: String) -> String {
    let mut out: String = "".to_string();
    for c in t.chars() {
        if c == ' ' {
            out.push_str("-");
        } else if c.is_alphabetic() || c.is_numeric() {
            out.push_str(&c.to_string());
        }
    }
    return out;
}
