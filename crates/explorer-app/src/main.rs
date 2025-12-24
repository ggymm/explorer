use std::{collections::HashSet, fs::create_dir_all, io::stdout, mem::forget, sync::Arc};

use dirs::home_dir;
use gpui::{prelude::*, *};
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{EnvFilter, fmt::layer, prelude::*};

use explorer_common::*;
use explorer_component::{
    Assets, Breadcrumb, BreadcrumbItem, BreadcrumbState, GroupedList, Icon, IconName, List,
    ListGroup, ListItem, Resizable, ResizableState, Theme, TitleBar,
};
use explorer_local_provider::LocalFileSystemProvider;
use explorer_storage::*;

mod quick_access;

// ===== 辅助函数 =====

/// 将路径字符串解析为面包屑项
fn parse_path_to_breadcrumb_items(path: &str) -> Vec<BreadcrumbItem> {
    let mut items = Vec::new();

    // 处理 Windows 路径
    #[cfg(target_os = "windows")]
    {
        // 检查是否有盘符
        if let Some((drive, rest)) = path.split_once(':') {
            let drive_with_colon = format!("{}:", drive);
            items.push(BreadcrumbItem::new(
                drive_with_colon.clone(),
                drive_with_colon,
            ));

            // 处理盘符后的路径
            for segment in rest.split('\\').filter(|s| !s.is_empty()) {
                let current_path = if items.is_empty() {
                    segment.to_string()
                } else {
                    format!("{}\\{}", items.last().unwrap().value, segment)
                };
                items.push(BreadcrumbItem::new(segment, current_path));
            }
            return items;
        }
    }

    // Unix 风格路径
    if path == "/" {
        items.push(BreadcrumbItem::new("/", "/"));
        return items;
    }

    items.push(BreadcrumbItem::new("/", "/"));

    let mut current_path = String::from("");
    for segment in path.split('/').filter(|s| !s.is_empty()) {
        current_path.push('/');
        current_path.push_str(segment);
        items.push(BreadcrumbItem::new(segment, current_path.clone()));
    }

    items
}

// ===== 面板数据结构 =====

/// 面板节点枚举，用于构建面板树
#[derive(Clone)]
pub enum PanelNode {
    /// 叶子节点：包含实际的文件浏览器实例
    Leaf {
        id: PanelId,
        path: String,
        entries: Vec<FileItem>,
        loading: bool,
        error: Option<String>,
        bounds: Bounds<Pixels>,                    // 保存面板尺寸
        breadcrumb_state: Entity<BreadcrumbState>, // 面包屑状态
    },
    /// 分支节点：包含两个子面板和拆分方向
    Split {
        id: PanelId,
        axis: Axis,
        first: Box<PanelNode>,
        second: Box<PanelNode>,
        state: Entity<ResizableState>,
        bounds: Bounds<Pixels>, // 保存容器尺寸
    },
}

impl PanelNode {
    /// 创建新的叶子面板
    pub fn new_leaf(id: PanelId, path: String, cx: &mut App) -> Self {
        let breadcrumb_state = cx.new(|_| BreadcrumbState::new());
        Self::Leaf {
            id,
            path,
            entries: Vec::new(),
            loading: true,
            error: None,
            bounds: Bounds::default(),
            breadcrumb_state,
        }
    }

    /// 获取面板 ID
    pub fn get_id(&self) -> PanelId {
        match self {
            Self::Leaf { id, .. } => *id,
            Self::Split { id, .. } => *id,
        }
    }

