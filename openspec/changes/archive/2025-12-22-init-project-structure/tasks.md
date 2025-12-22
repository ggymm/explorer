# 实施任务清单

## 1. 项目结构搭建

### 1.1 创建 Cargo Workspace
- [ ] 在项目根目录创建 `Cargo.toml` 文件
- [ ] 配置 `[workspace]` 部分，定义成员 crate
- [ ] 设置 workspace 级别的依赖项（如 GPUI、tokio）
- [ ] 配置编译选项和特性

### 1.2 创建 explorer-app Crate
- [x] 创建 `crates/explorer-app` 目录
- [x] 初始化 `crates/explorer-app/Cargo.toml`
- [x] 创建 `crates/explorer-app/src/main.rs`
- [x] 添加对 common、component、storage 和 local-provider crate 的依赖

### 1.3 创建 explorer-common Crate
- [x] 创建 `crates/explorer-common` 目录
- [x] 初始化 `crates/explorer-common/Cargo.toml`
- [x] 创建 `crates/explorer-common/src/lib.rs`
- [x] 定义共享数据类型（RootItem, FileItem, ProviderType, ItemType）

### 1.4 创建 explorer-component Crate
- [x] 创建 `crates/explorer-component` 目录
- [x] 初始化 `crates/explorer-component/Cargo.toml`
- [x] 创建 `crates/explorer-component/src/lib.rs`
- [x] 定义模块结构（layout、sidebar、file_list 等）
- [x] 依赖 explorer-common 而非 explorer-storage

### 1.5 创建 explorer-storage Crate
- [x] 创建 `crates/explorer-storage` 目录
- [x] 初始化 `crates/explorer-storage/Cargo.toml`
- [x] 创建 `crates/explorer-storage/src/lib.rs`
- [x] 添加 async-trait 依赖

### 1.6 创建 providers/explorer-local-provider Crate
- [x] 创建 `crates/providers/explorer-local-provider` 目录
- [x] 初始化 `crates/providers/explorer-local-provider/Cargo.toml`
- [x] 创建 `crates/providers/explorer-local-provider/src/lib.rs`
- [x] 添加对 explorer-storage crate 的依赖

### 1.7 配置模块依赖
- [x] explorer-app 依赖 common、component、storage 和 local-provider
- [x] explorer-component 依赖 common 和 gpui
- [x] explorer-local-provider 依赖 storage
- [x] 确保依赖关系无循环
- [x] 实现 app 层的数据转换函数（storage → common 类型）

### 1.8 验证项目编译
- [x] 运行 `cargo build` 确保 workspace 配置正确
- [x] 运行 `cargo check` 验证所有依赖项正确解析
- [x] 验证依赖关系：component 仅依赖 common 和 gpui
- [x] 修复任何编译错误或警告

---

## 2. GPUI 应用初始化

### 2.1 实现应用入口
- [x] 在 `app/main.rs` 中初始化 GPUI 应用
- [x] 初始化日志系统（tracing + tracing-appender）
- [x] 配置日志输出到 stdout 和文件
- [x] 配置日志文件按天滚动
- [x] 创建主窗口
- [x] 设置窗口标题为 "Explorer"
- [x] 设置默认窗口大小（1280x800）

### 2.2 实现主组件
- [x] 创建 `Explorer` 结构体
- [x] 实现 `new()` 方法 - 快速创建实例
- [x] 实现 `init()` 方法 - 启动异步数据加载
- [x] 使用 `cx.spawn_in()` 和 `cx.background_executor()` 进行后台加载
- [x] 实现 GPUI 的 `Render` trait
- [x] 添加 loading 和 error 状态管理
- [x] 确保 UI 不阻塞在数据加载时

### 2.3 测试应用启动
- [ ] 运行 `cargo run` 确保应用窗口打开
- [ ] 验证窗口标题和大小正确
- [ ] 测试窗口关闭功能

---

## 3. 存储抽象层实现

### 3.1 定义存储提供者接口
- [ ] 在 `storage/src/provider.rs` 中定义 `StorageProvider` trait
- [ ] 定义 `list_entries` 异步方法
- [ ] 定义 `get_metadata` 异步方法
- [ ] 定义 `exists` 异步方法
- [ ] 定义 `get_roots` 异步方法
- [ ] 定义 `provider_type` 同步方法
- [ ] 使用 async_trait 宏

