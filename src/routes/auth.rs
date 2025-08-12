use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::Form;
use futures_util::TryStreamExt;
use handlebars::Handlebars;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;
extern crate dotenv;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum_cookie::prelude::*;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

#[derive(Serialize)]
pub struct AuthPageHTMLValues {
    redirect: String,
}

pub async fn auth_page(
    Path(redirect): Path<String>,
    State(pool): State<Pool<Postgres>>,
) -> Html<String> {
    let mut reg = Handlebars::new();
    let mut file = File::open("./static/html/login.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("WOOPS");
    let val = AuthPageHTMLValues { redirect: redirect };
    let o = reg
        .render_template(&contents, &serde_json::to_value(val).expect("woop"))
        .expect("woops");
    return Html(o);
}

#[derive(Debug, Deserialize)]
pub struct LoginFormItems {
    email: String,
    password: String,
}

pub async fn submit_auth(
    Path(redirect): Path<String>,
    cookie: CookieManager,
    State(pool): State<Pool<Postgres>>,
    Form(input): Form<LoginFormItems>,
) -> Html<String> {
    let mut rows_first = sqlx::query("SELECT * FROM authors WHERE email = $1")
        .bind(input.email.clone())
        .fetch(&pool);
    if !rows_first.try_next().await.unwrap_or(None).is_some() {
        let salt = SaltString::generate(&mut OsRng);
        let pass_hash = Argon2::default().hash_password(input.password.as_bytes(), &salt);
        if pass_hash.is_err() {
            return Html("Password Parsing Error".to_string());
        }
        let mut q = sqlx::query(
            "INSERT INTO authors (author_id, name, year, bio, email, perm_level, google_magic) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        ).bind(Uuid::new_v4().to_string()).bind("").bind(2025).bind("").bind(input.email).bind(0).bind(pass_hash.unwrap().to_string());
        q.execute(&pool).await.expect("woops");
        return Html("Account Created - Please Request Edit Access from Quinn".to_string());
    }
    let mut rows = sqlx::query("SELECT * FROM authors WHERE email = $1")
        .bind(input.email.clone())
        .fetch(&pool);
    while let Some(row) = rows.try_next().await.expect("Woops") {
        let tok = row.try_get("google_magic").unwrap_or("");
        let hashed_tok = &PasswordHash::new(tok).unwrap();
        let auth_level: i16 = row.try_get("perm_level").unwrap_or(0);
        let a = Argon2::default()
            .verify_password(input.password.as_bytes(), hashed_tok)
            .is_ok();
        if a == true && auth_level == 5 {
            let mut email_cookie = Cookie::new("email", input.email.clone());
            email_cookie = email_cookie.with_path("/");
            cookie.add(email_cookie);

            let mut token_cookie = Cookie::new("token", hashed_tok.to_string());
            token_cookie = token_cookie.with_path("/");
            cookie.add(token_cookie);
            return Html(
                format!("<meta http-equiv=\"refresh\" content=\"0; url=/{redirect} \" />")
                    .to_string(),
            );
        } else if a == true {
            return Html("Login Sucessful - Please Request Edit Access From Quinn".to_string());
        }
    }
    return Html("Cannot Access".to_string());
}

pub async fn can_user_edit(cookie: CookieManager, pool: &Pool<Postgres>) -> bool {
    let mut email: Option<String> = None;
    let mut token: Option<String> = None;
    if let Some(cookie) = cookie.get("email") {
        email = Some(cookie.value().to_string());
    }
    if let Some(cookie) = cookie.get("token") {
        token = Some(cookie.value().to_string());
    }
    if token.is_none() || email.is_none() {
        return false;
    }

    let mut rows = sqlx::query("SELECT * FROM authors WHERE email = $1")
        .bind(email.clone())
        .fetch(pool);
    while let Some(row) = rows.try_next().await.expect("Woops") {
        let auth_level: i16 = row.try_get("perm_level").unwrap_or(0);
        if row.try_get("google_magic").unwrap_or("") == token.clone().unwrap() && auth_level == 5 {
            return true;
        }
    }

    return false;
}
