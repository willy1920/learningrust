use domain::config::{Config, DB};
use dotenv::dotenv;
use repository::postgres::user::UserPg;
use sqlx::postgres::PgPoolOptions;

mod domain;
mod repository;
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

    let userRepo = UserPg::new(pool);
    Ok(())
}