    /// 拆分指定 ID 的叶子面板
    pub fn split_panel(
        &mut self,
        target_id: PanelId,
        axis: Axis,
        split_id: PanelId,
        new_leaf_id: PanelId,
        new_path: String,
        initial_size: Pixels,
        cx: &mut App,
    ) -> bool {
        match self {
            PanelNode::Leaf {
                id,
                path,
                entries,
                loading,
                error,
                bounds,
                breadcrumb_state,
            } if *id == target_id => {
                // 找到目标面板，执行拆分
                let old_path = path.clone();
                let old_id = *id;
                // 保留原面板的数据
                let old_entries = entries.clone();
                let old_loading = *loading;
                let old_error = error.clone();
                let old_bounds = *bounds;
                let old_breadcrumb_state = breadcrumb_state.clone();

                // 创建两个新的叶子节点：第一个保留原数据，第二个创建新面板
                let first = Box::new(PanelNode::Leaf {
                    id: old_id,
                    path: old_path,
                    entries: old_entries,
                    loading: old_loading,
                    error: old_error,
                    bounds: Bounds::default(),
                    breadcrumb_state: old_breadcrumb_state,
                });
                let second = Box::new(PanelNode::new_leaf(new_leaf_id, new_path, cx));

                // 创建 ResizableState，使用传入的 initial_size
                // range 设置为 0 到最大值，不限制拆分尺寸
                let state =
                    cx.new(|_| ResizableState::new(axis, initial_size, px(0.)..Pixels::MAX));

                // 替换当前节点为 Split 节点，使用新的 split_id
                *self = PanelNode::Split {
                    id: split_id, // 使用新的 Split ID
                    axis,
                    first,
                    second,
                    state,
                    bounds: old_bounds, // 使用原面板的尺寸
                };

                true
            }
            PanelNode::Split { first, second, .. } => {
                // 递归查找子节点
                first.split_panel(
                    target_id,
                    axis,
                    split_id,
                    new_leaf_id,
                    new_path.clone(),
                    initial_size,
                    cx,
                ) || second.split_panel(
                    target_id,
                    axis,
                    split_id,
                    new_leaf_id,
                    new_path,
                    initial_size,
                    cx,
                )
            }
            _ => false,
        }
    }

    /// 查找指定 ID 的面板
    pub fn find_panel(&self, target_id: PanelId) -> Option<&PanelNode> {
        match self {
            PanelNode::Leaf { id, .. } if *id == target_id => Some(self),
            PanelNode::Split {
                id, first, second, ..
            } => {
                if *id == target_id {
                    Some(self)
                } else {
                    first
                        .find_panel(target_id)
                        .or_else(|| second.find_panel(target_id))
                }
            }
            _ => None,
        }
    }

    /// 更新指定面板的数据
    pub fn update_panel_data(
        &mut self,
        target_id: PanelId,
        path: String,
        entries: Vec<FileItem>,
        loading: bool,
        error: Option<String>,
    ) -> bool {
        match self {
            PanelNode::Leaf {
                id,
                path: panel_path,
                entries: panel_entries,
                loading: panel_loading,
                error: panel_error,
                ..
            } if *id == target_id => {
                *panel_path = path;
                *panel_entries = entries;
                *panel_loading = loading;
                *panel_error = error;
                true
            }
            PanelNode::Split { first, second, .. } => {
                first.update_panel_data(
                    target_id,
                    path.clone(),
                    entries.clone(),
                    loading,
                    error.clone(),
                ) || second.update_panel_data(target_id, path, entries, loading, error)
            }
            _ => false,
        }
    }

    /// 更新指定面板的 bounds（仅用于 Leaf 节点）
    pub fn update_panel_bounds(&mut self, target_id: PanelId, new_bounds: Bounds<Pixels>) -> bool {
        match self {
            PanelNode::Leaf { id, bounds, .. } if *id == target_id => {
                *bounds = new_bounds;
                true
            }
            PanelNode::Split { first, second, .. } => {
                // Split 节点的 bounds 由 ResizableContainer 管理，只递归处理子节点
                first.update_panel_bounds(target_id, new_bounds)
                    || second.update_panel_bounds(target_id, new_bounds)
            }
            _ => false,
        }
    }

    /// 移除指定 ID 的面板
    /// 返回新的树结构（如果目标被移除则提升兄弟节点）
    pub fn remove_panel(self, target_id: PanelId) -> Option<PanelNode> {
        match self {
            PanelNode::Leaf { id, .. } => {
                // 叶子节点：如果是目标节点，返回 None 表示移除
                if id == target_id { None } else { Some(self) }
            }
            PanelNode::Split {
                first,
                second,
                id,
                axis,
                state,
                bounds,
            } => {
                // 尝试从子节点中移除
                let first_result = first.remove_panel(target_id);
                let second_result = second.remove_panel(target_id);

                match (first_result, second_result) {
                    (None, Some(node)) => Some(node), // 第一个子节点被移除，提升第二个
                    (Some(node), None) => Some(node), // 第二个子节点被移除，提升第一个
                    (Some(f), Some(s)) => {
                        // 两个子节点都保留，重建 Split
                        Some(PanelNode::Split {
                            id,
                            axis,
                            first: Box::new(f),
                            second: Box::new(s),
                            state,
                            bounds,
                        })
                    }
                    (None, None) => None, // 两个都被移除（不应该发生）
                }
            }
        }
    }

