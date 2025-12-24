# 设计文档

## 概述

本变更旨在提升文件浏览器的交互体验，重点改进路径展示和文件选择功能。

## 核心组件设计

### 1. PathBreadcrumb 组件

#### 目的
提供直观的路径层级导航，替代当前的简单字符串展示。

#### 数据结构

```rust
pub struct PathBreadcrumb {
    /// 当前完整路径
    current_path: String,
    /// 是否激活
    is_active: bool,
    /// 路径段点击回调（参数为点击的完整路径）
    on_navigate: Option<Rc<dyn Fn(String, &mut Window, &mut App)>>,
    /// 关闭按钮回调
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    /// 每个段的最大显示宽度（像素）
    max_segment_width: Pixels,
}

struct PathSegment {
    /// 显示名称（可能被截断）
    name: String,
    /// 完整名称（用于 tooltip）
    full_name: String,
    /// 完整路径（用于导航）
    full_path: String,
    /// 是否需要截断显示
    truncated: bool,
}
```

#### 路径解析算法

```rust
fn parse_path(path: &str, max_segment_width: Pixels) -> Vec<PathSegment> {
    // 示例："/Users/ggymm/Documents"
    // 结果：[
    //   PathSegment { name: "/", full_name: "/", full_path: "/", truncated: false },
    //   PathSegment { name: "Users", full_name: "Users", full_path: "/Users", truncated: false },
    //   PathSegment { name: "ggymm", full_name: "ggymm", full_path: "/Users/ggymm", truncated: false },
    //   PathSegment { name: "Documents", full_name: "Documents", full_path: "/Users/ggymm/Documents", truncated: false },
    // ]

    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut segments = vec![
        PathSegment {
            name: "/".to_string(),
            full_name: "/".to_string(),
            full_path: "/".to_string(),
            truncated: false,
        }
    ];

    let mut current_path = String::from("/");
    for part in parts {
        if !current_path.ends_with('/') {
            current_path.push('/');
        }
        current_path.push_str(part);

        // 检查名称是否需要截断
        let (display_name, is_truncated) = truncate_segment(part, max_segment_width);

        segments.push(PathSegment {
            name: display_name,
            full_name: part.to_string(),
            full_path: current_path.clone(),
            truncated: is_truncated,
        });
    }

    segments
}

/// 截断过长的文件夹名称
fn truncate_segment(name: &str, max_width: Pixels) -> (String, bool) {
    // 简化实现：使用字符数估算宽度
    // 实际实现需要使用字体测量 API
    let max_chars = (max_width.0 / 8.0) as usize; // 假设平均字符宽度 8px

    if name.len() <= max_chars {
        (name.to_string(), false)
    } else {
        let truncated = format!("{}...", &name[..max_chars.saturating_sub(3)]);
        (truncated, true)
    }
}
```

#### 路径省略策略

当面包屑链的总宽度超过容器可用宽度时，应用省略策略：

