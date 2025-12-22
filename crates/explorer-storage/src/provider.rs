use async_trait::async_trait;

use explorer_common::{FileItem, ProviderType, RootItem};

use crate::StorageResult;

/// 存储提供者接口
///
/// 所有存储后端（本地文件系统、网络存储、云盘等）都需要实现此 trait
#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// 获取此存储提供者的所有根节点
    ///
    /// 例如：
    /// - 本地文件系统：磁盘分区列表
    /// - 网络存储：已挂载的网络驱动器
    /// - 云盘：已连接的云存储账户
    async fn get_roots(&self) -> StorageResult<Vec<RootItem>>;

    /// 获取指定路径的元数据
    ///
    /// # 参数
    /// * `path` - 文件或目录的路径
    async fn get_metadata(&self, path: &str) -> StorageResult<FileItem>;

    /// 列出指定路径下的所有条目
    ///
    /// # 参数
    /// * `path` - 要列出的目录路径
    ///
    /// # 返回
    /// 返回该目录下所有文件和子目录的列表
    async fn list_entries(&self, path: &str) -> StorageResult<Vec<FileItem>>;

    /// 检查路径是否存在
    ///
    /// # 参数
    /// * `path` - 要检查的路径
    async fn exists(&self, path: &str) -> StorageResult<bool>;

    /// 获取提供者类型标识
    fn provider_type(&self) -> ProviderType;
}