    /// 查找下一个激活面板 ID（深度优先）
    pub fn find_next_panel_id(&self, exclude_id: PanelId) -> Option<PanelId> {
        match self {
            PanelNode::Leaf { id, .. } => {
                if *id != exclude_id {
                    Some(*id)
                } else {
                    None
                }
            }
            PanelNode::Split { first, second, .. } => first
                .find_next_panel_id(exclude_id)
                .or_else(|| second.find_next_panel_id(exclude_id)),
        }
    }

    /// 统计叶子面板数量
    pub fn count_leaves(&self) -> usize {
        match self {
            PanelNode::Leaf { .. } => 1,
            PanelNode::Split { first, second, .. } => first.count_leaves() + second.count_leaves(),
        }
    }
}

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
    selected_sidebar_path: Option<String>,
    // 面板树管理
    panel_tree: PanelNode,
    active_panel_id: Option<PanelId>,
    next_panel_id: u64,
    // 文件选中状态
    selected_items: HashSet<String>,
    last_selected_index: Option<usize>,
}

impl Explorer {
    /// 创建 Explorer 实例
    pub fn new(cx: &mut Context<Self>) -> Self {
        let provider: Arc<dyn StorageProvider> = Arc::new(LocalFileSystemProvider::new());

        // 使用用户主目录作为默认路径，如果获取失败则使用根目录
        let default_path = home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        // 创建初始的单面板树
        let initial_panel_id = 0;
        let panel_tree = PanelNode::new_leaf(initial_panel_id, default_path.clone(), cx);

        Self {
            provider,
            roots: Vec::new(),
            selected_sidebar_path: Some(default_path),
            panel_tree,
            active_panel_id: Some(initial_panel_id),
            next_panel_id: 1,
            selected_items: HashSet::new(),
            last_selected_index: None,
        }
    }

