use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum SqlError {
    #[error("Database error: {0}")]
    DbError(String),

    #[error("SQL syntax error: {0}")]
    SyntaxError(String),
    
    #[error("No rows returned")]
    NoRows,
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}