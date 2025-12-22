# 设计文档：项目结构与基础布局

## 上下文

Explorer 是一个全新的跨平台文件浏览器项目，需要从零开始建立项目结构。项目采用 Rust 编写，使用 GPUI 框架构建 UI。这是项目的第一个重大变更，需要做出关键的架构决策。

### 约束条件
- 必须使用 GPUI 框架（所有 UI 组件自实现）
- 必须支持跨平台（macOS、Linux、Windows）
- 需要模块化设计，便于后续功能扩展
- 需要考虑性能（异步 IO、响应式 UI）

### 利益相关者
- 开发者：需要清晰的模块边界和开发流程
- 最终用户：需要流畅的用户体验

## 目标 / 非目标

### 目标
- 建立可扩展的 Cargo workspace 结构
- 实现基础的左右分栏布局
- 建立文件系统读取和状态管理机制
- 为后续功能开发提供清晰的架构基础

### 非目标
- 完整的文件操作功能（留待后续实现）
- 复杂的 UI 组件（如多标签页、分栏浏览）
- 主题系统和完整样式
- 性能优化（如虚拟滚动）

## 架构决策

### 1. Cargo Workspace 结构

**决策**：采用 monorepo 方式，使用 Cargo workspace 管理多个 crate

```
explorer/
├── Cargo.toml (workspace)
├── crates/
│   ├── explorer-app/       # 应用主入口
│   ├── explorer-common/    # 通用数据类型（本次实现）
│   ├── explorer-component/ # UI 组件库
│   ├── explorer-storage/   # 存储抽象层（本次实现）
│   └── providers/          # 具体存储提供者
│       ├── explorer-local-provider/ # 本地文件系统（本次实现）
│       ├── network/        # 网络存储（后续）
│       └── cloud/          # 云盘集成（后续）
```

**模块职责**：
- **explorer-app**：应用入口，负责集成各模块，实现数据转换
- **explorer-common**：共享数据类型，供 UI 组件和应用层使用
- **explorer-component**：纯 UI 组件，不依赖业务逻辑
- **explorer-storage**：存储抽象层，定义统一的存储接口
- **providers**：存储提供者的具体实现

**理由**：
- 清晰的职责分离，单一职责原则
- common 模块避免 UI 组件依赖业务逻辑
- 存储抽象层便于扩展不同存储后端
- providers 模块支持插件化扩展
- 便于代码复用和独立测试
- 符合 Rust 生态最佳实践
- 支持增量编译，提高开发效率

**考虑的替代方案**：
- 单一 crate：简单但不利于大型项目的模块化
- 多 repo：增加管理复杂度，不适合紧密耦合的组件

### 2. GPUI 应用架构

**决策**：采用 GPUI 的 Entity-View 模式，异步数据加载模式

```rust
// Explorer 主组件
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
        let provider = self.provider.clone();
        cx.spawn_in(window, async move |this, cx| {
            // 在后台线程执行数据加载
            let ret = cx.background_executor()
                .spawn(async move {
                    // 加载存储根节点和初始目录
                    // ...
                })
                .await;

            // 更新 UI
            let _ = cx.update(|_, cx| {
                let _ = this.update(cx, |explorer, cx| {
                    // 更新数据并通知重新渲染
                    cx.notify();
                });
            });
        }).detach();
    }
}

impl Render for Explorer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // 使用 GPUI 的布局 API 构建 UI
        let layout = SidebarLayout::new();
        let sidebar = Sidebar::new().roots(self.roots.clone());
        let file_list = FileList::new()
            .entries(self.entries.clone())
            .loading(self.loading)
            .error(self.error.clone());

        layout.render_with_children(sidebar, file_list)
    }
}
```

**关键模式**：
- **new + init 分离**：`new()` 创建实例，`init()` 启动异步加载
- **窗口先显示**：使用空数据立即显示窗口，避免阻塞
- **异步加载**：使用 `cx.spawn_in()` 和 `cx.background_executor()` 在后台加载数据
- **不阻塞 UI**：数据加载完成后通过 `cx.notify()` 触发重新渲染

**理由**：
- GPUI 的推荐模式
- 响应式状态管理
- 清晰的数据流向
- 非阻塞用户体验
- 便于调试和测试

### 3. 布局系统设计

