pub mod database;
pub mod redis;

pub mod error;

// Api modules
pub mod auth;
pub mod users;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = justpic::setup(todo!(), todo!());
}
