# 任务列表

## 1. 路径面包屑组件 (PathBreadcrumb)

### 1.1 重命名和基础结构
- [x] 将 `crates/explorer-component/src/panel_title_bar.rs` 重命名为 `breadcrumb.rs`
- [x] 将 `PanelTitleBar` 结构体重命名为 `Breadcrumb`
- [x] 更新 `lib.rs` 中的模块声明和导出
- [x] 更新 `main.rs` 中的导入语句

### 1.2 图标支持
- [x] 在 `icon.rs` 的 `IconName` 枚举中添加 `ChevronRight` 变体
- [x] 在 `icon.rs` 的 `IconName` 枚举中添加 `Close` 变体
- [x] 在 `icon.rs` 的 `IconName` 枚举中添加 `Ellipsis` 变体
- [x] 实现 `ChevronRight` 的 `path()` 方法，返回 `"icons/chevron-right.svg"`
- [x] 实现 `Close` 的 `path()` 方法，返回 `"icons/close.svg"`
- [x] 实现 `Ellipsis` 的 `path()` 方法，返回 `"icons/ellipsis.svg"`
- [x] 验证所有图标文件已存在于 `assets/icons/` 目录

### 1.3 路径解析和面包屑渲染
- [x] 实现路径解析函数 `parse_path`，将完整路径拆分为 PathSegment 数组
- [x] 处理根路径和特殊路径（如 `/`、`C:\`）
- [x] ~~实现文件夹名称截断逻辑 `truncate_segment`~~（简化方案：使用单元素文本溢出）
  - ~~设置最大显示宽度（默认 120px）~~
  - ~~超出宽度的名称添加省略号~~
  - ~~记录 truncated 状态用于显示 tooltip~~
- [x] ~~实现路径省略策略 `apply_ellipsis`~~（简化方案：CSS 文本溢出处理）
  - ~~获取容器可用宽度~~
  - ~~计算完整面包屑链的总宽度（`calculate_breadcrumb_width`）~~
  - ~~当总宽度超过容器宽度时应用省略~~
  - ~~保留：根目录 + 第一级 + 省略号 + 倒数第二级 + 当前目录~~
  - ~~将中间层级存储在 Ellipsis 结构中~~
- [x] 设计 PathBreadcrumb 的 render 方法：
  - 遍历 BreadcrumbItem 数组
  - 渲染普通 Segment：可点击的文本元素
  - ~~渲染 Ellipsis：可点击的省略号图标~~
  - 在项之间插入 ChevronRight 图标
  - 最后一个段使用不同样式（非加粗，使用颜色区分）
- [x] ~~为截断的段添加 tooltip 显示完整名称~~（简化方案不需要）
- [x] 添加 `on_navigate` 回调，接收完整路径参数

### 1.4 省略号菜单
- [x] ~~实现省略号菜单组件 `EllipsisMenu`~~（简化方案不需要）
- [x] ~~管理菜单显示/隐藏状态（使用 Entity<EllipsisMenuState>）~~
- [x] ~~点击省略号图标时显示菜单~~
- [x] ~~菜单项渲染：~~
  - ~~显示被省略的所有中间层级~~
  - ~~每个层级显示文件夹名称~~
  - ~~点击菜单项触发导航~~
- [x] ~~菜单定位：~~
  - ~~使用绝对定位~~
  - ~~出现在省略号图标正下方~~
  - ~~添加阴影和边框~~
- [x] ~~点击菜单外部时关闭菜单~~
- [x] ~~菜单滚动支持（超过最大高度时）~~

### 1.5 关闭按钮
- [x] 在面包屑右侧添加 Close 按钮
- [x] 按钮使用 ghost 或 outline 样式
- [x] 添加 `on_close` 回调
- [x] 添加悬停效果（背景色变化）

### 1.6 样式调整
- [x] 使用 flex 布局，左侧面包屑，右侧关闭按钮
- [x] 设置适当的间距和内边距
- [x] 确保与主题系统集成（颜色、字体大小）
- [x] 添加激活状态的视觉反馈

## 2. 文件列表选中状态

### 2.1 选中状态数据结构
- [x] 在 Explorer 结构体中添加 `selected_items: HashSet<String>` 字段
- [x] 实现 `add_selection`、`remove_selection`、`clear_selection` 方法
- [x] 实现 `toggle_selection` 方法（切换选中状态）
- [x] 实现 `is_selected` 方法（查询是否选中）

### 2.2 ListItem 组件扩展
- [x] 为 ListItem 添加 `selected` 属性支持
- [x] 修改 render 方法，根据 `selected` 状态应用不同样式：
  - 选中：使用背景色高亮（Fluent UI 风格，无边框）
  - 未选中：透明背景
- [x] 确保悬停和选中状态可同时显示
- [x] 添加 `on_selection_change` 回调

### 2.3 多选支持
- [x] 实现单击选中逻辑（清除其他选中项）
- [x] 实现 Ctrl/Cmd + 单击切换选中逻辑
- [x] 实现 Shift + 单击范围选择逻辑
- [x] 在 Explorer 中维护 `last_selected_index` 用于范围选择

### 2.4 键盘导航（可选）
- [ ] 实现方向键导航（上/下移动焦点）（未实现，不在当前范围）
- [ ] 实现空格键切换选中状态（未实现，不在当前范围）
- [ ] 实现 Ctrl/Cmd + A 全选（未实现，不在当前范围）
- [ ] 实现 Escape 取消所有选中（未实现，不在当前范围）

## 3. 面板关闭功能

### 3.1 面板移除逻辑
- [x] 在 PanelNode 中实现 `remove_panel` 方法
- [x] 处理叶子节点关闭：
  - 找到父 Split 节点
  - 用兄弟节点替换父节点
  - 更新面板树结构
- [x] 处理 Split 节点关闭：递归移除所有子节点
- [x] 添加边界检查：最后一个面板不可关闭

### 3.2 激活状态处理
- [x] 关闭激活面板时，自动激活相邻面板（优先右侧/下方）
- [x] 实现 `find_next_panel_id` 方法（深度优先遍历）
- [x] 更新 `active_panel_id` 字段

### 3.3 UI 集成
- [x] 在 PathBreadcrumb 中连接 `on_close` 回调到 Explorer 的关闭方法
- [x] 传递当前面板 ID 给关闭方法
- [x] 触发重新渲染

## 4. 测试和验证

### 4.1 PathBreadcrumb 测试
- [x] 测试简单路径：`/Users/ggymm/Documents`
- [x] 测试根路径：`/`
- [x] ~~测试长路径（宽度不足）：验证省略号显示~~（简化方案使用 CSS 溢出处理）
  - ~~在窄容器中渲染：`/Users/ggymm/Projects/Rust/explorer/src/components`~~
  - ~~预期：`/ > Users > [...] > src > components`~~
- [x] ~~测试容器宽度变化~~（简化方案不需要）
  - ~~从宽容器调整到窄容器：验证省略号自动出现~~
  - ~~从窄容器调整到宽容器：验证省略号自动消失~~
- [x] 测试超长文件夹名称：验证 CSS 截断
  - 示例：`/very-long-folder-name-that-exceeds-maximum-width`
  - 实际：使用 max_w + overflow_hidden + whitespace_nowrap
- [x] 测试点击面包屑项：验证导航回调
- [x] ~~测试点击省略号图标~~（简化方案不需要）
- [x] ~~测试省略号菜单项~~（简化方案不需要）
- [x] ~~测试点击菜单外部~~（简化方案不需要）
- [x] 测试关闭按钮：验证关闭回调

### 4.2 文件选中测试
- [x] 测试单选：点击单个文件，验证背景色显示
- [x] 测试 Ctrl 多选：验证多个文件同时选中
- [x] 测试 Shift 范围选择：验证连续文件选中
- [x] 测试悬停：验证悬停和选中状态同时显示
- [x] 测试取消选中：验证选中状态正确清除

### 4.3 面板关闭测试
- [x] 测试关闭中间面板：验证树结构正确更新
- [x] 测试关闭激活面板：验证新面板自动激活
- [x] 测试关闭最后一个面板：验证关闭按钮禁用或隐藏
- [x] 测试连续关闭：验证多次关闭后状态一致

## 5. 额外完成的改进

### 5.1 主题系统迁移
- [x] 将主题从 shadcn/ui 迁移到 Fluent UI Design System
- [x] 更新所有颜色为 Fluent UI 官方规范
- [x] 为列表组件添加独立颜色系统（背景、悬停、选中等）
- [x] 为面包屑组件添加独立颜色系统（区别于列表）
- [x] 调整圆角大小（2px/6px/8px）
- [x] 调整透明度级别（列表 4%/8%/12%，面包屑 2.5%）

### 5.2 可调整大小组件优化
- [x] 增加拖拽识别区域从 2px 到 8px
- [x] 视觉分割线保持 2px
- [x] 拖拽时全局显示调整大小光标
- [x] 防止鼠标离开拖拽区时光标丢失

### 5.3 文件排序优化
- [x] 实现多级排序：隐藏状态 > 类型（目录/文件）> 名称
- [x] 隐藏文件排列在普通文件之后
- [x] 保持目录优先于文件
- [x] 不区分大小写的名称排序

## 6. 文档更新

- [ ] 更新 ui-components 规范：添加 PathBreadcrumb 需求（待定）
- [ ] 更新 file-list 规范：添加选中状态需求（待定）
- [ ] 更新组件使用示例（待定）
- [ ] 更新 README：描述新功能（待定）