```rust
enum BreadcrumbItem {
    Segment(PathSegment),
    Ellipsis(Vec<PathSegment>), // 包含被省略的中间段
}

fn apply_ellipsis(
    segments: Vec<PathSegment>,
    container_width: Pixels,
    segment_spacing: Pixels, // 包括分隔符和内边距
) -> Vec<BreadcrumbItem> {
    // 1. 计算完整面包屑链的总宽度
    let total_width = calculate_breadcrumb_width(&segments, segment_spacing);

    if total_width <= container_width {
        // 宽度足够，不需要省略
        return segments.into_iter().map(BreadcrumbItem::Segment).collect();
    }

    // 2. 宽度不够，应用首尾显示策略
    // 策略：保留 [0], [1], ..., [-2], [-1]
    // 中间部分用省略号代替
    let mut result = vec![];

    // 保留根目录和第一级（如果存在）
    result.push(BreadcrumbItem::Segment(segments[0].clone()));
    if segments.len() > 1 {
        result.push(BreadcrumbItem::Segment(segments[1].clone()));
    }

    // 添加省略号，包含中间所有段
    if segments.len() > 4 {
        // 只有当段数大于4时才使用省略号
        // 因为 [0][1][...][n-2][n-1] 需要至少5个段才有意义
        let hidden_start = 2;
        let hidden_end = segments.len() - 2;
        let hidden_segments = segments[hidden_start..hidden_end].to_vec();
        result.push(BreadcrumbItem::Ellipsis(hidden_segments));
    } else if segments.len() > 2 {
        // 段数较少（3-4个），但宽度不够
        // 直接显示所有段，依赖单个段的截断机制
        for seg in &segments[2..segments.len()-1] {
            result.push(BreadcrumbItem::Segment(seg.clone()));
        }
    }

    // 添加倒数第二级和当前目录
    if segments.len() > 2 {
        result.push(BreadcrumbItem::Segment(segments[segments.len() - 2].clone()));
    }
    result.push(BreadcrumbItem::Segment(segments[segments.len() - 1].clone()));

    result
}

/// 计算面包屑链的总宽度
fn calculate_breadcrumb_width(segments: &[PathSegment], spacing: Pixels) -> Pixels {
    let mut total = px(0.);

    for (i, seg) in segments.iter().enumerate() {
        // 使用字体测量 API 计算实际文本宽度
        // 简化版本：估算宽度
        let text_width = estimate_text_width(&seg.name);
        total += text_width;

        // 添加内边距和分隔符宽度
        if i < segments.len() - 1 {
            total += spacing; // 包括 padding 和 ChevronRight 图标
        }
    }

    total
}

// 示例：
// 容器宽度 = 400px
// 完整路径：/ > Users > ggymm > Projects > Rust > explorer > src (总宽度 500px)
// 省略后：/ > Users > [...] > explorer > src (总宽度 350px)
```

#### 渲染布局

**正常路径**（宽度足够）：
```
┌─────────────────────────────────────────────────────────────┐
│  / > Users > ggymm > Documents              [Close Button]  │
└─────────────────────────────────────────────────────────────┘
```

**长路径**（宽度不足，应用省略）：
```
┌─────────────────────────────────────────────────────────────┐
│  / > Users > [...] > explorer > src         [Close Button]  │
└─────────────────────────────────────────────────────────────┘
```

**省略号菜单**（点击 [...] 后）：
```
┌─────────────────────────────────────────────────────────────┐
│  / > Users > [...] > explorer > src         [Close Button]  │
│             ┌─────────────┐                                 │
│             │ ggymm       │ ← 点击跳转到 /Users/ggymm      │
│             │ Projects    │ ← 点击跳转到 /Users/ggymm/Projects │
│             │ Rust        │ ← 点击跳转到 /Users/ggymm/Projects/Rust │
│             └─────────────┘                                 │
└─────────────────────────────────────────────────────────────┘
```

**截断的文件夹名称**（悬停显示 tooltip）：
```
┌─────────────────────────────────────────────────────────────┐
│  / > very-long-fold...               [Tooltip: very-long-folder-name] │
└─────────────────────────────────────────────────────────────┘
```

布局细节：
- 左侧：面包屑链（flex-1）
- 右侧：关闭按钮（flex-shrink-0，始终可见）
- 省略号菜单：绝对定位，出现在省略号图标下方
- 省略判断：基于容器可用宽度动态计算

#### 样式规范

