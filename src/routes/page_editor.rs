use crate::routes::auth::can_user_edit;
use crate::structs::PageEditInput::PageEditInput;
use crate::utils::get_articles::get_articles;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum_cookie::prelude::*;
use handlebars::Handlebars;
use sqlx::Pool;
use sqlx::Postgres;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

pub async fn create_page(
    State(pool): State<Pool<Postgres>>,
    cookie: CookieManager,
) -> Html<String> {
    let mut reg = Handlebars::new();

    if can_user_edit(cookie, &pool).await {
        let mut file = File::open("./static/html/page-editor.html").unwrap();
        let mut contents = String::new();
        let val = PageEditInput {
            article_id: Uuid::new_v4().to_string(),
            authors: "".to_string(),
            article_type: "".to_string(),
            title: "".to_string(),
            description: Some("".to_string()),
            image_slug: Some("".to_string()),
            article_contents: "".to_string(),
        };
        file.read_to_string(&mut contents).expect("WOOPS");

        let o = reg
            .render_template(&contents, &serde_json::to_value(val).expect("woop"))
            .expect("woops");
        return Html(o);
    }
    return Html(
        "<meta http-equiv=\"refresh\" content=\"0; url=/auth/page-editor \" />".to_string(),
    );
}

pub async fn edit_page(
    State(pool): State<Pool<Postgres>>,
    cookie: CookieManager,
    Path(article_id): Path<String>,
) -> Html<String> {
    let mut reg = Handlebars::new();
    if !can_user_edit(cookie, &pool).await {
        return Html(
            "<meta http-equiv=\"refresh\" content=\"0; url=/auth/page-editor \" />".to_string(),
        );
    }
    let mut file = File::open("./static/html/page-editor.html").unwrap();
    let mut contents = String::new();
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
    let article = a.unwrap();
    let val = PageEditInput {
        article_id: article.article_id,
        authors: article.authors,
        article_type: article.article_type,
        title: article.title,
        description: Some(article.description),
        image_slug: Some(article.thumbnail_slug),
        article_contents: article.article_contents,
    };
    file.read_to_string(&mut contents).expect("WOOPS");
    let o = reg
        .render_template(&contents, &serde_json::to_value(val).expect("woop"))
        .expect("woops");
    return Html(o);
}

pub async fn delete(
    State(pool): State<Pool<Postgres>>,
    cookie: CookieManager,
    Path(article_id): Path<String>,
) -> Html<String> {
    if !can_user_edit(cookie, &pool).await {
        return Html("<meta http-equiv=\"refresh\" content=\"0; url=/auth/home \" />".to_string());
    }
    if article_id.len() > 0 {
        sqlx::query("DELETE FROM articles WHERE article_id = $1")
            .bind(&article_id)
            .execute(&pool)
            .await
            .expect("woops");
    }

    return Html("<meta http-equiv=\"refresh\" content=\"0; url=/ \" />".to_string());
}