    /// 横向拆分面板
    pub fn split_panel_horizontal(&mut self, window: &Window, cx: &mut Context<Self>) {
        if let Some(active_id) = self.active_panel_id {
            // 获取当前激活面板的路径和尺寸
            let (path, panel_bounds) = if let Some(panel) = self.panel_tree.find_panel(active_id) {
                match panel {
                    PanelNode::Leaf { path, bounds, .. } => (path.clone(), *bounds),
                    PanelNode::Split { state, first, .. } => {
                        // Split 节点：从 ResizableState 获取 bounds，路径从第一个子面板获取
                        let bounds = state.read(cx).bounds();
                        let path = match first.as_ref() {
                            PanelNode::Leaf { path, .. } => path.clone(),
                            PanelNode::Split { first, .. } => {
                                // 递归获取最左侧/最上方的叶子节点路径
                                let mut node = first.as_ref();
                                loop {
                                    match node {
                                        PanelNode::Leaf { path, .. } => break path.clone(),
                                        PanelNode::Split { first, .. } => node = first.as_ref(),
                                    }
                                }
                            }
                        };
                        (path, bounds)
                    }
                }
            } else {
                let default_path = home_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "/".to_string());
                (default_path, Bounds::default())
            };

            // 分配两个新 ID：一个给 Split 节点，一个给新的 Leaf 节点
            let split_id = self.next_panel_id;
            self.next_panel_id += 1;
            let new_leaf_id = self.next_panel_id;
            self.next_panel_id += 1;

            // 横向拆分：使用面板实际宽度的 50%
            let initial_width = panel_bounds.size.width / 2.0;

            // 执行拆分
            if self.panel_tree.split_panel(
                active_id,
                Axis::Horizontal,
                split_id,
                new_leaf_id,
                path.clone(),
                initial_width,
                cx,
            ) {
                tracing::info!(
                    "横向拆分面板 {} 成功，Split ID: {}，新 Leaf ID: {}",
                    active_id,
                    split_id,
                    new_leaf_id
                );
                // 加载新面板的数据
                self.load_directory_for_panel(new_leaf_id, path, window, cx);
                cx.notify();
            }
        }
    }

    /// 纵向拆分面板
    pub fn split_panel_vertical(&mut self, window: &Window, cx: &mut Context<Self>) {
        if let Some(active_id) = self.active_panel_id {
            // 获取当前激活面板的路径和尺寸
            let (path, panel_bounds) = if let Some(panel) = self.panel_tree.find_panel(active_id) {
                match panel {
                    PanelNode::Leaf { path, bounds, .. } => (path.clone(), *bounds),
                    PanelNode::Split { state, first, .. } => {
                        // Split 节点：从 ResizableState 获取 bounds，路径从第一个子面板获取
                        let bounds = state.read(cx).bounds();
                        let path = match first.as_ref() {
                            PanelNode::Leaf { path, .. } => path.clone(),
                            PanelNode::Split { first, .. } => {
                                // 递归获取最左侧/最上方的叶子节点路径
                                let mut node = first.as_ref();
                                loop {
                                    match node {
                                        PanelNode::Leaf { path, .. } => break path.clone(),
                                        PanelNode::Split { first, .. } => node = first.as_ref(),
                                    }
                                }
                            }
                        };
                        (path, bounds)
                    }
                }
            } else {
                let default_path = home_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "/".to_string());
                (default_path, Bounds::default())
            };

            // 分配两个新 ID：一个给 Split 节点，一个给新的 Leaf 节点
            let split_id = self.next_panel_id;
            self.next_panel_id += 1;
            let new_leaf_id = self.next_panel_id;
            self.next_panel_id += 1;

            // 纵向拆分：使用面板实际高度的 50%
            let initial_height = panel_bounds.size.height / 2.0;

            // 执行拆分
            if self.panel_tree.split_panel(
                active_id,
                Axis::Vertical,
                split_id,
                new_leaf_id,
                path.clone(),
                initial_height,
                cx,
            ) {
                tracing::info!(
                    "纵向拆分面板 {} 成功，Split ID: {}，新 Leaf ID: {}",
                    active_id,
                    split_id,
                    new_leaf_id
                );
                // 加载新面板的数据
                self.load_directory_for_panel(new_leaf_id, path, window, cx);
                cx.notify();
            }
        }
    }

    /// 设置激活面板
    pub fn set_active_panel(&mut self, panel_id: PanelId, cx: &mut Context<Self>) {
        self.active_panel_id = Some(panel_id);
        cx.notify();
    }

    // ===== 文件选中状态管理 =====

    /// 检查文件是否被选中
    pub fn is_selected(&self, path: &str) -> bool {
        self.selected_items.contains(path)
    }

    /// 添加选中项
    pub fn add_selection(&mut self, path: String, cx: &mut Context<Self>) {
        self.selected_items.insert(path);
        cx.notify();
    }

    /// 移除选中项
    pub fn remove_selection(&mut self, path: &str, cx: &mut Context<Self>) {
        self.selected_items.remove(path);
        cx.notify();
    }

    /// 清空所有选中项
    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selected_items.clear();
        self.last_selected_index = None;
        cx.notify();
    }

    /// 切换选中状态
    pub fn toggle_selection(&mut self, path: String, index: usize, cx: &mut Context<Self>) {
        if self.selected_items.contains(&path) {
            self.selected_items.remove(&path);
        } else {
            self.selected_items.insert(path);
        }
        self.last_selected_index = Some(index);
        cx.notify();
    }

    /// 设置单个选中项（清除其他选中）
    pub fn set_single_selection(&mut self, path: String, index: usize, cx: &mut Context<Self>) {
        self.selected_items.clear();
        self.selected_items.insert(path);
        self.last_selected_index = Some(index);
        cx.notify();
    }

    /// 范围选择（Shift + 点击）
    pub fn select_range(&mut self, paths: &[String], current_index: usize, cx: &mut Context<Self>) {
        if let Some(last_index) = self.last_selected_index {
            let start = last_index.min(current_index);
            let end = last_index.max(current_index);
            for i in start..=end {
                if let Some(path) = paths.get(i) {
                    self.selected_items.insert(path.clone());
                }
            }
        } else {
            // 没有上次选中项，则单选当前项
            if let Some(path) = paths.get(current_index) {
                self.set_single_selection(path.clone(), current_index, cx);
                return;
            }
        }
        cx.notify();
    }

    /// 更新面板的 bounds
    pub fn update_panel_bounds(&mut self, panel_id: PanelId, bounds: Bounds<Pixels>) {
        self.panel_tree.update_panel_bounds(panel_id, bounds);
    }

    /// 为指定面板加载目录
    pub fn load_directory_for_panel(
        &mut self,
        panel_id: PanelId,
        path: String,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        tracing::info!("为面板 {} 加载目录: {}", panel_id, path);

        // 切换目录时清除文件列表的选中状态
        self.selected_items.clear();
        self.last_selected_index = None;

        // 更新面板状态为加载中
        self.panel_tree
            .update_panel_data(panel_id, path.clone(), Vec::new(), true, None);
        cx.notify();

        let provider = self.provider.clone();
        let path_clone = path.clone();
        cx.spawn_in(window, async move |this, cx| {
            // 在后台线程执行目录加载
            let ret = cx
                .background_executor()
                .spawn(async move { provider.list_entries(&path).await })
                .await;

            // 更新 UI
            let _ = cx.update(|_, cx| {
                let _ = this.update(cx, |explorer, cx| match ret {
                    Ok(mut entries) => {
                        // 排序：非隐藏文件在前，然后按目录/文件分类，最后按名称排序
                        entries.sort_by(|a, b| {
                            // 首先按是否隐藏排序
                            match (a.is_hidden, b.is_hidden) {
                                (false, true) => std::cmp::Ordering::Less,
                                (true, false) => std::cmp::Ordering::Greater,
                                _ => {
                                    // 然后按类型排序（目录在前）
                                    match (&a.item_type, &b.item_type) {
                                        (ItemType::Directory, ItemType::File) => std::cmp::Ordering::Less,
                                        (ItemType::File, ItemType::Directory) => std::cmp::Ordering::Greater,
                                        _ => {
                                            // 最后按名称排序（不区分大小写）
                                            a.name.to_lowercase().cmp(&b.name.to_lowercase())
                                        }
                                    }
                                }
                            }
                        });

                        tracing::info!("面板 {} 成功加载 {} 个条目", panel_id, entries.len());
                        explorer.panel_tree.update_panel_data(
                            panel_id,
                            path_clone.clone(),
                            entries,
                            false,
                            None,
                        );
                        cx.notify();
                    }
                    Err(e) => {
                        tracing::error!("面板 {} 加载失败: {:?}", panel_id, e);
                        explorer.panel_tree.update_panel_data(
                            panel_id,
                            path_clone,
                            Vec::new(),
                            false,
                            Some(format!("加载失败: {}", e)),
                        );
                        cx.notify();
                    }
                });
            });
        })
        .detach();
    }

    /// 初始化 Explorer（启动异步数据加载）
    pub fn init(&mut self, window: &Window, cx: &mut Context<Self>) {
        tracing::info!("初始化 Explorer");

        let provider = self.provider.clone();
        // 获取初始面板的路径
        let initial_panel_id = self.active_panel_id.unwrap_or(0);
        let initial_path = if let Some(PanelNode::Leaf { path, .. }) =
            self.panel_tree.find_panel(initial_panel_id)
        {
            path.clone()
        } else {
            home_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "/".to_string())
        };

        let initial_path_clone = initial_path.clone();
        cx.spawn_in(window, async move |this, cx| {
            tracing::info!("开始异步加载数据");

            // 在后台线程执行数据加载
            let ret = cx
                .background_executor()
                .spawn(async move {
                    // 加载存储根节点
                    let roots = provider.get_roots().await?;
                    tracing::info!("加载到 {} 个存储根节点", roots.len());

                    // 加载初始目录
                    let entries = provider.list_entries(&initial_path).await?;
                    tracing::info!("成功加载 {} 个条目", entries.len());

                    Ok::<_, anyhow::Error>((roots, entries))
                })
                .await;

            // 更新 UI
            let _ = cx.update(|_, cx| {
                let _ = this.update(cx, |explorer, cx| match ret {
                    Ok((roots, mut entries)) => {
                        // 排序：非隐藏文件在前，然后按目录/文件分类，最后按名称排序
                        entries.sort_by(|a, b| {
                            // 首先按是否隐藏排序
                            match (a.is_hidden, b.is_hidden) {
                                (false, true) => std::cmp::Ordering::Less,
                                (true, false) => std::cmp::Ordering::Greater,
                                _ => {
                                    // 然后按类型排序（目录在前）
                                    match (&a.item_type, &b.item_type) {
                                        (ItemType::Directory, ItemType::File) => std::cmp::Ordering::Less,
                                        (ItemType::File, ItemType::Directory) => std::cmp::Ordering::Greater,
                                        _ => {
                                            // 最后按名称排序（不区分大小写）
                                            a.name.to_lowercase().cmp(&b.name.to_lowercase())
                                        }
                                    }
                                }
                            }
                        });

                        explorer.roots = roots;
                        // 更新初始面板的数据
                        explorer.panel_tree.update_panel_data(
                            initial_panel_id,
                            initial_path_clone.clone(),
                            entries,
                            false,
                            None,
                        );
                        tracing::info!("数据加载完成");
                        cx.notify();
                    }
                    Err(e) => {
                        tracing::error!("加载失败: {:?}", e);
                        explorer.panel_tree.update_panel_data(
                            initial_panel_id,
                            initial_path_clone,
                            Vec::new(),
                            false,
                            Some(format!("加载失败: {}", e)),
                        );
                        cx.notify();
                    }
                });
            });
        })
        .detach();
    }

    /// 加载指定目录（为当前激活的面板）
    pub fn load_directory(&mut self, path: String, window: &Window, cx: &mut Context<Self>) {
        if let Some(active_id) = self.active_panel_id {
            self.load_directory_for_panel(active_id, path, window, cx);
        }
    }

    /// 关闭指定面板
    pub fn close_panel(&mut self, panel_id: PanelId, cx: &mut Context<Self>) {
        // 检查是否是最后一个面板
        if self.panel_tree.count_leaves() <= 1 {
            tracing::warn!("无法关闭最后一个面板");
            return;
        }

        // 如果关闭的是激活面板，需要找到新的激活面板
        let need_new_active = self.active_panel_id == Some(panel_id);
        let new_active_id = if need_new_active {
            self.panel_tree.find_next_panel_id(panel_id)
        } else {
            None
        };

        // 移除面板
        let old_tree = std::mem::replace(
            &mut self.panel_tree,
            PanelNode::new_leaf(9999, "/".to_string(), cx), // 临时占位
        );

        if let Some(new_tree) = old_tree.clone().remove_panel(panel_id) {
            self.panel_tree = new_tree;

            // 更新激活面板
            if need_new_active {
                if let Some(new_id) = new_active_id {
                    self.active_panel_id = Some(new_id);
                } else {
                    // 没找到新的激活面板，使用树中的第一个
                    self.active_panel_id = self.panel_tree.find_next_panel_id(panel_id);
                }
            }

            tracing::info!("成功关闭面板: {}", panel_id);
            cx.notify();
        } else {
            // 移除失败，恢复原树
            self.panel_tree = old_tree;
            tracing::error!("关闭面板失败: {}", panel_id);
        }
    }
}

