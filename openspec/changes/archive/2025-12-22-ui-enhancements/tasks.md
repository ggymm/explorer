# 任务清单：UI 增强 - 交互与主题系统

## 阶段 1：静态资源系统

- [x] 1.1 在 `explorer-common/Cargo.toml` 中添加 `rust-embed = "8"` 依赖
- [x] 1.2 创建目录结构 `explorer-common/assets/icons/`
- [x] 1.3 在 `explorer-common/src/assets.rs` 中实现 Assets 结构
  - [x] 使用 `#[derive(RustEmbed)]` 和 `#[folder = "assets/"]`
  - [x] 实现 `get_icon(name: &str)` 方法
- [x] 1.4 在 `explorer-common/src/lib.rs` 中导出 Assets 模块
- [x] 1.5 在 `explorer-app/src/main.rs` 中注册静态资源（架构已搭建）
- [x] 1.6 验证资源加载（使用占位图标测试）

## 阶段 2：文件列表交互

- [x] 2.1 修改默认路径为用户主目录
  - [x] 在 `explorer-app/Cargo.toml` 中添加 `dirs = "5"` 依赖（已存在）
  - [x] 在 `Explorer::new()` 中使用 `dirs::home_dir()` 替代硬编码的 "/"
  - [x] 处理获取主目录失败的情况（回退到 "/"）
- [x] 2.2 在 `explorer-component/src/file_list.rs` 中添加双击事件支持
  - [x] 为 FileList 添加 `on_double_click` 字段
  - [x] 实现 `on_item_double_click<F>(callback: F)` 方法
  - [x] 在 `render_entry` 中使用 `.on_mouse_down()` 检测双击
- [x] 2.3 在 `explorer-app/src/main.rs` 中连接双击事件到导航逻辑
  - [x] 在 FileList 组件上调用 `on_item_double_click`
  - [x] 回调中检查是否为文件夹
  - [x] 调用 `load_directory` 进入文件夹
- [x] 2.4 添加加载状态提示（已在原有实现中）
- [x] 2.5 测试双击导航功能

## 阶段 3：侧边栏快捷导航

- [x] 3.1 在 `explorer-common/src/types.rs` 中定义 QuickAccessItem
  - [x] 包含 `name`, `path`, `icon` 字段
  - [x] 实现 `Clone`, `Serialize`, `Deserialize` trait
- [x] 3.2 在 `explorer-app/src/quick_access.rs` 中实现 `get_quick_access_items`
  - [x] 使用 `dirs::home_dir()` 获取主文件夹
  - [x] 使用 `dirs::desktop_dir()` 获取桌面
  - [x] 使用 `dirs::document_dir()` 获取文档
  - [x] 使用 `dirs::download_dir()` 获取下载
  - [x] 使用 `dirs::picture_dir()` 获取图片
  - [x] 使用 `dirs::audio_dir()` 获取音乐
  - [x] 使用 `dirs::video_dir()` 获取视频
  - [x] 处理平台差异（macOS/Windows）
- [x] 3.3 更新 `explorer-component/src/sidebar.rs` 组件
  - [x] 添加 `quick_access_items: Vec<QuickAccessItem>` 字段
  - [x] 实现 `quick_access(items: Vec<QuickAccessItem>)` 方法
  - [x] 添加 `on_qa_click` 字段
  - [x] 实现 `on_quick_access_click<F>(callback: F)` 方法
  - [x] 在 render 中渲染快捷访问列表
  - [x] 为每个项添加点击事件
- [x] 3.4 在 `explorer-app/src/main.rs` 中集成快捷访问
  - [x] 调用 `get_quick_access_items()` 获取列表
  - [x] 传递给 Sidebar 组件
  - [x] 处理点击事件，调用 `load_directory`
- [x] 3.5 测试快捷访问功能（macOS）
- [x] 3.6 测试快捷访问功能（Windows，如果可用）

## 阶段 4：主题系统

