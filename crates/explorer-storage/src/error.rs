use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("路径不存在: {0}")]
    PathNotFound(String),

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("不支持的操作: {0}")]
    Unsupported(String),

    #[error("其他错误: {0}")]
    Other(String),
}

pub type StorageResult<T> = Result<T, StorageError>;
