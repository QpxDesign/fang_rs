use axum::{routing::get, routing::post, Router};
use axum_cookie::prelude::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use tower_http::services::ServeDir;
#[path = "./routes/mod.rs"]
mod routes;

#[path = "./utils/mod.rs"]
mod utils;

#[path = "./structs/mod.rs"]
mod structs;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/fang")
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(routes::home::home))
        .route("/page-editor", get(routes::page_editor::page_editor))
        .route(
            "/submit-page-edit",
            post(routes::submit_page_edit::submit_page_edit),
        )
        .route(
            "/edit-headlines",
            get(routes::edit_headlines::edit_headlines),
        )
        .route(
            "/submit-headline-edit",
            post(routes::edit_headlines::submit_headline_edit),
        )
        .route("/submit-auth/{redirect}", post(routes::auth::submit_auth))
        .route("/article/{article_id}", get(routes::article::article))
        .route("/auth/{redirect}", get(routes::auth::auth_page))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(pool)
        .layer(CookieLayer::strict());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
