use std::rc::Rc;

use domain::{
    config::{Config, DB},
    user::{User, UserTrait},
};
use dotenv::dotenv;
use repository::postgres::user::UserPg;
use service::user::UserService;
use sqlx::postgres::PgPoolOptions;

mod domain;
mod repository;
mod service;
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let conf = Config {
        db: DB {
            username: std::env::var("USERNAMEDB").expect("USERNAMEDB must be set."),
            password: std::env::var("PASSWORDDB").expect("PASSWORDDB must be set."),
            host: std::env::var("HOSTDB").expect("HOSTDB must be set."),
            name: std::env::var("NAMEDB").expect("NAMEDB must be set."),
            port: std::env::var("PORTDB")
                .expect("PORTDB must be set.")
                .parse()
                .unwrap(),
            max_connection: std::env::var("MAX_CONNECTIONDB")
                .expect("MAX_CONNECTIONDB must be set.")
                .parse()
                .unwrap(),
        },
    };

    let pool = PgPoolOptions::new()
        .max_connections(conf.db.max_connection)
        .connect(&format!(
            "postgres://{}:{}@{}:{}/{}",
            conf.db.username, conf.db.password, conf.db.host, conf.db.port, conf.db.name
        ))
        .await?;

    let data: User = User {
        id: 1,
        name: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    };

    let user_repo = Rc::new(UserPg::new(pool));

    let user_service = UserService::new(user_repo);
    user_service.create_user(&data);
    Ok(())
}
