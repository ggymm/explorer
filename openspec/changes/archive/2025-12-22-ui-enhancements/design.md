# 设计文档：UI 增强 - 交互与主题系统

## 上下文

Explorer 已完成基础架构和布局实现，现在需要添加用户交互功能和视觉主题系统，使应用真正可用。

### 约束条件
- 必须保持现有架构的分层设计
- 主题系统必须易于扩展
- 交互必须响应迅速，不阻塞 UI
- 跨平台兼容（macOS、Windows）

### 利益相关者
- 开发者：需要清晰的主题 API 和交互模式
- 用户：需要直观的交互和舒适的视觉体验

## 架构决策

### 1. 静态资源管理架构

**决策**：使用 rust-embed 在 common 模块中管理静态资源

```rust
// explorer-common/src/assets.rs
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

impl Assets {
    pub fn get_icon(name: &str) -> Option<Vec<u8>> {
        Self::get(&format!("icons/{}.svg", name))
            .map(|data| data.data.to_vec())
    }
}
```

**资源结构**：
```
explorer-common/
├── assets/
│   └── icons/
│       ├── folder.svg
│       ├── file.svg
│       ├── desktop.svg
│       ├── documents.svg
│       ├── downloads.svg
│       └── ...
└── src/
    └── assets.rs
```

**在 app 中注册**：
```rust
// explorer-app/src/main.rs
use gpui::{prelude::*,*};

fn main() {
    let app = Application::new();
    app.run(move |cx| {
        // 注册静态资源
        cx.asset_source()
            .load_embedded::<explorer_common::Assets>();

        // ... 其他初始化代码
    });
}
```

**理由**：
- rust-embed 支持编译时嵌入，无需运行时文件系统访问
- common 模块职责明确：提供共享数据和资源
- GPUI 原生支持嵌入式资源加载

### 2. 文件列表交互设计

**决策**：使用 GPUI 的事件系统实现双击交互

```rust
// explorer-component/src/file_list.rs
impl FileList {
    pub fn on_item_double_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&FileItem) + 'static,
    {
        self.on_double_click = Some(Box::new(callback));
        self
    }

    fn render_entry(&self, entry: &FileItem) -> impl IntoElement {
        let on_double_click = self.on_double_click.clone();
        let entry_clone = entry.clone();

        div()
            .on_click(move |_, _, _| {
                // 单击选中
            })
            .on_double_click(move |_, _, _| {
                if let Some(callback) = &on_double_click {
                    callback(&entry_clone);
                }
            })
            // ... 其他属性
    }
}
```

**默认路径修改**：
```rust
// explorer-app/src/main.rs
use dirs::home_dir;

impl Explorer {
    pub fn new() -> Self {
        let provider: Arc<dyn StorageProvider> = Arc::new(LocalFileSystemProvider::new());
        let default_path = home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        Self {
            provider,
            roots: Vec::new(),
            current_path: default_path,
            // ...
        }
    }
}
```

**理由**：
- 回调模式符合 GPUI 的设计哲学
- 双击是文件管理器的标准交互
- 用户主目录是最合理的默认起点

### 3. 侧边栏快捷导航设计

**决策**：定义 QuickAccessItem 类型，使用 dirs 库获取系统路径

```rust
// explorer-common/src/types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAccessItem {
    pub name: String,
    pub path: String,
    pub icon: String,  // 图标名称
}

// explorer-app/src/quick_access.rs
use dirs;

pub fn get_quick_access_items() -> Vec<QuickAccessItem> {
    let mut items = Vec::new();

    if let Some(home) = dirs::home_dir() {
        items.push(QuickAccessItem {
            name: "主文件夹".to_string(),
            path: home.to_string_lossy().to_string(),
            icon: "home".to_string(),
        });
    }

    if let Some(desktop) = dirs::desktop_dir() {
        items.push(QuickAccessItem {
            name: "桌面".to_string(),
            path: desktop.to_string_lossy().to_string(),
            icon: "desktop".to_string(),
        });
    }

    if let Some(documents) = dirs::document_dir() {
        items.push(QuickAccessItem {
            name: "文档".to_string(),
            path: documents.to_string_lossy().to_string(),
            icon: "documents".to_string(),
        });
    }

    if let Some(downloads) = dirs::download_dir() {
        items.push(QuickAccessItem {
            name: "下载".to_string(),
            path: downloads.to_string_lossy().to_string(),
            icon: "downloads".to_string(),
        });
    }

    // Pictures, Music, Videos 类似

    items
}
```