impl Render for Explorer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        // 获取快捷访问项
        let quick_access_items = quick_access::get_quick_access_items();

        // 获取 Explorer 实体的弱引用用于事件回调
        let this_entity = cx.entity().downgrade();

        // 构建侧边栏
        let sidebar = self.render_sidebar(quick_access_items, theme, &this_entity);

        // 构建面板树（递归渲染）
        let panel_content = self.render_panel_node(&self.panel_tree.clone(), theme, &this_entity);

        // 主内容区域（侧边栏 + 面板）
        let main_content = Resizable::new("explorer-layout", sidebar, panel_content)
            .size(px(240.))
            .range(px(180.)..px(480.));

        // 构建标题栏
        let this_clone_h = this_entity.clone();
        let this_clone_v = this_entity.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                // 标题栏
                TitleBar::new().child(
                    div()
                        .flex()
                        .items_center()
                        .justify_end()
                        .gap(theme.spacing.sm)
                        .pr(theme.spacing.md)
                        .child(
                            // 横向拆分按钮
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .size(px(28.))
                                .rounded(px(4.))
                                .cursor_pointer()
                                .hover(|style| style.bg(theme.colors.muted))
                                .child(
                                    svg()
                                        .path("icons/columns-split.svg")
                                        .size(px(16.))
                                        .text_color(theme.colors.foreground),
                                )
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    if let Some(this) = this_clone_h.upgrade() {
                                        let _ = this.update(cx, |explorer, cx| {
                                            explorer.split_panel_horizontal(window, cx);
                                        });
                                    }
                                }),
                        )
                        .child(
                            // 纵向拆分按钮
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .size(px(28.))
                                .rounded(px(4.))
                                .cursor_pointer()
                                .hover(|style| style.bg(theme.colors.muted))
                                .child(
                                    svg()
                                        .path("icons/rows-split.svg")
                                        .size(px(16.))
                                        .text_color(theme.colors.foreground),
                                )
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    if let Some(this) = this_clone_v.upgrade() {
                                        let _ = this.update(cx, |explorer, cx| {
                                            explorer.split_panel_vertical(window, cx);
                                        });
                                    }
                                }),
                        ),
                ),
            )
            .child(
                // 主内容区域
                div().flex_1().child(main_content),
            )
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
                                                theme.colors.brand_foreground
                                            } else {
                                                theme.colors.foreground
                                            }))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(if is_selected {
                                                        theme.colors.brand_foreground
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

    /// 递归渲染面板节点
    fn render_panel_node(
        &self,
        node: &PanelNode,
        theme: &Theme,
        this_entity: &WeakEntity<Self>,
    ) -> AnyElement {
        match node {
            PanelNode::Leaf {
                id,
                path,
                entries,
                loading,
                error,
                breadcrumb_state,
                ..
            } => {
                // 渲染叶子面板：标题栏 + 文件列表
                let panel_id = *id;
                let is_active = self.active_panel_id == Some(panel_id);
                let this_clone_list = this_entity.clone();
                let this_clone_title = this_entity.clone();

                // 解析路径为面包屑项
                let breadcrumb_items = parse_path_to_breadcrumb_items(path);

                let content = div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .bg(theme.colors.background)
                    .child(
                        div().w_full().child(
                            // 标题栏（面包屑导航）
                            Breadcrumb::new()
                                .items(breadcrumb_items)
                                .active(is_active)
                                .state(breadcrumb_state.clone())
                                .on_navigate(move |clicked_path, window, cx| {
                                    if let Some(this) = this_clone_title.upgrade() {
                                        let _ = this.update(cx, |explorer, cx| {
                                            // 先设置为激活面板
                                            explorer.set_active_panel(panel_id, cx);
                                            // 加载点击的路径
                                            explorer.load_directory_for_panel(
                                                panel_id,
                                                clicked_path,
                                                window,
                                                cx,
                                            );
                                        });
                                    }
                                })
                                .suffix(
                                    // 关闭按钮（后缀）
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w(px(24.))
                                        .h(px(24.))
                                        .rounded(theme.radius.sm)
                                        .cursor_pointer()
                                        .hover(|style| {
                                            style
                                                .bg(theme.colors.danger)
                                                .text_color(theme.colors.danger_foreground)
                                        })
                                        .child(IconName::Close)
                                        .on_mouse_down(MouseButton::Left, {
                                            let this_clone_close = this_entity.clone();
                                            move |_, _, cx| {
                                                tracing::info!("关闭面板: {}", panel_id);
                                                if let Some(this) = this_clone_close.upgrade() {
                                                    let _ = this.update(cx, |explorer, cx| {
                                                        explorer.close_panel(panel_id, cx);
                                                    });
                                                }
                                            }
                                        }),
                                ),
                        ),
                    )
                    .child(
                        // 文件列表
                        div()
                            .id(SharedString::from(format!("panel-{}", panel_id)))
                            .flex_1()
                            .overflow_scroll()
                            .p(theme.spacing.sm)
                            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                if let Some(this) = this_clone_list.upgrade() {
                                    let _ = this.update(cx, |explorer, cx| {
                                        explorer.set_active_panel(panel_id, cx);
                                    });
                                }
                            })
                            .child(
                                List::new()
                                    .items(entries.clone())
                                    .loading(*loading)
                                    .error(error.clone())
                                    .empty_text("目录为空")
                                    .loading_text("加载中...")
                                    .render_item({
                                        let this_entity_clone = this_entity.clone();
                                        let entries_clone = entries.clone();
                                        let selected_items = self.selected_items.clone();
                                        move |entry, theme| {
                                            let icon = match entry.item_type {
                                                ItemType::Directory => {
                                                    Icon::new(IconName::FolderClosed)
                                                }
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
                                            let this_clone_double = this_entity_clone.clone();
                                            let this_clone_click = this_entity_clone.clone();

                                            // 查找当前项的索引
                                            let entry_index = entries_clone
                                                .iter()
                                                .position(|e| e.path == entry.path)
                                                .unwrap_or(0);

                                            // 检查是否被选中
                                            let is_selected = selected_items.contains(&entry.path);

                                            let mut item =
                                                ListItem::new(entry.path.clone())
                                                    .selected(is_selected)
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(theme.spacing.sm)
                                                            .child(icon.text_color(
                                                                theme.colors.foreground,
                                                            ))
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .text_color(name_color)
                                                                    .child(entry.name.clone()),
                                                            ),
                                                    );

                                            // 所有类型都添加单击事件（选中）
                                            let entry_path_click = entry.path.clone();
                                            let entries_for_click = entries_clone.clone();
                                            item = item.on_click(move |event, cx| {
                                                if let Some(this) = this_clone_click.upgrade() {
                                                    let _ = this.update(cx, |explorer, cx| {
                                                        explorer.set_active_panel(panel_id, cx);

                                                        // 检测修饰键
                                                        let modifiers = event.modifiers();

                                                        if modifiers.shift {
                                                            // Shift + 点击：范围选择
                                                            let paths: Vec<String> =
                                                                entries_for_click
                                                                    .iter()
                                                                    .map(|e| e.path.clone())
                                                                    .collect();
                                                            explorer.select_range(
                                                                &paths,
                                                                entry_index,
                                                                cx,
                                                            );
                                                        } else if modifiers.platform
                                                            || modifiers.control
                                                        {
                                                            // Ctrl/Cmd + 点击：切换选中（platform 在 macOS 上是 Cmd）
                                                            explorer.toggle_selection(
                                                                entry_path_click.clone(),
                                                                entry_index,
                                                                cx,
                                                            );
                                                        } else {
                                                            // 普通点击：单选
                                                            explorer.set_single_selection(
                                                                entry_path_click.clone(),
                                                                entry_index,
                                                                cx,
                                                            );
                                                        }
                                                    });
                                                }
                                            });

                                            // 为目录额外添加双击事件（进入目录）
                                            if entry_type == ItemType::Directory {
                                                item = item.on_double_click(move |window, cx| {
                                                    tracing::info!("双击文件夹: {}", entry_path);
                                                    if let Some(this) = this_clone_double.upgrade()
                                                    {
                                                        let path = entry_path.clone();
                                                        let _ = this.update(cx, |explorer, cx| {
                                                            explorer.set_active_panel(panel_id, cx);
                                                            explorer
                                                                .load_directory(path, window, cx);
                                                        });
                                                    }
                                                });
                                            }

                                            item.into_any_element()
                                        }
                                    }),
                            ),
                    )
                    .into_any_element();

                // 使用 PanelContainer 包装以捕获 bounds
                PanelContainer {
                    panel_id,
                    explorer: this_entity.clone(),
                    content,
                }
                .into_any_element()
            }
            PanelNode::Split {
                id,
                axis,
                first,
                second,
                state,
                ..
            } => {
                // 渲染分支节点：使用 Resizable 包装两个子面板
                let panel_id = *id;
                let first_panel = self.render_panel_node(first, theme, this_entity);
                let second_panel = self.render_panel_node(second, theme, this_entity);

                Resizable::new(format!("split-{}", panel_id), first_panel, second_panel)
                    .axis(*axis)
                    .with_state(state.clone())
                    .into_any_element()
            }
        }
    }

    /// 渲染文件列表（已废弃，保留是为了兼容旧代码）
    #[allow(dead_code)]
    fn render_file_list_deprecated(
        &self,
        _theme: &Theme,
        _this_entity: WeakEntity<Self>,
    ) -> impl IntoElement {
        // 这个方法已经被 render_panel_node 替代
        div().child("已废弃的方法")
    }
}

// ===== 面板容器元素（用于捕获 Leaf 面板的 bounds）=====

/// 面板容器元素，用于捕获 Leaf 面板的实际渲染尺寸
struct PanelContainer {
    panel_id: PanelId,
    explorer: WeakEntity<Explorer>,
    content: AnyElement,
}

impl IntoElement for PanelContainer {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for PanelContainer {
    type RequestLayoutState = LayoutId;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(ElementId::Name(
            format!("panel-container-{}", self.panel_id).into(),
        ))
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = self.content.request_layout(window, cx);
        (layout_id, layout_id)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        // 更新面板的 bounds
        if let Some(explorer) = self.explorer.upgrade() {
            let panel_id = self.panel_id;
            let _ = explorer.update(cx, |explorer, _| {
                explorer.update_panel_bounds(panel_id, bounds);
            });
        }

        self.content.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.content.paint(window, cx);
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
                titlebar: Some(TitleBar::titlebar_options()),
                ..Default::default()
            },
            |_, cx| cx.new(Explorer::new),
        )
        .expect("failed to open window")
        .update(cx, |explorer, window, cx| {
            explorer.init(window, cx);
            window.activate_window();
        })
        .expect("failed to active window");
    });
}
