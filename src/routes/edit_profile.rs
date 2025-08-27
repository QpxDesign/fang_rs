use axum::extract::State;
use axum::response::Html;
use axum::Form;
use axum_cookie::CookieManager;
use handlebars::Handlebars;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct ProfileEditPageValue {
    bio: String,
    name: String,
    year: String,
    email: String,
    position: String,
}

pub async fn profile_editor_page(
    State(pool): State<Pool<Postgres>>,
    cookie: CookieManager,
) -> Html<String> {
    let mut reg = Handlebars::new();
    let mut page = File::open("./static/html/edit-profile.html").unwrap();
    let mut contents = String::new();
    let a: Option<crate::structs::Author::Author> =
        crate::utils::get_authors::get_authors(State(&pool))
            .await
            .iter()
            .find(|x| {
                cookie.get("token").is_some()
                    && x.google_magic == cookie.get("token").unwrap().value()
                    && cookie.get("email").is_some()
                    && x.email == cookie.get("email").unwrap().value()
                    && x.perm_level > 0
            })
            .cloned();
    if a.is_none() {
        return Html(
            format!("<meta http-equiv=\"refresh\" content=\"0; url=/auth \" />").to_string(),
        );
    }
    let a = a.unwrap().clone();
    let page_values: ProfileEditPageValue = ProfileEditPageValue {
        position: a.club_position,
        email: a.email,
        bio: a.bio,
        name: a.name,
        year: a.year.to_string(),
    };
    page.read_to_string(&mut contents).expect("WOOPS");
    let o = reg
        .render_template(&contents, &serde_json::to_value(page_values).expect("woop"))
        .expect("woops");

    return Html(o);
}

pub async fn submit_profile_edit(
    cookie: CookieManager,
    State(pool): State<Pool<Postgres>>,
    Form(input): Form<ProfileEditPageValue>,
) -> Html<String> {
    let a: Option<crate::structs::Author::Author> =
        crate::utils::get_authors::get_authors(State(&pool))
            .await
            .iter()
            .find(|x| {
                cookie.get("token").is_some()
                    && x.google_magic == cookie.get("token").unwrap().value()
                    && cookie.get("email").is_some()
                    && x.email == cookie.get("email").unwrap().value()
            })
            .cloned();
    if a.is_none() {
        return Html(
            format!("<meta http-equiv=\"refresh\" content=\"0; url=/auth \" />").to_string(),
        );
    }
    let y: i64 = input.year.parse().unwrap_or(2025);
    sqlx::query("UPDATE authors SET name=$1, bio=$2, year=$3, club_position=$4 WHERE email=$5")
        .bind(input.name)
        .bind(input.bio)
        .bind(y)
        .bind(input.position)
        .bind(cookie.get("email").unwrap().value())
        .execute(&pool)
        .await
        .expect("woops");

    return Html(
        "<h2>Form Submitted Okay (Go <a href='/' class='link' >Home</a>)</h2>".to_string(),
    );
}
