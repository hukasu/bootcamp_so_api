use error_stack::Context;

#[derive(Debug)]
pub enum DBError{
    InvalidUUID(String),
    Other,
}

impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUUID(uuid) => writeln!(f, "Invalid UUID provided: {0}", uuid),
            Self::Other => write!(f, "Database error occurred")
        }
    }
}

impl Context for DBError {}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}