# 实施任务清单

## 1. 基础数据结构和类型定义
- [x] 1.1 在 `crates/explorer-common/src/types.rs` 中定义 `PanelId` 类型（使用 u64 或 usize）
- [x] 1.2 在 `crates/explorer-app/src/main.rs` 中定义 `PanelNode` 枚举（Leaf 和 Split 变体）
- [x] 1.3 实现 `PanelId` 生成器（使用 AtomicU64 或简单的计数器）
- [x] 1.4 为 `PanelNode` 实现基本方法（new_leaf、get_id）

## 2. 图标系统扩展
- [x] 2.1 在 `crates/explorer-component/src/icon.rs` 中的 `IconName` 枚举添加 `ColumnsSplit` 变体
- [x] 2.2 在 `IconName` 枚举中添加 `RowsSplit` 变体
- [x] 2.3 在 `IconNamed` trait 实现中添加两个新图标的路径映射
- [x] 2.4 验证图标文件存在且路径正确

## 3. 标题栏组件实现
- [x] 3.1 创建 `crates/explorer-component/src/title_bar.rs` 文件
- [x] 3.2 定义 `TitleBar` 结构体，包含必要字段（panel_id、current_path、回调函数）
- [x] 3.3 实现 `TitleBar::new()` 构造函数
- [x] 3.4 实现设置回调函数的 builder 方法（on_split_horizontal、on_split_vertical）
- [x] 3.5 实现 `IntoElement` trait，渲染标题栏布局
- [x] 3.6 添加路径显示区域（左侧）
- [x] 3.7 添加拆分按钮区域（右侧），使用新的图标
- [x] 3.8 应用主题样式（背景色、边框、间距）
- [x] 3.9 在 `crates/explorer-component/src/lib.rs` 中导出 TitleBar

## 4. 面板拆分逻辑实现
- [x] 4.1 在 `PanelNode` 中实现 `split_panel` 方法，支持横向拆分
- [x] 4.2 扩展 `split_panel` 方法，支持纵向拆分
- [x] 4.3 实现递归查找目标面板的逻辑
- [x] 4.4 实现将叶子节点转换为分支节点的逻辑
- [x] 4.5 确保新面板继承当前面板的路径
- [x] 4.6 实现面板查找辅助函数（find_panel_by_id）
- [x] 4.7 实现面板更新辅助函数（update_panel_data）

## 5. Explorer 状态管理集成
- [x] 5.1 在 `Explorer` 结构体中添加 `panel_tree: PanelNode` 字段
- [x] 5.2 在 `Explorer` 结构体中添加 `active_panel_id: Option<PanelId>` 字段
- [x] 5.3 在 `Explorer` 结构体中添加 `next_panel_id: u64` 字段（或使用 AtomicU64）
- [x] 5.4 修改 `Explorer::new()` 创建初始的单叶子面板树
- [x] 5.5 实现 `Explorer::split_panel_horizontal` 方法
- [x] 5.6 实现 `Explorer::split_panel_vertical` 方法
- [x] 5.7 实现 `Explorer::set_active_panel` 方法
- [x] 5.8 实现 `Explorer::load_directory_for_panel` 方法（为特定面板加载目录）

## 6. 面板树递归渲染
- [x] 6.1 实现 `render_panel_node` 辅助方法，处理递归渲染
- [x] 6.2 实现 Leaf 节点渲染逻辑（标题栏 + 文件列表）
- [x] 6.3 实现 Split 节点渲染逻辑（使用 Resizable 包装两个子面板）
- [x] 6.4 在 `Explorer::render` 方法中调用 `render_panel_node` 替换现有的文件列表渲染
- [x] 6.5 确保 Resizable 的 axis 参数根据 Split 节点正确设置
- [x] 6.6 传递正确的 ResizableState 给每个 Split 节点

## 7. 交互和事件处理
- [x] 7.1 在每个叶子面板的内容区域添加点击事件处理器，更新 active_panel_id
- [x] 7.2 连接标题栏横向拆分按钮到 `Explorer::split_panel_horizontal`
- [x] 7.3 连接标题栏纵向拆分按钮到 `Explorer::split_panel_vertical`
- [x] 7.4 实现激活面板的视觉反馈（标题栏高亮或边框变色）
- [x] 7.5 确保拆分操作后正确更新 active_panel_id
- [x] 7.6 测试无激活面板时的拆分行为（使用根面板）

## 8. 初始化和数据加载
- [x] 8.1 修改 `Explorer::init` 方法，为根面板加载初始数据
- [x] 8.2 确保新拆分的面板自动加载数据（继承父面板路径）
- [x] 8.3 测试面板独立导航（在一个面板中切换目录不影响其他面板）
- [x] 8.4 确保每个面板的加载状态和错误信息独立管理

## 9. 样式和主题集成
- [x] 9.1 确保标题栏使用全局 Theme 的颜色和间距
- [x] 9.2 为激活面板的标题栏应用 accent 颜色
- [x] 9.3 调整面板之间的视觉分隔（边框、阴影）
- [x] 9.4 确保多层嵌套时样式保持一致
- [x] 9.5 测试暗色主题下的视觉效果

## 10. 测试和验证
- [x] 10.1 测试单面板场景（初始状态）
- [x] 10.2 测试简单横向拆分（2 个面板）
- [x] 10.3 测试简单纵向拆分（2 个面板）
- [x] 10.4 测试嵌套拆分（先横向再纵向，或反之）
- [x] 10.5 测试深度嵌套（3-4 层）
- [x] 10.6 测试激活面板切换
- [x] 10.7 测试面板独立导航（每个面板显示不同目录）
- [x] 10.8 测试拖拽调整面板大小
- [x] 10.9 测试边界情况（窗口缩小、最小宽度限制）
- [x] 10.10 性能测试（多面板同时加载大目录）

## 11. 代码清理和文档
- [x] 11.1 清理临时代码和调试日志
- [x] 11.2 为新增的公共 API 添加文档注释
- [x] 11.3 确保代码符合项目的导入语句规范
- [x] 11.4 运行 `cargo fmt` 格式化代码
- [x] 11.5 运行 `cargo clippy` 检查并修复警告
- [x] 11.6 更新 README.md（如有必要）

## 依赖关系说明

- **任务 2** 独立，可以首先完成
- **任务 1** 必须在 **任务 3、4、5** 之前完成（提供基础类型）
- **任务 3** 必须在 **任务 7** 之前完成（提供 UI 组件）
- **任务 4** 必须在 **任务 5、6** 之前完成（提供核心逻辑）
- **任务 5** 必须在 **任务 6、7、8** 之前完成（提供状态管理）
- **任务 6、7、8** 可以并行开发，但都依赖前面的任务
- **任务 9** 可以与其他任务并行，但最好在主要功能完成后进行
- **任务 10** 在所有功能实现完成后进行
- **任务 11** 在测试通过后最后执行

## 可并行化的工作

- **任务 2**（图标）和 **任务 1**（数据结构）可以同时进行
- **任务 3**（标题栏组件）可以在 **任务 1** 完成后独立开发
- **任务 9**（样式）的部分工作可以在组件开发过程中同步进行
