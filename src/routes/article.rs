use crate::structs::article::Article;
use crate::utils::get_articles::get_articles;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use handlebars::handlebars_helper;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use std::fs::File;
use std::io::Read;
extern crate chrono;
use chrono::prelude::*;

#[derive(Serialize)]
struct ArticleHTMLValues {
    date: String,
    article: Article,
    body: Vec<String>,
}

lazy_static! {
   // static ref BI_REGEX: Regex = Regex::new(r"").unwrap();
    static ref B_REGEX: regex::Regex = Regex::new(r"\*\*(.*)\*\*").unwrap();
    static ref I_REGEX: regex::Regex = Regex::new(r"\*(.*)\*").unwrap();
    static ref U_REGEX: regex::Regex = Regex::new(r"_(.*)_").unwrap();
    static ref IMG_REGEX: regex::Regex = Regex::new(r"\{\{(.*)}}").unwrap();
}

pub async fn article(
    Path(article_id): Path<String>,
    State(pool): State<Pool<Postgres>>,
) -> Html<String> {
    // TODO log view
    let mut reg = Handlebars::new();
    handlebars_helper!(ifEquals: |v1: String, v2: String| v1 == v2);
    reg.register_helper("ifEquals", Box::new(ifEquals));
    let mut file = File::open("./static/html/article.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to open file");
    let mut a = get_articles(State(&pool))
        .await
        .into_iter()
        .find(|x| x.article_id == article_id);
    if a.is_none() {
        let mut error_page = File::open("./static/html/404.html").unwrap();
        let mut contents_error = String::new();
        error_page
            .read_to_string(&mut contents_error)
            .expect("WOOPS");
        return Html(contents_error);
    }
    let article: Article = a.unwrap().clone();
    let paragraphs: Vec<&str> = article.article_contents.split("\n").collect();
    let mut body: Vec<String> = Vec::new();
    for paragraph in paragraphs {
        let mut o: String = B_REGEX
            .replace_all(paragraph, "<strong>$1</strong>")
            .to_string();
        o = I_REGEX.replace_all(&o, "<i>$1</i>").to_string();
        o = U_REGEX
            .replace_all(&o, "<span class='underline'>$1</span>")
            .to_string();
        o = IMG_REGEX.replace_all(&o, "<img src='$1' />").to_string();
        body.push(o);
    }

    let val = ArticleHTMLValues {
        date: Local::now().format("%m/%d/%Y").to_string(),
        article: article,
        body: body,
    };
    let o = reg
        .render_template(&contents, &serde_json::to_value(val).expect("woop"))
        .expect("woops");

    return Html(o);
}