**决策**：使用 Flexbox 布局实现左右分栏

- 左侧边栏：固定宽度或可调整（200-400px）
- 右侧主面板：占据剩余空间
- 使用 GPUI 的 `div()` 和 flex 布局 API

**组件层次**：
```
Explorer (app 层)
├── Sidebar (左侧 - component 层)
│   └── RootItem (common 层数据类型)
└── MainPanel (右侧 - component 层)
    └── FileList (使用 FileItem - common 层数据类型)
```

**数据流向**：
```
storage 层 (StorageRoot, StorageEntry)
    ↓ 转换函数 (app 层)
common 层 (RootItem, FileItem)
    ↓ props 传递
component 层 (UI 渲染)
```

**理由**：
- Flexbox 是现代 UI 布局的标准
- GPUI 原生支持
- 易于实现响应式调整

**考虑的替代方案**：
- Grid 布局：对于简单的两栏布局过于复杂
- 绝对定位：不够灵活，难以响应窗口大小变化

### 4. 存储抽象层设计

**决策**：引入存储抽象层（Storage Abstraction Layer），通过 trait 定义统一接口

```rust
// 伪代码
#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// 列出目录内容
    async fn list_entries(&self, path: &str) -> Result<Vec<StorageEntry>>;

    /// 获取条目元数据
    async fn get_metadata(&self, path: &str) -> Result<EntryMetadata>;

    /// 检查路径是否存在
    async fn exists(&self, path: &str) -> Result<bool>;

    /// 获取存储根节点（如磁盘列表、远程存储根等）
    async fn get_roots(&self) -> Result<Vec<StorageRoot>>;

    /// 获取提供者类型标识
    fn provider_type(&self) -> StorageProviderType;
}

// 本地文件系统实现
pub struct LocalFileSystemProvider {
    // 配置和缓存
}

#[async_trait]
impl StorageProvider for LocalFileSystemProvider {
    async fn list_entries(&self, path: &str) -> Result<Vec<StorageEntry>> {
        tokio::fs::read_dir(path)
            .await?
            .map(|entry| StorageEntry::from_local(entry))
            .collect()
    }
    // ... 其他方法实现
}

// 状态管理
struct FileListState {
    current_path: String,
    entries: Vec<StorageEntry>,
    loading: bool,
    provider: Arc<dyn StorageProvider>,
}
```

**理由**：
- **可扩展性**：支持不同存储后端（本地、网络、云盘）
- **统一接口**：上层 UI 代码无需关心具体存储类型
- **异步设计**：避免阻塞 UI 线程
- **类型安全**：通过 trait 保证接口一致性
- **便于测试**：可以轻松 mock StorageProvider

**考虑的替代方案**：
- 直接使用文件系统 API：不利于扩展，硬编码本地文件系统
- 枚举不同存储类型：增加复杂度，不如 trait 灵活
- 多线程同步方案：不如异步方案高效

### 5. 数据结构设计

**决策**：定义核心数据结构，支持多种存储后端

```rust
// 通用存储条目（适配不同存储类型）
pub struct StorageEntry {
    name: String,
    path: String,  // 使用字符串路径，支持 URL 等
    size: u64,
    modified: SystemTime,
    entry_type: EntryType,
    is_hidden: bool,
    metadata: EntryMetadata,
}

pub enum EntryType {
    File,
    Directory,
    Symlink,
}

pub struct EntryMetadata {
    permissions: Option<Permissions>,  // 本地文件系统
    mime_type: Option<String>,         // 网络存储可能提供
    custom_fields: HashMap<String, String>,  // 扩展字段
}

// 存储根节点（磁盘、网络挂载点等）
pub struct StorageRoot {
    id: String,
    name: String,
    root_path: String,
    provider_type: StorageProviderType,
    icon: Option<String>,
}

pub enum StorageProviderType {
    LocalFileSystem,
    NetworkDrive,
    CloudStorage { provider_name: String },
}

// 侧边栏状态
struct SidebarState {
    roots: Vec<StorageRoot>,  // 统一的根节点列表
    quick_access: Vec<QuickAccessItem>,
}
```

