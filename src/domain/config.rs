#[derive(Debug)]
pub struct Config {
    pub db: DB,
}
#[derive(Debug)]
pub struct DB {
    pub username: String,
    pub password: String,
    pub host: String,
    pub name: String,
    pub port: u16,
    pub max_connection: u8,
}
