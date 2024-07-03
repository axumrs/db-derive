use db_derive::Db;

#[derive(Debug, Default, Db)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub nickname: String,
    pub dateline: chrono::DateTime<chrono::Local>,
}

fn main() {
    let mut u = User::default();
    // 调用 setter
    u.set_email("team@axum.rs".into());
    // 调用 getter
    let email = u.email();
    println!("email is: {}", email);
}
