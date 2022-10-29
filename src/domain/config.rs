#[derive(Debug)]
pub struct Config {
    pub db: DB,
}
#[derive(Debug)]
pub struct DB {
    pub database_url: String,
    pub max_connection: u8,
}
