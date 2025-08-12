use crate::structs::article::Article;
use crate::utils::get_articles::get_articles;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use handlebars::Handlebars;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use std::fs::File;
use std::io::Read;

#[derive(Serialize)]
struct ArticleHTMLValues {
    date: String,
    article: Article,
}

pub async fn article(
    Path(article_id): Path<String>,
    State(pool): State<Pool<Postgres>>,
) -> Html<String> {
    // TODO log view
    let mut reg = Handlebars::new();
    let mut file = File::open("./static/html/article.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to open file");
    let a = get_articles(State(&pool))
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
    let val = ArticleHTMLValues {
        date: "".to_string(),
        article: a.unwrap(),
    };
    let o = reg
        .render_template(&contents, &serde_json::to_value(val).expect("woop"))
        .expect("woops");

    return Html(o);
}
