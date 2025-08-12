use crate::routes::auth::can_user_edit;
use crate::CookieManager;
use axum::extract::State;
use axum::{extract::Form, response::Html, routing::get, Router};
use handlebars::Handlebars;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

#[path = "../utils/mod.rs"]
mod utils;

#[derive(Serialize)]
struct HeadlinePageValues {
    current_headline: String,
}

pub async fn edit_headlines(
    cookie: CookieManager,
    State(pool): State<Pool<Postgres>>,
) -> Html<String> {
    if !can_user_edit(cookie, &pool).await {
        return Html(
            "<meta http-equiv=\"refresh\" content=\"0; url=/auth/edit-headlines \" />".to_string(),
        );
    }
    let mut reg = Handlebars::new();
    let mut file = File::open("./static/html/edit-headlines.html").unwrap();
    let mut contents = String::new();
    let o = utils::get_headlines::get_headlines(State(&pool)).await;
    let mut txt: String = "".to_string();
    let mut index = 0;
    for l in o {
        index += 1;
        txt.push_str(&index.to_string());
        txt.push_str(" ");
        txt.push_str(l.as_str());
        txt.push_str("\n");
    }
    let page_values: HeadlinePageValues = HeadlinePageValues {
        current_headline: txt,
    };
    file.read_to_string(&mut contents).expect("WOOPS");
    let o = reg
        .render_template(&contents, &serde_json::to_value(page_values).expect("woop"))
        .expect("woops");

    return Html(o);
}

#[derive(Deserialize)]
pub struct HeadlineEdit {
    new_headlines: String,
}

pub async fn submit_headline_edit(
    cookie: CookieManager,
    State(pool): State<Pool<Postgres>>,
    Form(input): Form<HeadlineEdit>,
) -> Html<String> {
    if !can_user_edit(cookie, &pool).await {
        return Html("<h2>Authentication Error</h2>".to_string());
    }
    let lines: Vec<&str> = input.new_headlines.split("\n").collect();
    sqlx::query("DELETE FROM headlines")
        .execute(&pool)
        .await
        .expect("woops");
    for l in lines {
        let id = Uuid::new_v4();
        let sp = l.split_once(" ").unwrap_or(("", ""));
        if sp.1.len() == 0 || sp.0.len() == 0 {
            continue;
        }
        let hl = sp.1;
        let rank: i64 = str::parse(sp.0).unwrap_or(0);
        sqlx::query("INSERT INTO headlines (item_id, item_rank, contents) VALUES ($1, $2, $3)")
            .bind(id.to_string())
            .bind(rank)
            .bind(hl)
            .execute(&pool)
            .await
            .expect("woops");
    }

    return Html("<h2>Form Submitted Okay (Go <a href='/'>Home</a>)</h2>".to_string());
}