**侧边栏组件更新**：
```rust
// explorer-component/src/sidebar.rs
impl Sidebar {
    pub fn quick_access(mut self, items: Vec<QuickAccessItem>) -> Self {
        self.quick_access_items = items;
        self
    }

    pub fn on_quick_access_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&QuickAccessItem) + 'static,
    {
        self.on_qa_click = Some(Box::new(callback));
        self
    }
}
```

**理由**：
- dirs 库提供跨平台的标准目录访问
- 图标名称字符串便于资源查找
- 快捷访问是文件管理器的标准功能

### 4. 主题系统设计

**决策**：在 component 模块中实现基于 Context 的主题系统

**主题定义**（使用 shadcn/ui 颜色）：
```rust
// explorer-component/src/theme.rs
use gpui::{prelude::*,*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub radius: ThemeRadius,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    // 背景色
    pub background: Rgba,      // hsl(0 0% 100%) / hsl(222.2 84% 4.9%)
    pub foreground: Rgba,      // hsl(222.2 84% 4.9%) / hsl(210 40% 98%)

    // 卡片
    pub card: Rgba,
    pub card_foreground: Rgba,

    // 边框
    pub border: Rgba,          // hsl(214.3 31.8% 91.4%) / hsl(217.2 32.6% 17.5%)
    pub input: Rgba,

    // 主要色
    pub primary: Rgba,         // hsl(222.2 47.4% 11.2%) / hsl(210 40% 98%)
    pub primary_foreground: Rgba,

    // 次要色
    pub secondary: Rgba,       // hsl(210 40% 96.1%) / hsl(217.2 32.6% 17.5%)
    pub secondary_foreground: Rgba,

    // 强调色
    pub accent: Rgba,          // hsl(210 40% 96.1%) / hsl(217.2 32.6% 17.5%)
    pub accent_foreground: Rgba,

    // 弱化
    pub muted: Rgba,           // hsl(210 40% 96.1%) / hsl(217.2 32.6% 17.5%)
    pub muted_foreground: Rgba, // hsl(215.4 16.3% 46.9%) / hsl(215 20.2% 65.1%)

    // 破坏性
    pub destructive: Rgba,
    pub destructive_foreground: Rgba,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThemeSpacing {
    pub xs: Pixels,   // 4px
    pub sm: Pixels,   // 8px
    pub md: Pixels,   // 16px
    pub lg: Pixels,   // 24px
    pub xl: Pixels,   // 32px
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThemeRadius {
    pub sm: Pixels,   // 4px
    pub md: Pixels,   // 8px
    pub lg: Pixels,   // 12px
}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ThemeColors {
                background: rgb(0xffffff),
                foreground: rgb(0x020617),
                // ... shadcn/ui light theme colors
                border: rgb(0xe2e8f0),
                primary: rgb(0x0f172a),
                muted: rgb(0xf1f5f9),
                muted_foreground: rgb(0x64748b),
                // ... 其他颜色
            },
            spacing: ThemeSpacing {
                xs: px(4.),
                sm: px(8.),
                md: px(16.),
                lg: px(24.),
                xl: px(32.),
            },
            radius: ThemeRadius {
                sm: px(4.),
                md: px(8.),
                lg: px(12.),
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ThemeColors {
                background: rgb(0x020617),
                foreground: rgb(0xf8fafc),
                // ... shadcn/ui dark theme colors
                border: rgb(0x1e293b),
                primary: rgb(0xf8fafc),
                muted: rgb(0x1e293b),
                muted_foreground: rgb(0x94a3b8),
                // ... 其他颜色
            },
            spacing: ThemeSpacing::default(),
            radius: ThemeRadius::default(),
        }
    }
}
```

