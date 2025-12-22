use std::{fs, path::Path};

use async_trait::async_trait;
use mime_guess::from_path;

use explorer_storage::*;

/// 本地文件系统存储提供者
pub struct LocalFileSystemProvider;

impl LocalFileSystemProvider {
    pub fn new() -> Self {
        Self
    }

    /// 检查文件名是否为隐藏文件
    fn is_hidden(path: &Path) -> bool {
        // Unix 系统：以 . 开头的文件是隐藏文件
        #[cfg(unix)]
        {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with('.'))
                .unwrap_or(false)
        }

        // Windows 系统：检查文件属性
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

            path.metadata()
                .map(|meta| (meta.file_attributes() & FILE_ATTRIBUTE_HIDDEN) != 0)
                .unwrap_or(false)
        }

        #[cfg(not(any(unix, windows)))]
        {
            false
        }
    }

    /// 推断文件的 MIME 类型
    fn guess_mime_type(path: &Path) -> Option<String> {
        from_path(path).first().map(|mime| mime.to_string())
    }
}

impl Default for LocalFileSystemProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageProvider for LocalFileSystemProvider {
    async fn get_roots(&self) -> StorageResult<Vec<RootItem>> {
        smol::unblock(|| {
            let mut roots = Vec::new();

            #[cfg(target_os = "macos")]
            {
                // macOS: 读取 /Volumes 目录
                if let Ok(read_dir) = fs::read_dir("/Volumes") {
                    for entry in read_dir {
                        if let Ok(entry) = entry {
                            if let Ok(metadata) = entry.metadata() {
                                if metadata.is_dir() {
                                    let name = entry.file_name().to_string_lossy().to_string();
                                    let path = entry.path().display().to_string();
                                    roots.push(RootItem {
                                        id: path.clone(),
                                        name: name.clone(),
                                        path,
                                        provider_type: ProviderType::LocalFileSystem,
                                        icon: None,
                                    });
                                }
                            }
                        }
                    }
                }

                // 添加根目录
                roots.insert(
                    0,
                    RootItem {
                        id: "/".to_string(),
                        name: "Macintosh HD".to_string(),
                        path: "/".to_string(),
                        provider_type: ProviderType::LocalFileSystem,
                        icon: None,
                    },
                );
            }

            #[cfg(target_os = "linux")]
            {
                // Linux: 根目录和常见挂载点
                roots.push(RootItem {
                    id: "/".to_string(),
                    name: "Root".to_string(),
                    path: "/".to_string(),
                    provider_type: ProviderType::LocalFileSystem,
                    icon: None,
                });

                // 检查 /mnt 和 /media
                for mount_point in &["/mnt", "/media"] {
                    if let Ok(read_dir) = fs::read_dir(mount_point) {
                        for entry in read_dir {
                            if let Ok(entry) = entry {
                                if let Ok(metadata) = entry.metadata() {
                                    if metadata.is_dir() {
                                        let name = entry.file_name().to_string_lossy().to_string();
                                        let path = entry.path().display().to_string();
                                        roots.push(RootItem {
                                            id: path.clone(),
                                            name,
                                            path,
                                            provider_type: ProviderType::LocalFileSystem,
                                            icon: None,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            #[cfg(target_os = "windows")]
            {
                // Windows: 获取所有驱动器
                // TODO: 使用 Windows API 获取驱动器列表
                for letter in 'A'..='Z' {
                    let path = format!("{}:\\", letter);
                    if Path::new(&path).exists() {
                        roots.push(RootItem {
                            id: path.clone(),
                            name: format!("Drive {}", letter),
                            path,
                            provider_type: ProviderType::LocalFileSystem,
                            icon: None,
                        });
                    }
                }
            }

            Ok(roots)
        })
        .await
    }

    async fn get_metadata(&self, path: &str) -> StorageResult<FileItem> {
        let path_str = path.to_string();

        smol::unblock(move || {
            let path = Path::new(&path_str);

            if !path.exists() {
                return Err(StorageError::PathNotFound(path.display().to_string()));
            }

            let metadata = fs::metadata(path)?;
            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());

            let item_type = if metadata.is_dir() {
                ItemType::Directory
            } else if metadata.is_symlink() {
                ItemType::Symlink
            } else {
                ItemType::File
            };

            // 获取修改时间
            let modified = metadata
                .modified()
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

            // 获取创建时间（某些平台可能不支持）
            let created = metadata.created().ok();

            // 获取访问时间（某些平台可能不支持）
            let accessed = metadata.accessed().ok();

            // 获取权限
            #[cfg(unix)]
            let permissions = {
                use std::os::unix::fs::PermissionsExt;
                Some(metadata.permissions().mode())
            };

            #[cfg(not(unix))]
            let permissions = None;

            // 推断 MIME 类型（仅对文件）
            let mime_type = if item_type == ItemType::File {
                Self::guess_mime_type(path)
            } else {
                None
            };

            Ok(FileItem {
                name: file_name.clone(),
                path: path.display().to_string(),
                item_type,
                is_hidden: Self::is_hidden(path),
                size: metadata.len(),
                modified,
                metadata: EntryMetadata {
                    permissions,
                    mime_type,
                    created,
                    accessed,
                    ..Default::default()
                },
            })
        })
        .await
    }

    async fn list_entries(&self, path: &str) -> StorageResult<Vec<FileItem>> {
        let path_str = path.to_string();

        smol::unblock(move || {
            let path = Path::new(&path_str);

            if !path.exists() {
                return Err(StorageError::PathNotFound(path.display().to_string()));
            }

            if !path.is_dir() {
                return Err(StorageError::Other(format!(
                    "路径不是目录: {}",
                    path.display()
                )));
            }

            let mut entries = Vec::new();
            let read_dir = fs::read_dir(path)?;

            for entry in read_dir {
                let entry = entry?;
                let entry_path = entry.path();
                let metadata = entry.metadata()?;
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy().to_string();
                let path_str = entry_path.display().to_string();

                let item_type = if metadata.is_dir() {
                    ItemType::Directory
                } else if metadata.is_symlink() {
                    ItemType::Symlink
                } else {
                    ItemType::File
                };

                // 获取修改时间
                let modified = metadata
                    .modified()
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

                // 获取创建时间（某些平台可能不支持）
                let created = metadata.created().ok();

                // 获取访问时间（某些平台可能不支持）
                let accessed = metadata.accessed().ok();

                // 获取权限
                #[cfg(unix)]
                let permissions = {
                    use std::os::unix::fs::PermissionsExt;
                    Some(metadata.permissions().mode())
                };

                #[cfg(not(unix))]
                let permissions = None;

                // 推断 MIME 类型（仅对文件）
                let mime_type = if item_type == ItemType::File {
                    Self::guess_mime_type(&entry_path)
                } else {
                    None
                };

                entries.push(FileItem {
                    name: name.clone(),
                    path: path_str,
                    item_type,
                    is_hidden: Self::is_hidden(&entry_path),
                    size: metadata.len(),
                    modified,
                    metadata: EntryMetadata {
                        permissions,
                        mime_type,
                        created,
                        accessed,
                        ..Default::default()
                    },
                });
            }

            // 按名称排序：目录在前，文件在后
            entries.sort_by(|a, b| match (a.item_type, b.item_type) {
                (ItemType::Directory, ItemType::Directory) => a.name.cmp(&b.name),
                (ItemType::Directory, _) => std::cmp::Ordering::Less,
                (_, ItemType::Directory) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            Ok(entries)
        })
        .await
    }

    async fn exists(&self, path: &str) -> StorageResult<bool> {
        Ok(Path::new(path).exists())
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::LocalFileSystem
    }
}
