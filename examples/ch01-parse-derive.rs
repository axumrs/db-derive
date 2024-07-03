use db_derive::Db;

#[derive(Db)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub nickname: String,
    pub dateline: chrono::DateTime<chrono::Local>,
}

fn main() {}