**理由**：
- **存储无关**：数据结构不依赖特定存储类型
- **可扩展**：通过 custom_fields 支持特定存储的额外信息
- **类型明确**：使用枚举区分不同条目和提供者类型
- **统一管理**：侧边栏可以同时显示本地磁盘和远程存储

### 6. 日志系统设计

**决策**：使用 tracing 系列库实现结构化日志

```rust
// 日志初始化配置
fn init_logging() {
    let log_dir = home_dir()
        .map(|home| home.join(".explorer").join("logs"))
        .expect("Failed to find log dir");

    let log_rolling = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("explorer")
        .filename_suffix("log")
        .build(&log_dir)
        .expect("Failed to create log file appender");

    let (non_blocking, _guard) = non_blocking(log_rolling);

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(log_level)))
        .with(layer().with_writer(stdout))
        .with(layer().with_writer(non_blocking).with_ansi(false))
        .init();
}
```

**日志配置**：
- **日志目录**：`~/.explorer/logs/`
- **文件命名**：`explorer.YYYY-MM-DD.log`
- **滚动策略**：按天滚动
- **日志级别**：
  - Debug 模式：`debug`
  - Release 模式：`info`
- **输出目标**：
  - 标准输出（带 ANSI 颜色）
  - 日志文件（不带颜色）

**理由**：
- **结构化日志**：tracing 提供更丰富的上下文信息
- **性能优良**：异步日志写入不阻塞主线程
- **灵活配置**：支持环境变量 `RUST_LOG` 动态调整级别
- **文件管理**：自动按天滚动，避免单文件过大
- **生态完善**：与 Rust 异步生态深度集成

**考虑的替代方案**：
- log + env_logger：功能相对简单，不支持结构化日志
- 自定义日志系统：开发成本高，不如成熟方案

### 7. 代码组织规范

**决策**：统一的导入语句格式和代码组织规范

#### 导入语句顺序

所有 Rust 文件的导入语句必须按以下顺序组织，各组之间用空行分隔：

1. **标准库** (`std::*`)：使用大括号分组
2. **外部库**：按字母顺序排列
3. **项目模块**：按依赖关系排列

**示例**：
```rust
use std::{
    fs::create_dir_all,
    io::stdout,
    sync::Arc,
};

use dirs::home_dir;
use gpui::{prelude::*,*};
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};

use explorer_common::*;
use explorer_component::*;
use explorer_storage::*;
```

**理由**：
- **一致性**：整个项目使用统一的格式
- **可读性**：清晰的分组便于理解依赖关系
- **维护性**：新增依赖时容易找到正确位置
- **标准化**：符合 Rust 社区最佳实践

#### 异步方法规范

- **避免公开 async 方法**：组件的公开 API 应该是同步的
- **内部使用异步**：使用 `cx.spawn_in()` 和 `cx.background_executor()` 进行异步操作
- **非阻塞原则**：所有 I/O 操作必须在后台线程执行

**示例**：
```rust
impl Explorer {
    /// 公开方法是同步的
    pub fn load_directory(&mut self, path: String, window: &Window, cx: &mut Context<Self>) {
        // 内部使用异步处理
        let provider = self.provider.clone();
        cx.spawn_in(window, async move |this, cx| {
            let ret = cx.background_executor()
                .spawn(async move {
                    provider.list_entries(&path).await
                })
                .await;

            // 更新 UI
            let _ = cx.update(|_, cx| {
                let _ = this.update(cx, |explorer, cx| {
                    // 更新数据并通知重新渲染
                    cx.notify();
                });
            });
        }).detach();
    }
}
```

**理由**：
- **简化 API**：调用方不需要处理 async/await
- **灵活性**：可以在同步上下文中调用
- **GPUI 兼容**：符合 GPUI 的事件模型
- **用户体验**：确保 UI 始终响应

## 技术选型

### 依赖项

| 依赖 | 版本 | 用途 |
|------|------|------|
| gpui | latest | UI 框架 |
| tokio | 1.x | 异步运行时 |
| smol | 2.x | 与 GPUI 兼容的异步运行时 |
| async-trait | 0.1 | trait 异步方法支持 |
| anyhow | 1.x | 错误处理 |
| serde | 1.x | 序列化（配置文件） |
| tracing | 0.1 | 结构化日志框架 |
| tracing-subscriber | 0.3 | 日志订阅器和格式化 |
| tracing-appender | 0.2 | 日志文件滚动和管理 |
| dirs | 5.x | 跨平台目录路径获取 |

