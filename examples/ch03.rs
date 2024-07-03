use db_derive::Db;

#[derive(Db)]
#[db(table = "messages", pk = "id")]
pub struct Message {
    pub id: i32,
    pub user_id: String,
    pub content: String,
    pub dateline: chrono::DateTime<chrono::Local>,
}

fn main() {}
