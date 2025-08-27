use crate::routes::auth::can_user_edit;
use crate::CookieManager;
use axum::extract::State;
use axum::{extract::Form, response::Html, routing::get, Router};
use chrono::prelude::*;
use futures_util::TryStreamExt;
use handlebars::Handlebars;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

#[derive(Serialize)]
struct EditBusinessPlansPageValues {
    current_business_plans: String,
}

pub async fn business_plan_editor(
    cookie: CookieManager,
    State(pool): State<Pool<Postgres>>,
) -> Html<String> {
    if !can_user_edit(cookie, &pool).await {
        return Html(
            "<meta http-equiv=\"refresh\" content=\"0; url=/auth/edit-business-plans \" />"
                .to_string(),
        );
    }
    let mut reg = Handlebars::new();
    let mut file = File::open("./static/html/edit-business-plans.html").unwrap();
    let mut contents = String::new();
    let mut page_values: EditBusinessPlansPageValues = EditBusinessPlansPageValues {
        current_business_plans: get_business_plans(&pool).await,
    };
    file.read_to_string(&mut contents).expect("WOOPS");
    let o = reg
        .render_template(&contents, &serde_json::to_value(page_values).expect("woop"))
        .expect("woops");

    return Html(o);
}

pub async fn get_business_plans(pool: &Pool<Postgres>) -> String {
    let mut rows = sqlx::query("SELECT * FROM business_plans").fetch(pool);
    let mut out: String = "".to_string();
    while let Some(row) = rows.try_next().await.expect("Woops") {
        out = row.try_get("contents").unwrap_or("").to_string();
    }
    return out;
}

#[derive(Deserialize)]
pub struct BusinessPlansEdit {
    new_business_plans: String,
}

pub async fn edit_business_plans(
    cookie: CookieManager,
    State(pool): State<Pool<Postgres>>,
    Form(input): Form<BusinessPlansEdit>,
) -> Html<String> {
    if !can_user_edit(cookie, &pool).await {
        return Html("<h2>Authentication Error</h2>".to_string());
    }
    let id = Uuid::new_v4();

    let q1 = sqlx::query("DELETE FROM business_plans")
        .execute(&pool)
        .await;
    if q1.is_err() {
        return Html("<h2>Connection Error</h2>".to_string());
    }

    let hl = input.new_business_plans;
    let q2 = sqlx::query("INSERT INTO business_plans (item_id, contents) VALUES ($1, $2)")
        .bind(id.to_string())
        .bind(hl)
        .execute(&pool)
        .await;
    if q2.is_err() {
        return Html("<h2>Connection Error</h2>".to_string());
    }
    return Html(
        "<h2>Form Submitted Okay (Go <a href='/' class='link' >Home</a>)</h2>".to_string(),
    );
}

#[derive(Serialize)]
struct BusinessPlansPageValues {
    bp: Vec<String>,
    date: String,
}

pub async fn business_plans_page(State(pool): State<Pool<Postgres>>) -> Html<String> {
    let mut reg = Handlebars::new();
    let page_values = BusinessPlansPageValues {
        date: Local::now().format("%m/%d/%Y").to_string(),
        bp: get_business_plans(&pool)
            .await
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    };
    let mut page_file = File::open("./static/html/business-plans.html").unwrap();
    let mut contents = String::new();
    page_file.read_to_string(&mut contents).expect("WOOPS");
    let o = reg
        .render_template(&contents, &serde_json::to_value(page_values).expect("woop"))
        .expect("woops");

    return Html(o);
}
