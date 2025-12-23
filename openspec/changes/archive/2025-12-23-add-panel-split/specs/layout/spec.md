## ADDED Requirements

### Requirement: 面板树数据结构
系统 MUST 实现递归的面板树结构，支持任意深度的面板嵌套。

#### Scenario: 定义面板节点枚举
- **WHEN** 查看代码结构
- **THEN** 存在 PanelNode 枚举，包含 Leaf 和 Split 两种变体
- **AND** Leaf 变体包含面板 ID、路径、文件列表、加载状态和错误信息
- **AND** Split 变体包含面板 ID、拆分方向（Axis）、两个子面板和 ResizableState
- **AND** 子面板使用 Box 包装以支持递归结构

#### Scenario: 面板 ID 管理
- **WHEN** 创建新面板
- **THEN** 系统生成唯一的 PanelId
- **AND** PanelId 使用递增计数器实现，确保唯一性
- **AND** PanelId 可用于查找和更新特定面板

### Requirement: 面板拆分操作
系统 MUST 支持将叶子面板拆分为两个子面板。

#### Scenario: 横向拆分面板
- **WHEN** 用户点击横向拆分按钮
- **THEN** 系统将当前激活的叶子面板转换为 Split 节点
- **AND** Split 节点的 axis 设置为 Horizontal
- **AND** 原叶子面板成为 first 子节点
- **AND** 创建新的叶子面板作为 second 子节点，继承原面板的路径
- **AND** 新面板显示在右侧

#### Scenario: 纵向拆分面板
- **WHEN** 用户点击纵向拆分按钮
- **THEN** 系统将当前激活的叶子面板转换为 Split 节点
- **AND** Split 节点的 axis 设置为 Vertical
- **AND** 原叶子面板成为 first 子节点
- **AND** 创建新的叶子面板作为 second 子节点，继承原面板的路径
- **AND** 新面板显示在下方

#### Scenario: 嵌套拆分
- **WHEN** 用户在已拆分的面板中的某个子面板上再次执行拆分操作
- **THEN** 系统递归查找目标叶子面板
- **AND** 将目标叶子面板拆分为 Split 节点
- **AND** 保持其他面板不变
- **AND** 支持任意深度的嵌套拆分

#### Scenario: 无激活面板时的拆分
- **WHEN** 用户执行拆分操作但没有激活的面板
- **THEN** 系统使用根面板作为拆分目标
- **OR** 如果是首次拆分，创建初始的两栏布局
- **AND** 新面板使用用户主目录作为默认路径

### Requirement: 激活面板管理
系统 MUST 跟踪当前激活的面板。

#### Scenario: 面板激活
- **WHEN** 用户点击某个面板的内容区域
- **THEN** 该面板被标记为激活状态
- **AND** 更新 Explorer 中的 active_panel_id 字段
- **AND** 之前激活的面板失去激活状态

#### Scenario: 激活面板视觉反馈
- **WHEN** 面板处于激活状态
- **THEN** 该面板的标题栏或边框显示高亮效果
- **AND** 用户可以清楚识别当前激活的面板

#### Scenario: 拆分按钮上下文
- **WHEN** 用户点击拆分按钮
- **THEN** 拆分操作针对当前激活的面板执行
- **AND** 如果没有激活面板，使用根面板或默认行为

### Requirement: 面板树递归渲染
系统 MUST 递归渲染面板树结构。

#### Scenario: 渲染叶子面板
- **WHEN** 渲染 Leaf 类型的 PanelNode
- **THEN** 显示面板标题栏，包含当前路径和拆分按钮
- **AND** 显示文件列表内容
- **AND** 应用适当的样式和边距

#### Scenario: 渲染分支节点
- **WHEN** 渲染 Split 类型的 PanelNode
- **THEN** 使用 Resizable 组件包装两个子面板
- **AND** 根据 axis 设置 Resizable 的方向（横向或纵向）
- **AND** 递归渲染 first 和 second 子面板
- **AND** 保持 ResizableState 用于拖拽调整大小

#### Scenario: 初始渲染
- **WHEN** 应用启动时
- **THEN** 主面板区域显示单个叶子面板
- **AND** 面板包含标题栏和文件列表
- **AND** 标题栏显示拆分按钮

## MODIFIED Requirements

### Requirement: 主面板组件
系统 MUST 实现右侧主面板组件，支持单面板和多面板拆分模式。

#### Scenario: 主面板基本显示
- **WHEN** 应用启动
- **THEN** 右侧主面板可见
- **AND** 主面板占据右侧所有可用空间
- **AND** 初始显示单个面板，包含标题栏和文件列表

#### Scenario: 主面板显示当前路径
- **WHEN** 面板渲染
- **THEN** 每个面板的标题栏顶部显示当前目录路径
- **AND** 路径清晰可读
- **AND** 路径随导航更新

#### Scenario: 主面板拆分模式
- **WHEN** 用户执行拆分操作后
- **THEN** 主面板显示多个子面板
- **AND** 子面板之间使用 Resizable 组件分隔
- **AND** 每个子面板独立显示内容和标题栏
- **AND** 子面板可以继续拆分

#### Scenario: 面板数据独立性
- **WHEN** 主面板包含多个子面板
- **THEN** 每个子面板独立管理自己的路径和文件列表
- **AND** 在一个面板中导航不影响其他面板
- **AND** 每个面板可以独立加载数据
