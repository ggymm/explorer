# file-list Specification Delta

## ADDED Requirements

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

## MODIFIED Requirements

无需修改现有 file-list 规范，上述为新增需求。