### 3.2 定义核心数据结构
- [x] 在 `storage/src/types.rs` 中定义 `StorageEntry` 结构体
- [x] 定义 `EntryType` 枚举（File/Directory/Symlink）
- [x] 定义 `EntryMetadata` 结构体（包含 custom_fields）
- [x] 添加 `created` 和 `accessed` 时间字段（Option 类型）
- [x] 添加 `mime_type` 字段（Option<String>）
- [x] 实现 SystemTime 序列化/反序列化
- [x] 定义 `StorageRoot` 结构体
- [x] 定义 `StorageProviderType` 枚举
- [x] 为数据结构实现 Clone、Debug、Serialize、Deserialize 等 trait

### 3.3 实现本地文件系统提供者
- [x] 在 `providers/local/src/lib.rs` 中定义 `LocalFileSystemProvider` 结构体
- [x] 实现 `StorageProvider` trait
- [x] 实现 `list_entries` 方法（使用 smol::unblock）
- [x] 实现 `get_metadata` 方法（包含创建时间、访问时间）
- [x] 实现 `exists` 方法
- [x] 实现 `get_roots` 方法（macOS：读取 /Volumes，添加根目录）
- [x] 实现 `provider_type` 方法返回 LocalFileSystem
- [x] 使用 `smol` 作为异步运行时（与 GPUI 兼容）

### 3.4 实现辅助函数
- [x] 实现从 DirEntry 转换为 StorageEntry 的方法
- [x] 实现隐藏文件检测（Unix：以 . 开头，Windows：文件属性）
- [x] 实现 MIME 类型推断（使用 mime_guess）
- [x] 实现错误处理和转换
- [x] 处理权限、符号链接等特殊情况
- [x] 收集 created 和 accessed 时间（跨平台兼容）

### 3.5 编写单元测试
- [ ] 测试 list_entries 方法
- [ ] 测试 get_roots 方法
- [ ] 测试错误处理
- [ ] 测试跨平台兼容性

---

## 4. 布局系统实现

### 4.1 创建布局组件
- [ ] 在 `comps/src/layout.rs` 中创建 `SidebarLayout` 组件
- [ ] 使用 GPUI 的 div 和 flex API 实现左右分栏
- [ ] 配置左侧栏宽度（默认 250px）
- [ ] 配置右侧栏自动填充剩余空间

### 4.2 创建侧边栏组件
- [ ] 创建 `comps/src/sidebar.rs`
- [ ] 定义 `Sidebar` 结构体
- [ ] 实现侧边栏的基本渲染
- [ ] 添加存储根节点列表区域
- [ ] 添加常用目录区域

### 4.3 创建主面板组件
- [ ] 创建 `comps/src/main_panel.rs`
- [ ] 定义 `MainPanel` 结构体
- [ ] 实现主面板的基本渲染
- [ ] 添加顶部路径显示区域
- [ ] 添加内容区域容器

### 4.4 集成布局到主组件
- [ ] 在 `Explorer` 中使用 `SidebarLayout`
- [ ] 将 `Sidebar` 放置在左栏
- [ ] 将 `MainPanel` 放置在右栏
- [ ] 测试布局显示效果

---

## 5. 状态管理与集成

### 5.1 定义应用状态
- [ ] 定义 `SidebarState` 结构体（存储根节点、常用目录）
- [ ] 定义 `FileListState` 结构体（当前路径、条目列表、加载状态、StorageProvider 引用）
- [ ] 定义 `AppState` 结构体（包含 sidebar 和 file_list 状态）

### 5.2 集成存储提供者到应用状态
- [ ] 在应用启动时创建 LocalFileSystemProvider 实例
- [ ] 使用 Arc 包装提供者以支持共享
- [ ] 将提供者存储在 AppState 中
- [ ] 确保线程安全
- [ ] 实现 `change_directory(path: PathBuf)` 方法
- [ ] 更新 `FileListState` 的当前路径
- [ ] 设置加载状态
- [ ] 异步加载新目录内容
- [ ] 更新文件列表

