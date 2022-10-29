#[derive(Debug)]
pub enum CustomError {
    SqlxError(sqlx::Error),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::SqlxError(code) => write!(f, "sql error with code {}.", code),
        }
    }
}

impl std::error::Error for CustomError {}

#[derive(Debug)]
pub struct Error {
    error: CustomError,
    description: String,
}

impl Error {
    pub fn new(error: CustomError) -> Self {
        Error {
            description: error.to_string(),
            error,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}
