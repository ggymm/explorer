use std::{
    fs::create_dir_all,
    io::stdout,
    mem::forget,
    sync::Arc,
};

use dirs::home_dir;
use gpui::*;
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt::layer, prelude::*, EnvFilter};

use explorer_common::*;
use explorer_component::*;
use explorer_local_provider::LocalFileSystemProvider;
use explorer_storage::*;

// ===== 数据转换函数 =====
// 将 storage 层的数据类型转换为 UI 组件所需的类型

/// 将 StorageRoot 转换为 RootItem
fn storage_root_to_root_item(root: &StorageRoot) -> RootItem {
    let provider_type = match root.provider_type {
        StorageProviderType::LocalFileSystem => ProviderType::LocalFileSystem,
        StorageProviderType::NetworkDrive => ProviderType::NetworkDrive,
        StorageProviderType::CloudStorage { .. } => ProviderType::CloudStorage,
    };

    RootItem {
        id: root.id.clone(),
        name: root.name.clone(),
        path: root.root_path.clone(),
        provider_type,
    }
}

/// 将 StorageEntry 转换为 FileItem
fn storage_entry_to_file_item(entry: &StorageEntry) -> FileItem {
    let item_type = match entry.entry_type {
        EntryType::File => ItemType::File,
        EntryType::Directory => ItemType::Directory,
        EntryType::Symlink => ItemType::Symlink,
    };

    FileItem {
        name: entry.name.clone(),
        path: entry.path.clone(),
        item_type,
        is_hidden: entry.is_hidden,
    }
}

// ===== Explorer 组件 =====

/// Explorer 主组件
pub struct Explorer {
    provider: Arc<dyn StorageProvider>,
    roots: Vec<StorageRoot>,
    current_path: String,
    entries: Vec<StorageEntry>,
    loading: bool,
    error: Option<String>,
}

impl Explorer {
    /// 创建 Explorer 实例
    pub fn new() -> Self {
        let provider: Arc<dyn StorageProvider> = Arc::new(LocalFileSystemProvider::new());

        Self {
            provider,
            roots: Vec::new(),
            current_path: String::new(),
            entries: Vec::new(),
            loading: true,
            error: None,
        }
    }

    /// 初始化 Explorer（启动异步数据加载）
    pub fn init(&mut self, window: &Window, cx: &mut Context<Self>) {
        tracing::info!("初始化 Explorer");

        let provider = self.provider.clone();
        cx.spawn_in(window, async move |this, cx| {
            tracing::info!("开始异步加载数据");

            // 在后台线程执行数据加载
            let ret = cx
                .background_executor()
                .spawn(async move {
                    // 加载存储根节点
                    let roots = provider.get_roots().await?;
                    tracing::info!("加载到 {} 个存储根节点", roots.len());

                    // 默认选择第一个根节点
                    let current_path = roots
                        .first()
                        .map(|r| r.root_path.clone())
                        .unwrap_or_else(|| "/".to_string());

                    // 加载初始目录
                    let entries = provider.list_entries(&current_path).await?;
                    tracing::info!("成功加载 {} 个条目", entries.len());

                    Ok::<_, anyhow::Error>((roots, current_path, entries))
                })
                .await;

            // 更新 UI
            let _ = cx.update(|_, cx| {
                let _ = this.update(cx, |explorer, cx| match ret {
                    Ok((roots, current_path, entries)) => {
                        explorer.roots = roots;
                        explorer.current_path = current_path;
                        explorer.entries = entries;
                        explorer.loading = false;
                        explorer.error = None;
                        tracing::info!("数据加载完成");
                        cx.notify();
                    }
                    Err(e) => {
                        tracing::error!("加载失败: {:?}", e);
                        explorer.loading = false;
                        explorer.error = Some(format!("加载失败: {}", e));
                        cx.notify();
                    }
                });
            });
        })
        .detach();
    }

    /// 加载指定目录
    pub fn load_directory(&mut self, path: String, window: &Window, cx: &mut Context<Self>) {
        tracing::info!("请求加载目录: {}", path);
        self.loading = true;
        self.error = None;
        self.current_path = path.clone();
        cx.notify();

        let provider = self.provider.clone();
        cx.spawn_in(window, {
            async move |this, cx| {
                // 在后台线程执行目录加载
                let ret = cx
                    .background_executor()
                    .spawn(async move { provider.list_entries(&path).await })
                    .await;

                // 更新 UI
                let _ = cx.update(|_, cx| {
                    let _ = this.update(cx, |explorer, cx| match ret {
                        Ok(entries) => {
                            tracing::info!("成功加载 {} 个条目", entries.len());
                            explorer.entries = entries;
                            explorer.loading = false;
                            explorer.error = None;
                            cx.notify();
                        }
                        Err(e) => {
                            tracing::error!("加载目录失败: {}", e);
                            explorer.loading = false;
                            explorer.error = Some(format!("加载失败: {}", e));
                            explorer.entries = Vec::new();
                            cx.notify();
                        }
                    });
                });
            }
        })
        .detach();
    }
}

impl Render for Explorer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let layout = SidebarLayout::new();

        // 转换 storage 数据类型为 UI 组件类型
        let root_items: Vec<RootItem> = self
            .roots
            .iter()
            .map(storage_root_to_root_item)
            .collect();

        let file_items: Vec<FileItem> = self
            .entries
            .iter()
            .map(storage_entry_to_file_item)
            .collect();

        let sidebar = Sidebar::new().roots(root_items);

        let file_list = FileList::new()
            .entries(file_items)
            .current_path(self.current_path.clone())
            .loading(self.loading)
            .error(self.error.clone());

        layout.render_with_children(sidebar, file_list)
    }
}

fn init() {
    // 设置日志目录
    let log_dir = home_dir()
        .map(|home| home.join(".explorer").join("logs"))
        .expect("Failed to find log dir");
    if !log_dir.exists() {
        create_dir_all(&log_dir).expect("Failed to create log dir");
    }

    // 根据编译模式设置日志级别
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    // 配置日志文件滚动
    let log_rolling = RollingFileAppender::builder()
        .rotation(Rotation::DAILY) // 按天滚动日志文件
        .filename_prefix("explorer") // 日志文件名前缀
        .filename_suffix("log") // 日志文件名后缀
        .build(&log_dir)
        .expect("Failed to create log file appender");
    let (non_blocking, _guard) = non_blocking(log_rolling);

    // 初始化 tracing subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level)))
        .with(layer().with_writer(stdout)) // 输出到标准输出
        .with(layer().with_writer(non_blocking).with_ansi(false)) // 输出到日志文件（不使用 ANSI 颜色）
        .init();
    forget(_guard); // 保持日志文件句柄
}

fn main() {
    let app = Application::new();
    app.run(move |cx| {
        // 初始化日志系统
        init();

        tracing::info!("Explorer 应用启动");
        tracing::info!("版本: {}", env!("CARGO_PKG_VERSION"));

        cx.activate(true);
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                tracing::info!("所有窗口已关闭，退出应用");
                cx.quit();
            }
        })
        .detach();

        let window_size = size(px(1280.), px(800.));
        let window_bounds = Bounds::centered(None, window_size, cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Explorer::new()),
        )
        .expect("failed to open window")
        .update(cx, |explorer, window, cx| {
            explorer.init(window, cx);
            window.activate_window();
        })
        .expect("failed to active window");
    });
}