### 6.3 实现错误处理
- [ ] 定义错误类型（如 `FsError`）
- [ ] 在状态中存储错误信息
- [ ] 在 UI 中显示错误信息

---

## 7. UI 渲染实现

### 7.1 实现侧边栏渲染
- [ ] 渲染磁盘列表
  - [ ] 显示磁盘名称
  - [ ] 显示挂载点路径
  - [ ] 添加点击事件处理
- [ ] 渲染常用目录列表
  - [ ] 显示目录名称
  - [ ] 添加图标（可选，简单文本也可）
  - [ ] 添加点击事件处理

### 6.2 实现主面板渲染
- [ ] 渲染当前路径显示
- [ ] 渲染文件列表
  - [ ] 显示文件/目录名称
  - [ ] 根据 EntryType 区分文件和目录（样式或图标）
  - [ ] 添加点击事件（目录可导航）
- [ ] 实现加载状态显示（简单的"加载中..."文本）
- [ ] 实现错误状态显示
- [ ] 实现空目录提示

---

## 7. 事件处理

### 7.1 侧边栏点击事件
- [ ] 为存储根节点添加点击回调
- [ ] 点击时调用 `change_directory` 切换到根路径
- [ ] 为常用目录项添加点击回调
- [ ] 点击时调用 `change_directory` 切换到对应目录

### 7.2 文件列表点击事件
- [ ] 为目录项添加双击或点击回调
- [ ] 点击目录时调用 `change_directory` 进入子目录
- [ ] 文件点击暂不处理（或简单高亮）

---

## 8. 测试与验证

### 8.1 存储抽象层测试
- [ ] 测试 StorageProvider trait 实现
- [ ] 测试 LocalFileSystemProvider 各方法
- [ ] 测试异步操作
- [ ] 测试错误处理

### 8.2 功能测试
- [ ] 测试应用启动和窗口显示
- [ ] 测试侧边栏存储根节点显示
- [ ] 测试侧边栏常用目录显示
- [ ] 测试主面板文件列表显示
- [ ] 测试点击侧边栏项切换目录
- [ ] 测试点击文件列表中的目录进入子目录
- [ ] 测试空目录显示
- [ ] 测试加载状态显示

### 8.3 错误处理测试
- [ ] 测试无权限目录的错误显示
- [ ] 测试不存在路径的错误处理
- [ ] 测试应用在错误情况下不崩溃

### 8.4 跨平台测试（如果适用）
- [ ] macOS 平台测试
- [ ] Linux 平台测试（如果已配置）
- [ ] Windows 平台测试（如果已配置）

---

## 9. 代码优化与文档

### 9.1 代码审查
- [ ] 运行 `cargo clippy` 修复所有警告
- [ ] 运行 `cargo fmt` 格式化代码
- [ ] 审查代码质量和可读性
- [ ] 确保模块边界清晰

### 9.2 添加文档
- [ ] 为 StorageProvider trait 添加文档注释
- [ ] 为公共数据结构添加文档注释
- [ ] 为 UI 组件添加使用说明
- [ ] 在 README 中添加构建和运行说明
- [ ] 记录已知限制和待实现功能

### 9.3 性能检查
- [ ] 测试大目录加载性能
- [ ] 确认 UI 不阻塞
- [ ] 验证内存使用合理
- [ ] 检查异步操作效率

---

## 依赖关系说明

- **阶段 1-2** 必须顺序完成（项目搭建 → 应用初始化）
- **阶段 3** 存储抽象层实现，可以与阶段 4 部分并行
- **阶段 4** 布局实现，依赖阶段 2
- **阶段 5** 状态管理，依赖阶段 3（需要 StorageProvider）
- **阶段 6-7** UI 渲染和事件处理，依赖阶段 4-5
- **阶段 8-9** 在所有功能实现后进行

## 预估工作量

- **核心功能实现**：阶段 1-7（主要工作）
- **验证和优化**：阶段 8-9（辅助工作）
- **建议迭代方式**：先完成最小可行版本（MVP），再逐步完善
- **关键路径**：存储抽象层 → 状态管理 → UI 集成