```rust
// 面包屑项样式
.cursor_pointer()
.px(spacing.xs)
.py(spacing.xxs)
.rounded(radius.sm)
.hover(|style| style.bg(colors.muted))
.max_w(px(120.)) // 最大宽度 120px
.overflow_hidden()
.text_ellipsis() // CSS text-overflow: ellipsis

// 截断的段显示 tooltip
.when(segment.truncated, |this| {
    this.tooltip(|cx| {
        div().child(segment.full_name.clone())
    })
})

// 分隔符样式（ChevronRight）
.text_color(colors.muted_foreground)
.mx(spacing.xxs)

// 最后一个段（当前目录）
.font_weight(FontWeight::SEMIBOLD)
.text_color(colors.foreground)

// 省略号图标
.cursor_pointer()
.px(spacing.xs)
.py(spacing.xxs)
.rounded(radius.sm)
.hover(|style| style.bg(colors.muted))
.text_color(colors.muted_foreground)

// 省略号菜单容器
.absolute()
.top_full() // 在省略号图标下方
.left_0()
.mt(spacing.xs)
.bg(colors.popover)
.border_1()
.border_color(colors.border)
.rounded(radius.md)
.shadow_lg()
.z_index(50)
.min_w(px(200.))
.max_h(px(400.))
.overflow_y_auto()

// 菜单项样式
.flex()
.items_center()
.px(spacing.md)
.py(spacing.sm)
.cursor_pointer()
.hover(|style| style.bg(colors.accent).text_color(colors.accent_foreground))
.text_sm()

// 关闭按钮
.w(px(24.))
.h(px(24.))
.rounded(radius.sm)
.hover(|style| style.bg(colors.destructive).text_color(colors.destructive_foreground))
```

### 2. 文件选中状态管理

#### 状态设计

```rust
pub struct Explorer {
    // ... 现有字段

    /// 当前选中的项目（使用路径作为唯一标识）
    selected_items: HashSet<String>,

    /// 最后选中的索引（用于 Shift 范围选择）
    last_selected_index: Option<usize>,
}
```

#### 选中逻辑流程图

```
用户点击文件
    ↓
检测修饰键
    ↓
┌─────────────┬──────────────┬──────────────┐
│  无修饰键    │  Ctrl/Cmd    │    Shift     │
└─────────────┴──────────────┴──────────────┘
    ↓              ↓              ↓
清除其他选中    切换当前项    范围选择
    ↓              ↓              ↓
选中当前项    更新选中集    选中范围内所有项
    ↓              ↓              ↓
更新 last_selected_index
    ↓
触发重新渲染
```

#### 多选实现细节

**单选**（默认行为）：
```rust
fn handle_single_select(&mut self, item_path: String) {
    self.selected_items.clear();
    self.selected_items.insert(item_path.clone());
    self.last_selected_index = Some(current_index);
}
```

**Ctrl/Cmd 切换**：
```rust
fn handle_toggle_select(&mut self, item_path: String) {
    if self.selected_items.contains(&item_path) {
        self.selected_items.remove(&item_path);
    } else {
        self.selected_items.insert(item_path.clone());
    }
    self.last_selected_index = Some(current_index);
}
```

**Shift 范围选择**：
```rust
fn handle_range_select(&mut self, current_index: usize, entries: &[FileItem]) {
    if let Some(last_index) = self.last_selected_index {
        let start = last_index.min(current_index);
        let end = last_index.max(current_index);

        for i in start..=end {
            self.selected_items.insert(entries[i].path.clone());
        }
    } else {
        // 如果没有上次选中的项，按单选处理
        self.handle_single_select(entries[current_index].path.clone());
    }
}
```

### 3. ListItem 视觉状态

#### 状态组合矩阵

| 状态           | 背景色          | 边框          | 文本颜色          |
|----------------|----------------|---------------|------------------|
| 默认           | transparent    | none          | foreground       |
| 悬停           | muted          | none          | foreground       |
| 选中           | transparent    | accent (2px)  | foreground       |
| 选中+悬停      | muted          | accent (2px)  | foreground       |

#### 实现策略

```rust
let mut item = div()
    .rounded(radius.md)
    .p(spacing.sm)
    .cursor_pointer();

// 选中状态：添加边框
if selected {
    item = item.border_2().border_color(theme.colors.accent);
}

// 悬停状态：添加背景
item = item.hover(|style| style.bg(theme.colors.muted));
```

**关键点**：边框和背景可同时存在，互不冲突。

### 4. 面板关闭逻辑

#### 树节点移除算法

