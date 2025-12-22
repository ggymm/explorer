## ADDED Requirements

### Requirement: 存储提供者接口
系统 MUST 定义统一的存储提供者接口，支持不同类型的存储后端。

#### Scenario: StorageProvider trait 定义
- **WHEN** 定义存储提供者接口
- **THEN** 接口包含列出目录内容的方法（list_entries）
- **AND** 接口包含获取元数据的方法（get_metadata）
- **AND** 接口包含检查路径存在性的方法（exists）
- **AND** 接口包含获取存储根节点的方法（get_roots）
- **AND** 接口包含获取提供者类型的方法（provider_type）
- **AND** 所有 IO 方法都是异步的

#### Scenario: 接口线程安全
- **WHEN** 使用 StorageProvider
- **THEN** trait 要求实现 Send + Sync
- **AND** 可以在多线程环境中安全使用
- **AND** 支持 Arc 包装

### Requirement: 本地文件系统提供者
系统 MUST 实现本地文件系统的存储提供者。

#### Scenario: LocalFileSystemProvider 实现
- **WHEN** 实现本地文件系统提供者
- **THEN** 实现 StorageProvider trait 的所有方法
- **AND** 使用 tokio::fs 进行异步文件操作
- **AND** 正确处理本地文件路径

#### Scenario: 列出本地目录内容
- **WHEN** 调用 list_entries 方法
- **THEN** 返回目录中的所有文件和子目录
- **AND** 每个条目包含名称、路径、大小、修改时间等信息
- **AND** 正确区分文件和目录
- **AND** 支持隐藏文件的识别

#### Scenario: 获取本地存储根节点
- **WHEN** 调用 get_roots 方法
- **THEN** macOS 返回 /Volumes 下的所有挂载点
- **AND** Linux 返回根目录和挂载点
- **AND** 每个根节点包含名称、路径和类型信息

### Requirement: 通用存储条目数据结构
系统 MUST 定义存储无关的条目数据结构。

#### Scenario: StorageEntry 结构定义
- **WHEN** 定义 StorageEntry 结构
- **THEN** 包含名称字段（String）
- **AND** 包含路径字段（String，支持 URL）
- **AND** 包含大小字段（u64）
- **AND** 包含修改时间字段（SystemTime）
- **AND** 包含条目类型枚举（File/Directory/Symlink）
- **AND** 包含隐藏标志（bool）
- **AND** 包含可扩展的元数据字段

#### Scenario: EntryMetadata 可扩展性
- **WHEN** 定义 EntryMetadata 结构
- **THEN** 权限信息使用 Option 包装（可选）
- **AND** MIME 类型使用 Option 包装（可选）
- **AND** 包含 custom_fields 哈希表用于扩展
- **AND** 不同存储提供者可以添加自定义字段

### Requirement: 存储根节点数据结构
系统 MUST 定义存储根节点的数据结构。

#### Scenario: StorageRoot 结构定义
- **WHEN** 定义 StorageRoot 结构
- **THEN** 包含唯一标识符（id）
- **AND** 包含显示名称（name）
- **AND** 包含根路径（root_path）
- **AND** 包含提供者类型（provider_type）
- **AND** 可选包含图标信息

#### Scenario: 提供者类型枚举
- **WHEN** 定义 StorageProviderType 枚举
- **THEN** 包含 LocalFileSystem 变体
- **AND** 包含 NetworkDrive 变体（预留）
- **AND** 包含 CloudStorage 变体（预留，带提供者名称）
- **AND** 支持模式匹配和类型判断

### Requirement: 存储抽象层模块组织
系统 MUST 建立清晰的存储抽象层模块结构。

#### Scenario: Crate 结构
- **WHEN** 查看项目结构
- **THEN** 存在 storage crate 定义核心 trait 和数据结构
- **AND** 存在 providers/local crate 实现本地文件系统
- **AND** storage crate 不依赖具体实现
- **AND** providers/local 依赖 storage crate

#### Scenario: 模块导出
- **WHEN** 其他 crate 使用存储功能
- **THEN** 可以从 storage crate 导入 trait 和数据结构
- **AND** 可以从 providers crate 导入具体实现
- **AND** 接口清晰易用

### Requirement: 错误处理
系统 MUST 定义存储操作的错误类型。

#### Scenario: 统一错误类型
- **WHEN** 存储操作失败
- **THEN** 返回统一的错误类型
- **AND** 错误包含清晰的描述信息
- **AND** 错误可以区分不同失败原因（权限、不存在、网络等）
- **AND** 错误可以转换为用户友好的提示

### Requirement: 提供者集成
系统 MUST 支持在应用中集成和使用存储提供者。

#### Scenario: 提供者注册
- **WHEN** 应用启动
- **THEN** 创建本地文件系统提供者实例
- **AND** 将提供者存储在应用状态中
- **AND** 使用 Arc 包装以支持共享

#### Scenario: 切换存储后端
- **WHEN** UI 需要访问不同存储
- **THEN** 可以使用不同的 StorageProvider 实例
- **AND** 上层代码无需修改
- **AND** 通过统一接口操作

### Requirement: 异步操作支持
系统 MUST 确保所有存储操作都是异步的。

#### Scenario: 异步方法实现
- **WHEN** 实现 StorageProvider trait
- **THEN** 所有 IO 方法使用 async 标记
- **AND** 使用 async-trait crate 支持 trait 异步方法
- **AND** 方法返回 Future

#### Scenario: 非阻塞 UI
- **WHEN** 执行存储操作
- **THEN** UI 线程不被阻塞
- **AND** 用户可以继续交互
- **AND** 显示加载状态指示
