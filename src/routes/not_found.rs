use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use std::fs::File;
use std::io::Read;

pub async fn handler_404() -> impl IntoResponse {
    let mut error_page = File::open("./static/html/404.html").unwrap();
    let mut contents_error = String::new();
    error_page
        .read_to_string(&mut contents_error)
        .expect("WOOPS");
    return (StatusCode::NOT_FOUND, Html(contents_error));
}
