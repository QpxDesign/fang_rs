use axum::response::Html;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::Read;

#[derive(Serialize)]
pub struct HomePageValues {
    headlines: Vec<String>,
}

pub async fn home() -> Html<String> {
    let mut reg = Handlebars::new();
    let mut file = File::open("./static/html/homepage.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("WOOPS");
    let headlines: HomePageValues = HomePageValues {
        headlines: [
            "Test1".to_string(),
            "Test2".to_string(),
            "Test3".to_string(),
            "Test4".to_string(),
        ]
        .to_vec(),
    };
    let o = reg
        .render_template(&contents, &serde_json::to_value(headlines).expect("woop"))
        .expect("woops");

    return Html(o);
}
