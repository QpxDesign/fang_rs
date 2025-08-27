use crate::structs::article::Article;
use crate::structs::Author::Author;
use crate::utils::get_articles::get_articles;
use crate::utils::get_authors::get_authors;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use chrono::prelude::*;
use handlebars::Handlebars;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use std::fs::File;
use std::io::Read;

#[derive(Serialize)]
struct MembersPageValue {
    authors: Vec<crate::structs::Author::Author>,
}

pub async fn members(State(pool): State<Pool<Postgres>>) -> Html<String> {
    let mut reg = Handlebars::new();
    let mut page = File::open("./static/html/members.html").unwrap();
    let mut contents = String::new();
    let page_values: MembersPageValue = MembersPageValue {
        authors: crate::utils::get_authors::get_authors(State(&pool)).await,
    };
    page.read_to_string(&mut contents).expect("WOOPS");
    let o = reg
        .render_template(&contents, &serde_json::to_value(page_values).expect("woop"))
        .expect("woops");

    return Html(o);
}

#[derive(Serialize)]
struct MemberArticlePageValue {
    date: String,
    author: Author,
    articles: Vec<Article>,
}

pub async fn member_article_page(
    State(pool): State<Pool<Postgres>>,
    Path(author_name): Path<String>,
) -> Html<String> {
    let mut reg = Handlebars::new();
    let mut page = File::open("./static/html/member-page.html").unwrap();
    let mut contents = String::new();
    let author = get_authors(State(&pool))
        .await
        .into_iter()
        .find(|x| x.name.replace(" ", "-") == author_name);
    if author.is_none() {
        return Html("<meta http-equiv=\"refresh\" content=\"0; url=/404 \" />".to_string());
    }
    let page_values: MemberArticlePageValue = MemberArticlePageValue {
        date: Local::now().format("%m/%d/%Y").to_string(),
        author: author.unwrap(),
        articles: get_articles(State(&pool))
            .await
            .into_iter()
            .filter(|x| x.authors.replace(" ", "-").contains(&author_name))
            .collect::<Vec<Article>>(),
    };
    page.read_to_string(&mut contents).expect("WOOPS");
    let o = reg
        .render_template(&contents, &serde_json::to_value(page_values).expect("woop"))
        .expect("woops");

    return Html(o);
}
