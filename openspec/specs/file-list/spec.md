# file-list Specification

## Purpose
TBD - created by archiving change init-project-structure. Update Purpose after archive.
## Requirements
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

### Requirement: 文件项选中状态支持
系统 MUST 支持文件和文件夹的选中状态管理。

#### Scenario: 选中状态数据结构
- **WHEN** 查看 Explorer 结构体定义
- **THEN** 存在 selected_items 字段，类型为 HashSet<String>
- **AND** 存在 last_selected_index 字段，类型为 Option<usize>
- **AND** 提供 add_selection、remove_selection、clear_selection 方法

#### Scenario: 单选行为
- **WHEN** 用户点击未选中的文件项（无修饰键）
- **THEN** 清除所有其他选中项
- **AND** 选中当前点击的项
- **AND** 更新 last_selected_index 为当前索引
- **AND** 触发界面重新渲染

#### Scenario: Ctrl/Cmd 多选
- **WHEN** 用户按住 Ctrl（Windows/Linux）或 Cmd（macOS）点击文件项
- **THEN** 不清除其他选中项
- **AND** 切换当前项的选中状态（选中变未选中，未选中变选中）
- **AND** 更新 last_selected_index 为当前索引

#### Scenario: Shift 范围选择
- **WHEN** 用户按住 Shift 点击文件项
- **AND** 存在 last_selected_index
- **THEN** 选中从 last_selected_index 到当前索引之间的所有项
- **AND** 保持原有选中项不变（追加范围选择）

#### Scenario: Shift 范围选择（无上次选中）
- **WHEN** 用户按住 Shift 点击文件项
- **AND** last_selected_index 为 None
- **THEN** 按单选行为处理（清除其他，选中当前）

### Requirement: 选中状态视觉反馈
系统 MUST 提供清晰的选中状态视觉指示，与悬停状态区分。

#### Scenario: 未选中项默认样式
- **WHEN** 文件项未被选中且未悬停
- **THEN** 无背景色
- **AND** 无边框
- **AND** 使用默认文本颜色（foreground）

#### Scenario: 选中项边框样式
- **WHEN** 文件项被选中
- **THEN** 显示 2px 宽度的边框
- **AND** 边框颜色使用主题 accent 颜色
- **AND** 保持圆角样式
- **AND** 背景保持透明

#### Scenario: 悬停状态样式
- **WHEN** 鼠标悬停在文件项上
- **AND** 文件项未被选中
- **THEN** 背景色变为主题 muted 颜色
- **AND** 无边框

#### Scenario: 选中项悬停样式
- **WHEN** 鼠标悬停在已选中的文件项上
- **THEN** 保持 2px accent 边框
- **AND** 背景色变为主题 muted 颜色
- **AND** 边框和背景同时显示

#### Scenario: 选中状态持久性
- **WHEN** 用户选中多个文件后移开鼠标
- **THEN** 选中项的边框保持显示
- **AND** 背景色恢复为透明
- **AND** 选中状态不会因鼠标移开而消失

### Requirement: ListItem 组件选中支持
系统 MUST 扩展 ListItem 组件以支持选中状态。

#### Scenario: ListItem 选中属性
- **WHEN** 查看 ListItem 结构体定义
- **THEN** 存在 selected 字段，类型为 bool
- **AND** 提供 selected() 方法设置选中状态

#### Scenario: ListItem 选中渲染
- **WHEN** 渲染 selected 为 true 的 ListItem
- **THEN** 应用选中样式（accent 边框）
- **AND** 使用 `.when(self.selected, |this| this.border_2().border_color(accent))`

#### Scenario: ListItem 未选中渲染
- **WHEN** 渲染 selected 为 false 的 ListItem
- **THEN** 不应用边框样式
- **AND** 保留 hover 背景效果

#### Scenario: ListItem 选中事件回调
- **WHEN** ListItem 提供 on_selection_change 回调
- **THEN** 回调接收 selected 状态和 item 标识
- **AND** 回调在点击事件中触发
- **AND** 回调可访问修饰键状态（Ctrl/Cmd、Shift）

### Requirement: 键盘导航支持
系统 MUST 支持键盘操作选中状态。

#### Scenario: 方向键导航
- **WHEN** 文件列表获得焦点
- **AND** 用户按下向下箭头键
- **THEN** 焦点移动到下一个文件项
- **AND** 如果按住 Shift，选中新焦点项

#### Scenario: 空格键切换选中
- **WHEN** 文件项获得焦点
- **AND** 用户按下空格键
- **THEN** 切换当前焦点项的选中状态

#### Scenario: Ctrl/Cmd + A 全选
- **WHEN** 文件列表获得焦点
- **AND** 用户按下 Ctrl+A（Windows/Linux）或 Cmd+A（macOS）
- **THEN** 选中当前目录下所有文件和文件夹

#### Scenario: Escape 取消选中
- **WHEN** 文件列表获得焦点
- **AND** 用户按下 Escape 键
- **THEN** 清除所有选中项
- **AND** 触发界面重新渲染

