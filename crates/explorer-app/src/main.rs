use std::{fs::create_dir_all, io::stdout, mem::forget, sync::Arc};

use dirs::home_dir;
use gpui::{prelude::*, *};
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{EnvFilter, fmt::layer, prelude::*};

use explorer_common::*;
use explorer_component::{
    Assets, GroupedList, Icon, IconName, List, ListGroup, ListItem, Resizable, Theme,
};
use explorer_local_provider::LocalFileSystemProvider;
use explorer_storage::*;

mod quick_access;

// ===== 侧边栏数据类型 =====

/// 侧边栏项
#[derive(Clone)]
struct SidebarItem {
    name: String,
    path: String,
    icon_name: IconName,
}

impl From<&QuickAccessItem> for SidebarItem {
    fn from(item: &QuickAccessItem) -> Self {
        Self {
            name: item.name.clone(),
            path: item.path.clone(),
            icon_name: IconName::Folder,
        }
    }
}

impl From<&RootItem> for SidebarItem {
    fn from(item: &RootItem) -> Self {
        Self {
            name: item.name.clone(),
            path: item.path.clone(),
            icon_name: match item.provider_type {
                ProviderType::LocalFileSystem => IconName::FolderClosed,
                ProviderType::NetworkDrive => IconName::FolderClosed,
                ProviderType::CloudStorage { .. } => IconName::FolderClosed,
            },
        }
    }
}

// ===== Explorer 组件 =====

/// Explorer 主组件
pub struct Explorer {
    provider: Arc<dyn StorageProvider>,
    roots: Vec<RootItem>,
    current_path: String,
    selected_sidebar_path: Option<String>,
    entries: Vec<FileItem>,
    loading: bool,
    error: Option<String>,
}

impl Explorer {
    /// 创建 Explorer 实例
    pub fn new() -> Self {
        let provider: Arc<dyn StorageProvider> = Arc::new(LocalFileSystemProvider::new());

        // 使用用户主目录作为默认路径，如果获取失败则使用根目录
        let default_path = home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        Self {
            provider,
            roots: Vec::new(),
            current_path: default_path.clone(),
            selected_sidebar_path: Some(default_path),
            entries: Vec::new(),
            loading: true,
            error: None,
        }
    }

