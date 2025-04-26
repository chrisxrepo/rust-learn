use thiserror::Error;

#[macro_export]
macro_rules! io_error {
    ($x:expr) => {
        crate::errors::Errors::IoError(error_position!($x))
    };
}

#[macro_export]
macro_rules! custom_error {
    ($x:expr) => {
        crate::errors::Errors::CustomError(error_position!($x))
    };
}

#[derive(Debug, Error)]
pub enum Errors {
    #[error("[IoError] {0}")]
    IoError(#[from] ErrorWithPosition<std::io::Error>),

    #[error("[CustomError] {0}")]
    CustomError(#[from] ErrorWithPosition<CustomError>),
}

#[derive(Debug)]
pub struct ErrorWithPosition<Err: std::error::Error> {
    pub e: Err,
    pub file: &'static str,
    pub line: u32,
}

impl<Err: std::error::Error> std::fmt::Display for ErrorWithPosition<Err> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "catch: {}:{} error: {}", self.file, self.line, self.e)
    }
}

impl<Err: std::error::Error> std::error::Error for ErrorWithPosition<Err> {}

#[derive(Debug)]
pub struct CustomError {
    pub reason: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl std::error::Error for CustomError {}

impl From<String> for CustomError {
    fn from(value: String) -> Self {
        CustomError { reason: value }
    }
}

impl From<&str> for CustomError {
    fn from(value: &str) -> Self {
        CustomError {
            reason: value.to_string(),
        }
    }
}

#[macro_export]
macro_rules! error_position {
    ($x:expr) => {
        crate::errors::ErrorWithPosition {
            e: $x,
            file: file!(),
            line: line!(),
        }
    };
}
