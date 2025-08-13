use axum::extract::State;
use axum::response::Html;
use futures_util::TryStreamExt;
use handlebars::Handlebars;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;
use std::fs::File;
use std::io::Read;
extern crate chrono;
use crate::routes::business_plans::get_business_plans;
use crate::structs::article::Article;
use crate::utils::get_articles::get_articles;
use crate::utils::get_headlines::get_headlines;
use chrono::prelude::*;

#[derive(Serialize)]
pub struct HomePageHTMLValues {
    headlines: Vec<String>,
    business_plans: Vec<String>,
    date: String,
    articles: Vec<Article>,
}

pub async fn home(State(pool): State<Pool<Postgres>>) -> Html<String> {
    let mut reg = Handlebars::new();
    let mut file = File::open("./static/html/homepage.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("WOOPS");

    let page_values: HomePageHTMLValues = HomePageHTMLValues {
        headlines: get_headlines(State(&pool)).await,
        articles: get_articles(State(&pool)).await,
        date: Local::now().format("%m/%d/%Y").to_string(),
        business_plans: get_business_plans(&pool)
            .await
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    };
    let o = reg
        .render_template(&contents, &serde_json::to_value(page_values).expect("woop"))
        .expect("woops");

    return Html(o);
}
