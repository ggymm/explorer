# ui-components Specification

## Purpose
TBD - created by archiving change add-panel-split. Update Purpose after archive.
## Requirements
### Requirement: 标题栏组件
系统 MUST 提供面板标题栏组件，包含路径显示和拆分控制。

#### Scenario: 标题栏基本结构
- **WHEN** 查看组件定义
- **THEN** 存在 TitleBar 结构体
- **AND** TitleBar 包含 panel_id、current_path 和拆分回调函数
- **AND** TitleBar 实现 IntoElement trait

#### Scenario: 标题栏渲染
- **WHEN** 渲染标题栏
- **THEN** 显示当前面板路径
- **AND** 显示横向拆分按钮（使用 ColumnsSplit 图标）
- **AND** 显示纵向拆分按钮（使用 RowsSplit 图标）
- **AND** 应用适当的样式（背景色、边框、内边距）

#### Scenario: 标题栏布局
- **WHEN** 标题栏渲染
- **THEN** 使用水平 flex 布局
- **AND** 路径文本位于左侧
- **AND** 拆分按钮位于右侧
- **AND** 按钮之间有适当的间距

#### Scenario: 拆分按钮交互
- **WHEN** 用户点击横向拆分按钮
- **THEN** 触发 on_split_horizontal 回调函数
- **AND** 回调函数接收 Window 和 App 上下文

#### Scenario: 拆分按钮样式
- **WHEN** 渲染拆分按钮
- **THEN** 按钮使用 ghost 或 outline 样式
- **AND** 按钮尺寸适中（small 或 default）
- **AND** 按钮显示对应的图标（ColumnsSplit 或RowsSplit）
- **AND** 按钮支持 hover 效果

### Requirement: 拆分图标支持
系统 MUST 在图标系统中支持拆分操作图标。

#### Scenario: IconName 枚举扩展
- **WHEN** 查看 IconName 枚举定义
- **THEN** 存在 ColumnsSplit 变体
- **AND** 存在 RowsSplit 变体
- **AND** 每个变体正确映射到对应的 SVG 文件路径

#### Scenario: 横向拆分图标路径
- **WHEN** 使用 IconName::ColumnsSplit
- **THEN** path 方法返回 "icons/columns-split.svg"
- **AND** 图标文件存在于 assets/icons/ 目录

#### Scenario: 纵向拆分图标路径
- **WHEN** 使用 IconName::RowsSplit
- **THEN** path 方法返回 "icons/rows-split.svg"
- **AND** 图标文件存在于 assets/icons/ 目录

#### Scenario: 图标渲染
- **WHEN** 渲染拆分图标
- **THEN** 图标正确显示 SVG 内容
- **AND** 图标尺寸与其他图标保持一致
- **AND** 图标颜色可通过 text_color 方法自定义

### Requirement: 标题栏组件导出
系统 MUST 从 explorer-component crate 导出标题栏组件。

#### Scenario: 公共 API 导出
- **WHEN** 查看 crates/explorer-component/src/lib.rs
- **THEN** 标题栏模块被声明为 pub mod title_bar
- **AND** TitleBar 结构体在 lib.rs 中重新导出
- **OR** 可通过 explorer_component::TitleBar 访问

#### Scenario: 跨 crate 使用
- **WHEN** 在 explorer-app crate 中导入
- **THEN** 可以使用 `use explorer_component::TitleBar;`
- **AND** 可以创建和渲染 TitleBar 实例
- **AND** 编译通过无错误

### Requirement: 标题栏样式集成
系统 MUST 确保标题栏样式与主题系统集成。

#### Scenario: 使用全局主题
- **WHEN** 渲染标题栏
- **THEN** 通过 `cx.global::<Theme>()` 获取主题
- **AND** 使用主题的颜色（background、border、foreground）
- **AND** 使用主题的间距（spacing.sm、spacing.md）

#### Scenario: 标题栏视觉一致性
- **WHEN** 标题栏与其他组件同时显示
- **THEN** 颜色和样式保持一致
- **AND** 边框样式与其他组件匹配
- **AND** 字体大小和行高与其他文本元素一致

#### Scenario: 激活状态样式
- **WHEN** 面板处于激活状态
- **THEN** 标题栏应用激活样式（如高亮边框或背景色）
- **AND** 激活样式使用主题的 accent 颜色
- **AND** 非激活面板使用默认样式

