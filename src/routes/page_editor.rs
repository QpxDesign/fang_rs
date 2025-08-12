use crate::routes::auth::can_user_edit;
use axum::extract::State;
use axum::response::Html;
use axum_cookie::prelude::*;
use sqlx::Pool;
use sqlx::Postgres;
use std::fs::File;
use std::io::Read;

pub async fn page_editor(
    State(pool): State<Pool<Postgres>>,
    cookie: CookieManager,
) -> Html<String> {
    if can_user_edit(cookie, &pool).await {
        let mut file = File::open("./static/html/page-editor.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("WOOPS");
        return Html(contents);
    }
    return Html(
        "<meta http-equiv=\"refresh\" content=\"0; url=/auth/page-editor \" />".to_string(),
    );
}