**主题使用方式**：
```rust
// explorer-component/src/sidebar.rs
impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.colors.background)
            .border_r_1()
            .border_color(theme.colors.border)
            .p(theme.spacing.md)
            // ... 其他样式从主题获取
    }
}
```

**在 App 中初始化主题**：
```rust
// explorer-app/src/main.rs
fn main() {
    let app = Application::new();
    app.run(move |cx| {
        // 设置全局主题
        cx.set_global(Theme::dark());  // 或 Theme::light()

        // ... 其他初始化
    });
}
```

**理由**：
- 使用 GPUI 的 Global context 实现主题共享
- shadcn/ui 是成熟的设计系统，颜色值经过验证
- 所有样式从主题获取，易于维护和扩展
- 支持运行时切换主题（后续功能）

## 技术选型

### 新增依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| rust-embed | 8.x | 嵌入静态资源 |
| dirs | 5.x | 跨平台系统目录访问 |

### shadcn/ui 颜色映射

**Light Theme**:
- background: `hsl(0 0% 100%)` → `#ffffff`
- foreground: `hsl(222.2 84% 4.9%)` → `#020617`
- border: `hsl(214.3 31.8% 91.4%)` → `#e2e8f0`
- muted: `hsl(210 40% 96.1%)` → `#f1f5f9`
- muted-foreground: `hsl(215.4 16.3% 46.9%)` → `#64748b`

**Dark Theme**:
- background: `hsl(222.2 84% 4.9%)` → `#020617`
- foreground: `hsl(210 40% 98%)` → `#f8fafc`
- border: `hsl(217.2 32.6% 17.5%)` → `#1e293b`
- muted: `hsl(217.2 32.6% 17.5%)` → `#1e293b`
- muted-foreground: `hsl(215 20.2% 65.1%)` → `#94a3b8`

## 实施计划

### 阶段 1：静态资源系统（优先级：高）
1. 在 explorer-common 中添加 rust-embed 依赖
2. 创建 assets 目录结构
3. 实现 Assets 结构和接口
4. 在 explorer-app 中注册资源
5. 验证资源加载

### 阶段 2：文件列表交互（优先级：高）
1. 修改默认路径为用户主目录
2. 为 FileList 添加双击事件支持
3. 在 Explorer 中处理双击，调用 load_directory
4. 添加加载状态提示
5. 测试双击导航

### 阶段 3：侧边栏快捷导航（优先级：高）
1. 定义 QuickAccessItem 类型
2. 实现 get_quick_access_items 函数
3. 更新 Sidebar 组件支持快捷访问
4. 添加点击事件处理
5. 测试跨平台兼容性

### 阶段 4：主题系统（优先级：中）
1. 定义主题数据结构
2. 实现 Light 和 Dark 主题
3. 转换 shadcn/ui 颜色值
4. 更新所有组件使用主题
5. 在 App 中初始化默认主题
6. 测试主题切换（手动修改代码）

## 验收标准

本变更完成后应满足：

1. ✅ 静态资源系统正常工作，可以加载图标
2. ✅ 双击文件夹可以进入该文件夹
3. ✅ 默认打开用户主目录
4. ✅ 侧边栏显示快捷访问项（桌面、文档、下载等）
5. ✅ 点击快捷访问项可以跳转
6. ✅ 主题系统实现，支持亮色和暗色主题
7. ✅ 所有组件使用主题系统的颜色和间距
8. ✅ 跨平台测试通过（macOS 和 Windows）

## 风险与权衡

### 风险 1：图标资源准备
- **风险**：需要准备多套 SVG 图标
- **缓解**：先使用 emoji 占位，后续替换为正式图标

### 风险 2：主题切换性能
- **风险**：运行时切换主题可能需要重绘所有组件
- **缓解**：初期只支持启动时设置主题，后续优化切换性能

### 权衡 1：Linux 支持
- **选择**：初期不支持 Linux 平台的快捷文件夹
- **理由**：优先支持主要平台，Linux 支持在后续迭代中添加
