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

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InternalError("Something went wrong! Please try again.".to_owned())
    }
}

impl std::fmt::Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest(bdstr) => writeln!(f, "Bad Request: {0}.", bdstr),
            Self::InternalError(_) => write!(f, "An Internal Error has occured.")
        }
    }
}

impl Context for HandlerError {}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}

