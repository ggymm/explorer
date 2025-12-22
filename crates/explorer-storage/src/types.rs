use std::{collections::HashMap, time::SystemTime};

use serde::{Deserialize, Serialize};

/// 存储条目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    File,
    Directory,
    Symlink,
}

/// 条目元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    /// Unix 权限（可选）
    pub permissions: Option<u32>,
    /// MIME 类型（可选）
    pub mime_type: Option<String>,
    /// 创建时间（可选，某些平台不支持）
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "opt_systemtime_serde"
    )]
    pub created: Option<SystemTime>,
    /// 最后访问时间（可选，某些平台不支持）
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "opt_systemtime_serde"
    )]
    pub accessed: Option<SystemTime>,
    /// 自定义字段，用于扩展不同存储提供者的特定信息
    pub custom_fields: HashMap<String, String>,
}

impl Default for EntryMetadata {
    fn default() -> Self {
        Self {
            permissions: None,
            mime_type: None,
            created: None,
            accessed: None,
            custom_fields: HashMap::new(),
        }
    }
}

/// 存储条目（文件或目录）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    /// 条目名称
    pub name: String,
    /// 完整路径（支持本地路径和 URL）
    pub path: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 最后修改时间
    #[serde(with = "systemtime_serde")]
    pub modified: SystemTime,
    /// 条目类型
    pub entry_type: EntryType,
    /// 是否为隐藏文件
    pub is_hidden: bool,
    /// 元数据
    pub metadata: EntryMetadata,
}

/// 存储提供者类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageProviderType {
    LocalFileSystem,
    NetworkDrive,
    CloudStorage { provider_name: String },
}

/// 存储根节点（磁盘、挂载点、云盘等）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRoot {
    /// 唯一标识符
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 根路径
    pub root_path: String,
    /// 提供者类型
    pub provider_type: StorageProviderType,
    /// 图标（可选）
    pub icon: Option<String>,
}

// SystemTime 序列化辅助模块
mod systemtime_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}

// Option<SystemTime> 序列化辅助模块
mod opt_systemtime_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match time {
            Some(t) => {
                let duration = t.duration_since(UNIX_EPOCH).unwrap();
                Some(duration.as_secs()).serialize(serializer)
            }
            None => None::<u64>.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs: Option<u64> = Option::deserialize(deserializer)?;
        Ok(secs.map(|s| UNIX_EPOCH + std::time::Duration::from_secs(s)))
    }
}