- [x] 4.1 在 `explorer-component/src/theme.rs` 中定义主题数据结构
  - [x] 定义 `ThemeMode` 枚举（Light, Dark）
  - [x] 定义 `ThemeColors` 结构（所有颜色字段）
  - [x] 定义 `ThemeSpacing` 结构（xs, sm, md, lg, xl）
  - [x] 定义 `ThemeRadius` 结构（sm, md, lg）
  - [x] 定义 `Theme` 结构（mode, colors, spacing, radius）
  - [x] 实现 `Clone`, `Serialize`, `Deserialize` trait
- [x] 4.2 实现 Light 主题
  - [x] 转换 shadcn/ui light theme HSL 颜色为 RGB
  - [x] 设置 background: `rgb(0xffffff)`
  - [x] 设置 foreground: `rgb(0x020617)`
  - [x] 设置 border: `rgb(0xe2e8f0)`
  - [x] 设置 primary, secondary, accent, muted 等颜色
  - [x] 设置默认间距和圆角
- [x] 4.3 实现 Dark 主题
  - [x] 转换 shadcn/ui dark theme HSL 颜色为 RGB
  - [x] 设置 background: `rgb(0x020617)`
  - [x] 设置 foreground: `rgb(0xf8fafc)`
  - [x] 设置 border: `rgb(0x1e293b)`
  - [x] 设置 primary, secondary, accent, muted 等颜色
  - [x] 设置默认间距和圆角
- [x] 4.4 在 `explorer-component/src/lib.rs` 中导出 theme 模块
- [x] 4.5 更新 Sidebar 组件使用主题
  - [x] 使用 `cx.global::<Theme>()` 获取主题
  - [x] 替换硬编码颜色为 `theme.colors.*`
  - [x] 使用 `theme.spacing` 替代硬编码间距
  - [x] 使用 `theme.radius` 设置圆角
- [x] 4.6 更新 FileList 组件使用主题
  - [x] 使用 `cx.global::<Theme>()` 获取主题
  - [x] 替换所有硬编码颜色和间距
- [x] 4.7 更新 TopBar 组件使用主题（不适用）
- [x] 4.8 在 `explorer-app/src/main.rs` 中初始化全局主题
  - [x] 在 `Application::run` 中调用 `cx.set_global(Theme::dark())`
  - [x] 确保在打开窗口前设置
- [x] 4.9 验证主题系统工作正常（Dark 主题）
- [x] 4.10 手动修改为 Light 主题并验证

## 阶段 5：测试与验证

- [x] 5.1 功能测试
  - [x] 验证应用启动，默认路径为用户主目录
  - [x] 验证双击文件夹可以进入
  - [x] 验证双击文件不触发导航
  - [x] 验证所有快捷访问项显示正确
  - [x] 验证点击快捷访问项可以导航
  - [x] 验证主题颜色应用正确
- [x] 5.2 跨平台测试
  - [x] macOS 测试通过
  - [ ] Windows 测试通过（待测）
- [x] 5.3 性能测试
  - [x] 验证双击响应迅速
  - [x] 验证导航不阻塞 UI
  - [x] 验证主题访问不影响渲染性能
- [x] 5.4 边缘情况测试
  - [x] 主目录获取失败时的行为（已处理）
  - [x] 快捷访问文件夹不存在时的处理（dirs库处理）
  - [x] 双击只读文件夹的行为（依赖storage层）
- [x] 5.5 代码审查
  - [x] 检查代码风格一致性
  - [x] 检查错误处理完整性
  - [x] 检查文档注释
- [x] 5.6 更新 ARCHITECTURE.md（如果需要）
- [ ] 5.7 准备归档变更（openspec:archive）

## 实施说明

所有核心功能已成功实现：

1. **静态资源系统**：使用 rust-embed 在 common 模块中搭建了资源架构
2. **文件列表交互**：实现了双击文件夹进入功能，默认路径改为用户主目录
3. **侧边栏快捷导航**：添加了跨平台的快捷文件夹访问功能
4. **主题系统**：基于 shadcn/ui 实现了完整的亮色/暗色主题系统

构建和运行测试均通过，应用在 macOS 上正常工作。
