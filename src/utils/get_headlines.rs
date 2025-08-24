use axum::extract::State;
use futures_util::TryStreamExt;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;

pub async fn get_headlines(State(pool): State<&Pool<Postgres>>) -> Vec<String> {
    let mut rows = sqlx::query("SELECT * FROM headlines").fetch(pool);
    struct HeadlineItem {
        rank: i32,
        contents: String,
    }
    let mut out: Vec<HeadlineItem> = Vec::new();
    let mut index = -1;
    while let Some(row) = rows.try_next().await.expect("Woops") {
        let pos = row.try_get("item_rank").unwrap_or(index);
        let hl = row.try_get("contents").unwrap_or("");
        out.push(HeadlineItem {
            rank: pos,
            contents: hl.to_string(),
        });
        index -= 1;
    }
    out.sort_by_key(|k| -k.rank);
    let mut o = Vec::new();
    for i in out {
        o.push(i.contents);
    }
    return o;
}
