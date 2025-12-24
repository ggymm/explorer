# ui-components Specification Delta

## ADDED Requirements

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

## MODIFIED Requirements

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

## REMOVED Requirements

### ~~Requirement: 面包屑整体省略号和下拉菜单~~（已移除）
**移除原因**：由于 GPUI 框架限制（AnyElement 不能 clone），无法实现准确的内容宽度测量。基于数量或估算的方案不够准确。因此移除整体省略策略（显示 `[...]` 和下拉菜单），仅保留单个元素的文本溢出省略功能。

