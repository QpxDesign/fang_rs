use axum::{response::Redirect, routing::get, routing::post, Router};
use axum_cookie::prelude::*;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::services::ServeDir;
#[path = "./routes/mod.rs"]
mod routes;

#[path = "./utils/mod.rs"]
mod utils;

#[path = "./structs/mod.rs"]
mod structs;

#[tokio::main]
async fn main() {
    dotenvy::dotenv();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::vars().find(|v| v.0 == "DATABASE_URL").unwrap().1)
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(routes::home::home))
        .route("/page-editor", get(routes::page_editor::create_page))
        .route(
            "/page-editor/{article_id}",
            get(routes::page_editor::edit_page),
        )
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
        .route("/home", get(|| async { Redirect::permanent("/") }))
        .route("/auth", get(|| async { Redirect::permanent("/auth/home") }))
        .route(
            "/edit-business-plans",
            get(routes::business_plans::business_plan_editor),
        )
        .route(
            "/submit-business-plan-edit",
            post(routes::business_plans::edit_business_plans),
        )
        .route("/delete/{article_id}", get(routes::page_editor::delete))
        .fallback(routes::not_found::handler_404)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(pool)
        .layer(CookieLayer::strict());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