### Requirement: 路径面包屑导航组件
系统 MUST 提供路径面包屑导航组件，支持层级展示、快速导航和单个元素文本省略。

#### Scenario: PathBreadcrumb 组件结构
- **WHEN** 查看组件定义
- **THEN** 存在 Breadcrumb 结构体
- **AND** Breadcrumb 包含 items、is_active 字段
- **AND** Breadcrumb 包含 on_navigate 和 prefix/suffix 支持
- **AND** Breadcrumb 实现 IntoElement trait

#### Scenario: 路径解析和渲染
- **WHEN** 渲染路径 "/Users/ggymm/Documents"
- **THEN** 路径被拆分为多个可点击的 BreadcrumbItem
- **AND** 每个项显示文件夹名称（如 "Users"、"ggymm"、"Documents"）
- **AND** 项之间使用 ChevronRight 图标分隔
- **AND** 第一个项显示为根路径 "/"

#### Scenario: 单个面包屑项文本溢出
- **WHEN** 单个面包屑项的文本超过最大宽度（200px）
- **THEN** 文本被截断，末尾显示省略号（...）
- **AND** 使用 CSS overflow: hidden 和 text-overflow: ellipsis
- **AND** 使用 whitespace: nowrap 防止换行

#### Scenario: 面包屑项交互
- **WHEN** 用户点击面包屑项 "ggymm"
- **THEN** 触发 on_navigate 回调
- **AND** 回调接收完整路径参数 "/Users/ggymm"
- **AND** 应用程序导航到该路径

#### Scenario: 当前目录高亮
- **WHEN** 渲染面包屑
- **THEN** 最后一个项（当前目录）使用加粗字体
- **AND** 最后一个项使用 foreground 颜色
- **AND** 其他项使用 muted_foreground 颜色

#### Scenario: 关闭按钮支持（通过 suffix）
- **WHEN** 渲染 Breadcrumb 并设置 suffix
- **THEN** suffix 元素显示在右侧
- **AND** suffix 可以包含关闭按钮或其他控件
- **AND** suffix 支持独立的交互逻辑

#### Scenario: 面包屑布局
- **WHEN** 渲染 Breadcrumb
- **THEN** 使用水平 flex 布局
- **AND** 面包屑链占据中间空间（flex-1）
- **AND** 容器使用 overflow-hidden 防止溢出
- **AND** prefix 和 suffix 固定在两侧

### Requirement: 导航图标支持
系统 MUST 在图标系统中支持路径导航相关图标。

#### Scenario: ChevronRight 图标
- **WHEN** 查看 IconName 枚举定义
- **THEN** 存在 ChevronRight 变体
- **AND** ChevronRight 的 path() 方法返回 "icons/chevron-right.svg"
- **AND** 图标文件存在于 assets/icons/ 目录

#### Scenario: Close 图标
- **WHEN** 查看 IconName 枚举定义
- **THEN** 存在 Close 变体
- **AND** Close 的 path() 方法返回 "icons/close.svg"
- **AND** 图标文件存在于 assets/icons/ 目录

#### Scenario: 图标在面包屑中的使用
- **WHEN** 渲染 Breadcrumb
- **THEN** ChevronRight 图标正确显示在项之间
- **AND** Close 图标（如果提供）正确显示在 suffix 中
- **AND** 所有图标尺寸与文本对齐
- **AND** 所有图标颜色使用主题的 muted_foreground

### Requirement: 面包屑组件导出
系统 MUST 从 explorer-component crate 导出面包屑组件。

#### Scenario: 公共 API 导出
- **WHEN** 查看 crates/explorer-component/src/lib.rs
- **THEN** breadcrumb 模块被声明
- **AND** Breadcrumb、BreadcrumbItem、BreadcrumbState 在 lib.rs 中重新导出
- **AND** 可通过 explorer_component::Breadcrumb 访问

#### Scenario: 跨 crate 使用
- **WHEN** 在 explorer-app crate 中导入
- **THEN** 可以使用 `use explorer_component::{Breadcrumb, BreadcrumbItem};`
- **AND** 可以创建和渲染 Breadcrumb 实例
- **AND** 编译通过无错误