### 平台兼容性考虑

- **路径处理**：使用字符串路径而非 `PathBuf`，支持本地路径和 URL
- **存储抽象**：通过 `StorageProvider` trait 隔离平台差异
- **本地文件系统**：
  - macOS/Linux：读取 `/Volumes` 或 `/mnt` 获取挂载点
  - Windows：使用 WinAPI 获取驱动器列表（后续实现）
- **异步运行时**：使用 tokio（跨平台）

## 风险 / 权衡

### 风险 1：GPUI 学习曲线
- **风险**：GPUI 是相对较新的框架，文档可能不够完善
- **缓解措施**：
  - 从简单组件开始，逐步学习
  - 参考 GPUI 官方示例和 Zed 编辑器源码
  - 保持组件简单，避免过早优化

### 风险 2：性能问题
- **风险**：大目录可能导致性能问题
- **缓解措施**：
  - 初期限制显示数量（如前 1000 项）
  - 后续实现虚拟滚动
  - 使用异步加载，避免阻塞

### 权衡 1：完整性 vs 简洁性
- **选择**：初期实现最小可行布局，不实现所有功能
- **理由**：快速验证架构可行性，避免过度设计

## 实施计划

### 阶段 1：项目搭建（优先级：最高）
1. 创建 Cargo workspace 配置
2. 创建 app、comps、storage 和 providers/local crate
3. 配置依赖项（包括 async-trait）
4. 创建简单的 GPUI 应用入口

### 阶段 2：存储抽象层（优先级：最高）
1. 定义 `StorageProvider` trait
2. 定义核心数据结构（StorageEntry、StorageRoot 等）
3. 实现 `LocalFileSystemProvider`
4. 编写单元测试验证接口

### 阶段 3：布局实现（优先级：高）
1. 实现主布局组件（SidebarLayout）
2. 实现侧边栏组件骨架
3. 实现主面板组件骨架
4. 验证布局响应性

### 阶段 4：存储集成（优先级：高）
1. 在应用状态中集成 StorageProvider
2. 实现目录内容加载逻辑
3. 实现状态管理
4. 在主面板中显示文件列表（简单列表形式）

### 阶段 5：侧边栏完善（优先级：中）
1. 使用 StorageProvider 获取根节点列表
2. 实现常用目录列表
3. 实现侧边栏项目点击导航

## 待解决问题

1. **存储提供者注册机制**：
   - 如何动态注册多个存储提供者？（建议：使用 Arc<Vec<dyn StorageProvider>>）
   - 如何在 UI 中切换不同的存储提供者？

2. **路径表示**：
   - 统一使用字符串路径还是自定义类型？（建议：字符串，支持 URL）
   - 如何处理本地路径和远程 URL 的区分？（建议：通过 provider_type）

3. **GPUI 组件样式**：
   - 如何定义统一的样式系统？（建议：在 comps 中定义 theme 模块）

4. **错误处理**：
   - 如何向用户展示错误（如权限不足）？（建议：简单的错误提示，后续增强）
   - 不同存储提供者的错误如何统一？（建议：定义通用错误类型）

5. **配置文件**：
   - 是否在此阶段实现配置文件？（建议：暂不实现，使用硬编码默认值）

## 验收标准

本变更完成后应满足：

1. ✅ 项目可以成功编译和运行
2. ✅ 显示左右分栏布局
3. ✅ 定义并实现 StorageProvider trait
4. ✅ 实现 LocalFileSystemProvider
5. ✅ 左侧边栏显示存储根节点（至少本地磁盘，macOS）
6. ✅ 左侧边栏显示常用目录
7. ✅ 右侧主面板能够通过 StorageProvider 显示文件列表
8. ✅ 点击侧边栏项目可以切换目录
9. ✅ 文件列表显示文件名和基本信息
10. ✅ UI 不会阻塞在文件读取时
11. ✅ storage 和 providers/local crate 正常工作

## 参考资料

- [GPUI 官方文档](https://www.gpui.rs/)
- [Zed 编辑器源码](https://github.com/zed-industries/zed)（GPUI 实际应用）
- [Tokio 异步编程指南](https://tokio.rs/)