```rust
impl PanelNode {
    fn remove_panel(&mut self, target_id: PanelId) -> Option<PanelNode> {
        match self {
            PanelNode::Leaf { id, .. } => {
                // 叶子节点：如果是目标节点，返回 None 表示移除
                if *id == target_id {
                    None
                } else {
                    Some(self.clone())
                }
            }
            PanelNode::Split { first, second, .. } => {
                // 尝试从子节点中移除
                let first_result = first.remove_panel(target_id);
                let second_result = second.remove_panel(target_id);

                match (first_result, second_result) {
                    (None, Some(node)) => Some(node),      // 第一个子节点被移除，提升第二个
                    (Some(node), None) => Some(node),      // 第二个子节点被移除，提升第一个
                    (Some(f), Some(s)) => {
                        // 两个子节点都保留，重建 Split
                        Some(PanelNode::Split { first: Box::new(f), second: Box::new(s), .. })
                    }
                    (None, None) => None,                  // 两个都被移除（不应该发生）
                }
            }
        }
    }
}
```

#### 激活面板切换策略

当关闭激活面板时，按以下顺序查找新的激活面板：
1. 优先选择同级的下一个面板（右侧或下方）
2. 如果没有下一个，选择上一个面板（左侧或上方）
3. 如果是唯一面板，禁止关闭

```rust
fn find_next_active_panel(tree: &PanelNode, closed_id: PanelId) -> Option<PanelId> {
    // 深度优先遍历，返回第一个不是 closed_id 的叶子节点
    match tree {
        PanelNode::Leaf { id, .. } => {
            if *id != closed_id {
                Some(*id)
            } else {
                None
            }
        }
        PanelNode::Split { first, second, .. } => {
            find_next_active_panel(first, closed_id)
                .or_else(|| find_next_active_panel(second, closed_id))
        }
    }
}
```

## 技术挑战与解决方案

### 挑战 1：长路径溢出处理

**问题**：当路径层级很深时，面包屑可能超出容器宽度。

**解决方案**：
1. 使用 `overflow_x_scroll()` 允许水平滚动
2. 或实现省略中间段的逻辑：`/ > ... > parent > current`

### 挑战 2：多选性能优化

**问题**：频繁的选中状态变化可能导致性能问题。

**解决方案**：
1. 使用 `HashSet` 提供 O(1) 查询性能
2. 仅在选中状态变化时触发重新渲染，使用 `cx.notify()` 精确控制

### 挑战 3：键盘事件捕获

**问题**：GPUI 的键盘事件需要焦点管理。

**解决方案**：
1. 为文件列表容器添加 `focusable()` 属性
2. 使用 `on_key_down` 处理方向键和修饰键
3. 维护焦点状态，确保键盘事件正确路由

## 权衡与决策

### 决策 1：组件重命名而非扩展

**选项 A**：保留 PanelTitleBar，添加面包屑作为子组件
**选项 B**：重命名为 PathBreadcrumb，专注单一职责

**选择 B 的理由**：
- 名称更准确反映组件功能
- 避免组件职责膨胀
- 简化组件 API，易于维护

### 决策 2：边框 vs 背景色表示选中

**选项 A**：使用背景色（accent）表示选中
**选项 B**：使用边框（accent）表示选中

**选择 B 的理由**：
- 边框与悬停的背景色可同时显示，视觉上更清晰
- 背景色冲突时难以区分选中和悬停
- 边框更符合 Windows 文件管理器的习惯

### 决策 3：面板关闭的树重构策略

**选项 A**：移除节点后重新平衡整个树
**选项 B**：局部调整，仅提升兄弟节点

**选择 B 的理由**：
- 实现简单，性能更好
- 保持用户的面板布局习惯
- 避免意外的布局变化

## 实现顺序建议

1. **阶段 1**：PathBreadcrumb 组件（不依赖其他变更）
2. **阶段 2**：文件选中状态（独立功能）
3. **阶段 3**：面板关闭逻辑（依赖 PathBreadcrumb 的关闭按钮）

每个阶段独立测试和验证，降低集成风险。