    /// 初始化 Explorer（启动异步数据加载）
    pub fn init(&mut self, window: &Window, cx: &mut Context<Self>) {
        tracing::info!("初始化 Explorer");

        let provider = self.provider.clone();
        let current_path = self.current_path.clone();
        cx.spawn_in(window, async move |this, cx| {
            tracing::info!("开始异步加载数据");

            // 在后台线程执行数据加载
            let ret = cx
                .background_executor()
                .spawn(async move {
                    // 加载存储根节点
                    let roots = provider.get_roots().await?;
                    tracing::info!("加载到 {} 个存储根节点", roots.len());

                    // 加载初始目录（使用用户主目录）
                    let entries = provider.list_entries(&current_path).await?;
                    tracing::info!("成功加载 {} 个条目", entries.len());

                    Ok::<_, anyhow::Error>((roots, entries))
                })
                .await;

            // 更新 UI
            let _ = cx.update(|_, cx| {
                let _ = this.update(cx, |explorer, cx| match ret {
                    Ok((roots, entries)) => {
                        explorer.roots = roots;
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        // 获取快捷访问项
        let quick_access_items = quick_access::get_quick_access_items();

        // 获取 Explorer 实体的弱引用用于事件回调
        let this_entity = cx.entity().downgrade();
        let this_entity_file = this_entity.clone();

        // 构建侧边栏
        let sidebar = self.render_sidebar(quick_access_items, theme, &this_entity);

        // 构建文件列表
        let file_list = self.render_file_list(theme, this_entity_file);

        // 使用可调整大小的布局组件
        Resizable::new("explorer-layout", sidebar, file_list)
            .size(px(240.))
            .range(px(180.)..px(480.))
    }
}

impl Explorer {
    /// 渲染侧边栏
    fn render_sidebar(
        &self,
        quick_access_items: Vec<QuickAccessItem>,
        theme: &Theme,
        this_entity: &WeakEntity<Self>,
    ) -> impl IntoElement {
        let selected_path = self.selected_sidebar_path.clone();

        // 构建分组数据
        let mut groups = Vec::new();

        // 快捷访问分组
        if !quick_access_items.is_empty() {
            let items: Vec<SidebarItem> =
                quick_access_items.iter().map(SidebarItem::from).collect();
            groups.push(ListGroup::new("快捷访问", items));
        }

        // 存储位置分组
        if !self.roots.is_empty() {
            let items: Vec<SidebarItem> = self.roots.iter().map(SidebarItem::from).collect();
            groups.push(ListGroup::new("存储位置", items));
        }

        let this_entity_clone = this_entity.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.colors.background)
            .border_r_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .id("sidebar-content")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_scroll()
                    .p(theme.spacing.md)
                    .child(
                        GroupedList::new()
                            .groups(groups)
                            .render_item(move |item, theme| {
                                let is_selected = selected_path
                                    .as_ref()
                                    .map(|p| p == &item.path)
                                    .unwrap_or(false);

                                let icon = Icon::new(item.icon_name);
                                let item_path = item.path.clone();
                                let this_clone = this_entity_clone.clone();

                                ListItem::new(item.path.clone())
                                    .selected(is_selected)
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(theme.spacing.sm)
                                            .child(icon.text_color(if is_selected {
                                                theme.colors.accent_foreground
                                            } else {
                                                theme.colors.foreground
                                            }))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(if is_selected {
                                                        theme.colors.accent_foreground
                                                    } else {
                                                        theme.colors.foreground
                                                    })
                                                    .child(item.name.clone()),
                                            ),
                                    )
                                    .on_click(move |window, cx| {
                                        tracing::info!("点击侧边栏项: {}", item_path);
                                        if let Some(this) = this_clone.upgrade() {
                                            let path = item_path.clone();
                                            let _ = this.update(cx, |explorer, cx| {
                                                explorer.selected_sidebar_path = Some(path.clone());
                                                explorer.load_directory(path, window, cx);
                                            });
                                        }
                                    })
                                    .into_any_element()
                            }),
                    ),
            )
    }

    /// 渲染文件列表
    fn render_file_list(&self, theme: &Theme, this_entity: WeakEntity<Self>) -> impl IntoElement {
        let entries = self.entries.clone();
        let loading = self.loading;
        let error = self.error.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.colors.background)
            .child(
                // 路径栏
                div()
                    .flex()
                    .items_center()
                    .p(theme.spacing.md)
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.muted_foreground)
                            .child(self.current_path.clone()),
                    ),
            )
            .child(
                // 文件列表内容 - 使用 List 组件
                div()
                    .id("file-list-content")
                    .flex_1()
                    .overflow_scroll()
                    .p(theme.spacing.sm)
                    .child(
                        List::new()
                            .items(entries)
                            .loading(loading)
                            .error(error)
                            .empty_text("目录为空")
                            .loading_text("加载中...")
                            .render_item(move |entry, theme| {
                                let icon = match entry.item_type {
                                    ItemType::Directory => Icon::new(IconName::FolderClosed),
                                    ItemType::File => Icon::new(IconName::File),
                                    ItemType::Symlink => Icon::new(IconName::File),
                                };

                                let name_color = if entry.is_hidden {
                                    theme.colors.muted_foreground
                                } else {
                                    theme.colors.foreground
                                };

                                let entry_path = entry.path.clone();
                                let entry_type = entry.item_type;
                                let this_clone = this_entity.clone();

                                let mut item = ListItem::new(entry.path.clone()).child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(theme.spacing.sm)
                                        .child(icon.text_color(theme.colors.foreground))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(name_color)
                                                .child(entry.name.clone()),
                                        ),
                                );

                                // 只为目录添加双击事件
                                if entry_type == ItemType::Directory {
                                    item = item.on_double_click(move |window, cx| {
                                        tracing::info!("双击文件夹: {}", entry_path);
                                        if let Some(this) = this_clone.upgrade() {
                                            let path = entry_path.clone();
                                            let _ = this.update(cx, |explorer, cx| {
                                                explorer.load_directory(path, window, cx);
                                            });
                                        }
                                    });
                                }

                                item.into_any_element()
                            }),
                    ),
            )
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
    let app = Application::new().with_assets(Assets);
    app.run(move |cx| {
        // 初始化日志系统
        init();

        tracing::info!("Explorer 应用启动");
        tracing::info!("版本: {}", env!("CARGO_PKG_VERSION"));

        // 初始化全局主题（使用暗色主题）
        cx.set_global(Theme::dark());

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
