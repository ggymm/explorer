# Explorer 架构说明

## 项目结构

```
explorer/
├── Cargo.toml              # Workspace 配置
├── crates/
│   ├── explorer-app/       # 应用层
│   ├── explorer-common/    # 共享层
│   ├── explorer-component/ # UI 组件层
│   ├── explorer-storage/   # 业务逻辑层
│   └── providers/          # 实现层
│       └── explorer-local-provider/
```

## 分层架构

### 1. 应用层 (explorer-app)
**职责**：
- 应用程序入口
- 集成各个模块
- 数据类型转换（storage 层 ↔ common 层）
- 日志系统初始化

**依赖**：
- `explorer-common` - 共享数据类型
- `explorer-component` - UI 组件
- `explorer-storage` - 存储抽象
- `explorer-local-provider` - 本地文件系统实现

### 2. 共享层 (explorer-common)
**职责**：
- 定义通用数据类型
- 供 UI 组件和应用层共享使用
- 与业务逻辑解耦

**数据类型**：
- `RootItem` - 存储根节点信息
- `ProviderType` - 存储提供者类型
- `FileItem` - 文件/目录条目信息
- `ItemType` - 条目类型枚举

**依赖**：
- `serde` - 序列化支持

### 3. UI 组件层 (explorer-component)
**职责**：
- 纯 UI 组件实现
- 不依赖业务逻辑
- 使用 common 层的数据类型

**组件**：
- `SidebarLayout` - 左右分栏布局
- `Sidebar` - 侧边栏组件
- `FileList` - 文件列表组件

**依赖**：
- `explorer-common` - 共享数据类型
- `gpui` - UI 框架

### 4. 业务逻辑层 (explorer-storage)
**职责**：
- 定义存储抽象接口
- 定义业务数据类型
- 错误类型定义

**核心接口**：
- `StorageProvider` trait - 存储提供者统一接口
  - `get_roots()` - 获取存储根节点
  - `list_entries()` - 列出目录内容
  - `get_metadata()` - 获取文件元数据
  - `exists()` - 检查路径是否存在

**数据类型**：
- `StorageRoot` - 存储根节点（包含详细元数据）
- `StorageEntry` - 文件/目录条目（包含详细元数据）
- `EntryType` - 条目类型枚举
- `EntryMetadata` - 条目元数据

**依赖**：
- `async-trait` - 异步 trait 支持
- `serde` - 序列化支持

### 5. 实现层 (providers)
**职责**：
- 实现具体的存储提供者
- 处理平台特定逻辑

**当前实现**：
- `explorer-local-provider` - 本地文件系统
  - 跨平台支持（macOS/Linux/Windows）
  - 自动检测磁盘挂载点
  - 隐藏文件处理
  - MIME 类型推断

**依赖**：
- `explorer-storage` - 存储抽象
- `smol` - 异步运行时（与 GPUI 兼容）
- `mime_guess` - MIME 类型推断

## 数据流向

```
┌─────────────────────────────────────────────────┐
│ explorer-storage (业务数据类型)                  │
│ StorageRoot, StorageEntry, EntryType           │
└────────────────┬────────────────────────────────┘
                 │
                 ↓ 转换函数 (在 explorer-app 中)
┌────────────────┴────────────────────────────────┐
│ explorer-common (UI 数据类型)                    │
│ RootItem, FileItem, ItemType, ProviderType     │
└────────────────┬────────────────────────────────┘
                 │
                 ↓ props 传递
┌────────────────┴────────────────────────────────┐
│ explorer-component (UI 渲染)                    │
│ Sidebar, FileList, SidebarLayout               │
└─────────────────────────────────────────────────┘
```

## 设计原则

### 1. 单一职责原则
每个模块有明确的职责边界：
- **common** - 仅定义数据类型
- **component** - 仅负责 UI 渲染
- **storage** - 仅定义抽象接口
- **providers** - 仅实现具体逻辑
- **app** - 负责集成和转换

### 2. 依赖倒置原则
- UI 组件不依赖业务逻辑，只依赖共享类型
- 业务逻辑通过 trait 定义接口，具体实现通过依赖注入

### 3. 开闭原则
- 新增存储提供者：实现 `StorageProvider` trait
- 新增 UI 组件：使用 common 层类型
- 无需修改现有代码

### 4. 接口隔离原则
- common 层提供最小化的数据类型
- component 层只接收渲染所需的数据
- storage 层接口精简，易于实现

## 扩展指南

### 添加新的存储提供者

1. 在 `crates/providers/` 下创建新 crate
2. 实现 `StorageProvider` trait
3. 在 `explorer-app` 中注册

```rust
// 示例：网络驱动提供者
pub struct NetworkDriveProvider {
    // ...
}

#[async_trait]
impl StorageProvider for NetworkDriveProvider {
    async fn get_roots(&self) -> StorageResult<Vec<StorageRoot>> {
        // 实现网络驱动的根节点获取
    }
    
    // ... 实现其他方法
}
```

### 添加新的 UI 组件

1. 在 `explorer-component/src/` 中创建新文件
2. 使用 `explorer-common` 的数据类型
3. 实现 GPUI 的 `Render` trait

```rust
use gpui::{prelude::*,*};
use explorer_common::FileItem;

pub struct FileGrid {
    items: Vec<FileItem>,
}

impl Render for FileGrid {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 实现网格布局渲染
    }
}
```

### 添加新的共享类型

1. 在 `explorer-common/src/types.rs` 中定义
2. 确保实现必要的 trait（Clone, Debug, Serialize, Deserialize）
3. 在 `lib.rs` 中导出

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub matches: Vec<Match>,
}
```

## 最佳实践

### 1. 导入语句规范
按以下顺序组织，各组之间用空行分隔：
1. 标准库 (`std::*`) - 使用大括号分组
2. 外部库 - 按字母顺序
3. 项目模块 - 按依赖关系

### 2. 异步编程规范
- 公开方法避免 async
- 使用 `cx.spawn_in()` 和 `cx.background_executor()` 进行异步操作
- 所有 I/O 操作必须在后台线程执行

### 3. 组件初始化模式
- `new()` - 快速创建实例
- `init()` - 启动异步初始化
- 窗口立即显示，数据后台加载

## 依赖关系图

```
┌─────────────────────────┐
│    explorer-app         │
│                         │
└──┬──────────┬───────┬───┘
   │          │       │
   ↓          ↓       ↓
┌─────┐   ┌──────┐  ┌────────────┐
│common│   │component│  │storage     │
└─────┘   └──┬───┘  └─────┬──────┘
             │             │
             ↓             ↓
          ┌─────┐    ┌──────────────┐
          │gpui │    │local-provider│
          └─────┘    └──────────────┘
```

## 性能考虑

1. **增量编译**：模块化设计支持增量编译，提高开发效率
2. **并行构建**：独立模块可以并行编译
3. **按需加载**：UI 组件和存储提供者可以按需加载
4. **异步 I/O**：所有文件操作使用异步，避免阻塞 UI

## 测试策略

1. **单元测试**：每个模块独立测试
2. **集成测试**：测试模块间交互
3. **Mock 友好**：使用 trait 便于 mock 存储提供者
4. **UI 测试**：组件使用 common 类型，易于构造测试数据
