use domain::{
    config::{Config, DB},
    user::User,
};
use dotenv::dotenv;
use repository::postgres::user::UserPg;
use service::user::UserService;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

mod domain;
mod repository;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let conf = Config {
        db: DB {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
            max_connection: std::env::var("MAX_CONNECTIONDB")
                .expect("MAX_CONNECTIONDB must be set.")
                .parse()
                .unwrap(),
        },
    };

    let pool = PgPoolOptions::new()
        .max_connections(conf.db.max_connection as u32)
        .connect(&conf.db.database_url)
        .await?;

    let data: User = User {
        id: 1,
        name: "nama".to_string(),
        email: Some("email@email.com".to_string()),
        password: "password".to_string(),
    };

    let user_repo = UserPg::new(pool);

    let user_service = UserService::new(user_repo);
    user_service.create_user(&data).await;
    user_service.update_user(&data).await;
    user_service.get_all().await;
    user_service.delete_user(&data.id).await;
    Ok(())
}
