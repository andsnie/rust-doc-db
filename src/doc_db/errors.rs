
use std::fmt;

#[derive(Debug, Clone)]
pub enum DocDbError {
    InternalError {
        message: String,
        inner_type_name: String,
    },
    FileStorageError {
        message: String,
        inner_type_name: String,
    },
    SqlStorageError {
        message: String,
        inner_type_name: String,
    },
}

impl fmt::Display for DocDbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocDbError::InternalError {
                message,
                inner_type_name,
            } => write!(f, "InternalError: [{}]: {}", inner_type_name, message),

            DocDbError::FileStorageError {
                message,
                inner_type_name,
            } => write!(f, "FileStorageError: [{}]: {}", inner_type_name, message),

            DocDbError::SqlStorageError {
                message,
                inner_type_name,
            } => write!(f, "SqlStorageError: [{}]: {}", inner_type_name, message),
        }
    }
}

// TODO: is it idiomatic to implement From for all errors?
impl std::convert::From<sqlite::Error> for DocDbError {
    fn from(err: sqlite::Error) -> Self {
        DocDbError::SqlStorageError {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<sqlite::Error>().to_string(),
        }
    }
}

impl std::convert::From<std::io::Error> for DocDbError {
    fn from(err: std::io::Error) -> Self {
        DocDbError::FileStorageError {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<std::io::Error>().to_string(),
        }
    }
}

impl std::convert::From<glob::GlobError> for DocDbError {
    fn from(err: glob::GlobError) -> Self {
        DocDbError::FileStorageError {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<glob::GlobError>().to_string(),
        }
    }
}

impl std::convert::From<glob::PatternError> for DocDbError {
    fn from(err: glob::PatternError) -> Self {
        DocDbError::FileStorageError {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<glob::PatternError>().to_string(),
        }
    }
}

impl std::convert::From<serde_json::Error> for DocDbError {
    fn from(err: serde_json::Error) -> Self {
        DocDbError::InternalError {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<serde_json::Error>().to_string(),
        }
    }
}

impl std::convert::From<ulid::DecodeError> for DocDbError {
    fn from(err: ulid::DecodeError) -> Self {
        DocDbError::InternalError {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<ulid::DecodeError>().to_string(),
        }
    }
}

impl std::convert::From<serde_yaml::Error> for DocDbError {
    fn from(err: serde_yaml::Error) -> Self {
        DocDbError::FileStorageError {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<serde_yaml::Error>().to_string(),
        }
    }
}
