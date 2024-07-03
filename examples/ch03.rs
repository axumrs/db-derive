use db_derive::Db;
use serde::{Deserialize, Serialize};

#[derive(Db, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
#[db(table = "messages", pk = "id")]
pub struct Message {
    #[db(find)]
    #[db(skip_insert)]
    pub id: i32,

    #[db(list_opt)]
    #[db(skip_update)]
    pub user_id: String,

    #[db(list_opt)]
    #[db(opt_like)]
    pub content: String,

    #[db(skip_update)]
    pub dateline: chrono::DateTime<chrono::Local>,
}

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("postgres://axum.rs:axum.rs@127.0.0.1:5432/axum_rs")
        .await
        .unwrap();
    let msg = Message {
        user_id: "AXUM-RS".to_string(),
        content: "Hello, AXUM.RS!".to_string(),
        dateline: chrono::Local::now(),
        ..Default::default()
    };
    let id = msg.insert(&pool).await.unwrap();
    println!("id: {}", id);
}
