use crate::structs::article::Article;
use crate::structs::Author::Author;
use axum::extract::State;
use chrono::prelude::*;
use futures_util::TryStreamExt;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;

pub async fn get_authors(State(pool): State<&Pool<Postgres>>) -> Vec<Author> {
    let mut rows = sqlx::query("SELECT * FROM authors").fetch(pool);
    let mut out: Vec<Author> = Vec::new();

    while let Some(row) = rows.try_next().await.expect("Woops") {
        let fy = "'".to_owned() + &(row.try_get("year").unwrap_or(1997) % 100).to_string();
        let a = Author {
            author_id: row.try_get("author_id").unwrap_or("".to_string()),
            name: row.try_get("name").unwrap_or("".to_string()),
            year: row.try_get("year").unwrap_or(1997),
            bio: row.try_get("bio").unwrap_or("".to_string()),
            google_magic: row.try_get("google_magic").unwrap_or("".to_string()),
            email: row.try_get("email").unwrap_or("".to_string()),
            perm_level: row.try_get("perm_level").unwrap_or(0),
            club_position: row.try_get("club_position").unwrap_or("Goon".to_string()),
            formatted_name: row
                .try_get("name")
                .unwrap_or("GAY".to_string())
                .replace(" ", "-"),
            formatted_year: fy,
        };
        if (a.perm_level > 0) {
            out.push(a);
        }
    }
    return out;
}
