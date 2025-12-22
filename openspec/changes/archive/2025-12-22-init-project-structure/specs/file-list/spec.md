## ADDED Requirements

### Requirement: 目录读取功能
系统 MUST 通过存储提供者接口读取目录内容。

#### Scenario: 读取目录成功
- **WHEN** 通过 StorageProvider 请求读取有效目录路径
- **THEN** 返回目录中的所有文件和子目录作为 StorageEntry
- **AND** 包含每个条目的基本信息（名称、类型、大小等）
- **AND** 操作不阻塞 UI 线程

#### Scenario: 读取目录失败处理
- **WHEN** 请求读取无效或无权限的目录
- **THEN** 返回错误信息
- **AND** 错误信息描述清晰（如"权限不足"）
- **AND** 不导致应用崩溃

#### Scenario: 异步读取
- **WHEN** 读取大目录
- **THEN** 使用异步 IO 操作
- **AND** UI 保持响应
- **AND** 显示加载状态指示

### Requirement: 文件信息数据结构
系统 MUST 使用存储抽象层定义的数据结构表示文件信息。

#### Scenario: 使用 StorageEntry
- **WHEN** 表示文件或目录条目
- **THEN** 使用 storage crate 中的 StorageEntry 结构
- **AND** 包含文件名（String）
- **AND** 包含路径（String，支持 URL）
- **AND** 包含大小（u64）
- **AND** 包含修改时间（SystemTime）
- **AND** 包含条目类型枚举（EntryType）
- **AND** 包含隐藏标志（bool）
- **AND** 包含可扩展元数据（EntryMetadata）

#### Scenario: 跨存储类型兼容
- **WHEN** 使用 StorageEntry 表示不同存储的条目
- **THEN** 本地文件系统、网络存储、云盘都使用同一结构
- **AND** 通过 EntryMetadata 的 custom_fields 支持特定存储的额外信息
- **AND** 路径字段支持本地路径和 URL

### Requirement: 文件列表状态管理
系统 MUST 管理文件列表的状态并集成存储提供者。

#### Scenario: 状态结构定义
- **WHEN** 定义文件列表状态
- **THEN** 包含当前路径（String）
- **AND** 包含文件条目列表（Vec<StorageEntry>）
- **AND** 包含加载状态标志（bool）
- **AND** 包含错误信息（Option<String>）
- **AND** 包含存储提供者引用（Arc<dyn StorageProvider>）

#### Scenario: 状态更新
- **WHEN** 切换目录
- **THEN** 更新当前路径
- **AND** 清空旧的文件列表
- **AND** 设置加载状态为 true
- **AND** 通过 StorageProvider.list_entries 异步加载新目录内容
- **AND** 加载完成后更新文件列表和状态

### Requirement: 文件列表渲染
系统 MUST 在主面板中渲染文件列表。

#### Scenario: 列表基本显示
- **WHEN** 文件列表数据就绪
- **THEN** 在主面板显示文件列表
- **AND** 每个文件/目录显示为一行
- **AND** 显示文件名
- **AND** 目录和文件有视觉区分（如图标或样式）

#### Scenario: 空目录显示
- **WHEN** 当前目录为空
- **THEN** 显示"目录为空"提示
- **AND** 不显示错误信息

#### Scenario: 加载状态显示
- **WHEN** 正在加载目录内容
- **THEN** 显示加载指示器
- **AND** 用户可以看到正在加载的状态

#### Scenario: 错误状态显示
- **WHEN** 加载目录失败
- **THEN** 显示错误信息
- **AND** 错误信息易于理解
- **AND** 提供可能的解决建议（如果适用）

### Requirement: 文件列表交互准备
系统 MUST 为文件列表交互做准备（基础实现）。

#### Scenario: 项目可点击
- **WHEN** 用户点击目录项
- **THEN** 进入该子目录
- **AND** 文件列表更新为子目录内容
- **AND** 当前路径更新

#### Scenario: 文件点击（预留）
- **WHEN** 用户点击文件项
- **THEN** 暂不执行操作（预留给后续功能）
- **AND** 可选：高亮选中的文件

### Requirement: 初始目录设置
系统 MUST 在启动时通过存储提供者加载合理的初始目录。

#### Scenario: 默认初始目录
- **WHEN** 应用首次启动
- **THEN** 使用 StorageProvider.get_roots 获取根节点
- **AND** 默认选择第一个根节点或用户主目录
- **AND** 如果获取失败，显示错误提示
