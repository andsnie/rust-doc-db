use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DocDbError {
    #[error("InternalError: [{inner_type_name:?}]: {message:?}")]
    Internal {
        message: String,
        inner_type_name: String,
    },
    #[error("FileStorageError: [{inner_type_name:?}]: {message:?}")]
    FileStorage {
        message: String,
        inner_type_name: String,
    },
    #[error("SqlStorageError: [{inner_type_name:?}]: {message:?}")]
    SqlStorage {
        message: String,
        inner_type_name: String,
    },
}

// TODO: is it idiomatic to implement From for all errors?
impl std::convert::From<sqlite::Error> for DocDbError {
    fn from(err: sqlite::Error) -> Self {
        DocDbError::SqlStorage {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<sqlite::Error>().to_string(),
        }
    }
}

impl std::convert::From<std::io::Error> for DocDbError {
    fn from(err: std::io::Error) -> Self {
        DocDbError::FileStorage {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<std::io::Error>().to_string(),
        }
    }
}

impl std::convert::From<glob::GlobError> for DocDbError {
    fn from(err: glob::GlobError) -> Self {
        DocDbError::FileStorage {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<glob::GlobError>().to_string(),
        }
    }
}

impl std::convert::From<glob::PatternError> for DocDbError {
    fn from(err: glob::PatternError) -> Self {
        DocDbError::FileStorage {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<glob::PatternError>().to_string(),
        }
    }
}

impl std::convert::From<serde_json::Error> for DocDbError {
    fn from(err: serde_json::Error) -> Self {
        DocDbError::Internal {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<serde_json::Error>().to_string(),
        }
    }
}

impl std::convert::From<ulid::DecodeError> for DocDbError {
    fn from(err: ulid::DecodeError) -> Self {
        DocDbError::Internal {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<ulid::DecodeError>().to_string(),
        }
    }
}

impl std::convert::From<serde_yaml::Error> for DocDbError {
    fn from(err: serde_yaml::Error) -> Self {
        DocDbError::FileStorage {
            message: err.to_string(),
            inner_type_name: std::any::type_name::<serde_yaml::Error>().to_string(),
        }
    }
}
