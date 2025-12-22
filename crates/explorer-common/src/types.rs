use serde::{Deserialize, Serialize};

/// 通用数据类型模块
/// 供 UI 组件和应用层共享使用

/// 存储根节点信息（用于侧边栏显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootItem {
    /// 唯一标识符
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 根路径
    pub path: String,
    /// 提供者类型
    pub provider_type: ProviderType,
}

/// 存储提供者类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {
    LocalFileSystem,
    NetworkDrive,
    CloudStorage,
}

/// 文件/目录条目信息（用于文件列表显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    /// 条目名称
    pub name: String,
    /// 完整路径
    pub path: String,
    /// 条目类型
    pub item_type: ItemType,
    /// 是否为隐藏文件
    pub is_hidden: bool,
}

/// 条目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    File,
    Directory,
    Symlink,
}
