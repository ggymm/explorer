## ADDED Requirements

### Requirement: 虚拟列表组件基础结构

系统 SHALL 提供 `VirtualList` 组件，用于高性能渲染大量列表项，仅渲染可视区域内的项目。

#### Scenario: VirtualList 组件定义

- **WHEN** 开发者需要渲染大量列表项（如包含数千文件的目录）
- **THEN** 系统提供 `VirtualList` 结构体，支持泛型类型 `T: Clone + 'static`
- **AND** 组件包含以下核心字段：
  - `items`: 所有列表项数据 `Vec<T>`
  - `item_sizes`: 每个项目的高度信息 `Rc<Vec<Pixels>>`
  - `render_item`: 项目渲染回调
  - `scroll_handle`: 虚拟滚动句柄
  - 状态字段：`loading`、`error`、`empty_text`、`loading_text`

#### Scenario: 组件构建器 API

- **WHEN** 开发者创建 `VirtualList` 实例
- **THEN** 提供以下构建器方法：
  - `new()` - 创建默认实例
  - `items(Vec<T>)` - 设置列表项数据
  - `item_sizes(Rc<Vec<Pixels>>)` - 设置每个项目的高度
  - `render_item(F)` - 设置渲染回调，签名为 `Fn(&T, usize, &Theme) -> AnyElement`（第二参数为索引）
  - `loading(bool)` - 设置加载状态
  - `error(Option<String>)` - 设置错误信息
  - `empty_text(impl Into<String>)` - 设置空状态文本
  - `loading_text(impl Into<String>)` - 设置加载中文本

#### Scenario: 虚拟滚动句柄

- **WHEN** 开发者需要控制列表滚动行为
- **THEN** 系统提供 `VirtualListScrollHandle` 类型
- **AND** 句柄支持以下方法：
  - `new()` - 创建新句柄
  - `scroll_to_item(index, ScrollStrategy)` - 滚动到指定索引
  - `scroll_to_bottom()` - 滚动到列表底部
  - `offset()` - 获取当前滚动偏移
  - `set_offset(Point<Pixels>)` - 设置滚动偏移

### Requirement: 虚拟滚动渲染逻辑

系统 SHALL 实现虚拟滚动核心算法，只渲染可见区域内的列表项。

#### Scenario: 可见范围计算

- **WHEN** 列表进行布局和绘制
- **THEN** 系统根据当前滚动偏移和视口高度计算可见项目范围 `first_visible_index..last_visible_index`
- **AND** 仅调用 `render_item` 渲染可见范围内的项目
- **AND** 使用累积高度算法确定哪些项目在可见区域内

#### Scenario: 项目定位

- **WHEN** 渲染可见项目
- **THEN** 系统为每个项目计算正确的 Y 轴偏移位置
- **AND** 使用预计算的 `item_origins` 向量快速获取项目起始位置
- **AND** 结合滚动偏移调整项目最终渲染位置

#### Scenario: 内容大小计算

- **WHEN** 列表初始化或项目高度变化
- **THEN** 系统计算总内容高度为所有项目高度之和
- **AND** 更新滚动容器的内容大小，确保滚动条正确显示
- **AND** 支持项目间间距（gap）的计算

#### Scenario: 滚动状态保持

- **WHEN** 用户滚动列表
- **THEN** 系统通过 `scroll_handle` 追踪当前滚动偏移
- **AND** 滚动偏移在重新渲染时保持不变（除非主动调整）
- **AND** 支持滚动到边界时的自动限制

### Requirement: 不同高度项目支持

系统 SHALL 支持列表中每个项目具有不同的高度。

#### Scenario: 项目高度配置

- **WHEN** 开发者通过 `item_sizes` 提供每个项目的高度
- **THEN** 系统使用 `Rc<Vec<Pixels>>` 存储高度信息，避免不必要的克隆
- **AND** 每个索引对应一个高度值
- **AND** 项目数量必须与 `items` 数量一致

#### Scenario: 高度预计算

- **WHEN** 列表布局阶段
- **THEN** 系统预计算每个项目的起始 Y 坐标（origins）
- **AND** 使用 `scan` 累积前序项目高度生成 `origins` 向量
- **AND** 缓存计算结果以优化性能

#### Scenario: 动态高度测量

- **WHEN** 项目高度未预先知晓
- **THEN** 系统支持通过 `measure_item` 测量第一个项目的高度
- **AND** 开发者可基于测量结果动态生成 `item_sizes`
- **AND** 支持统一高度列表的快速初始化

### Requirement: 列表状态显示

系统 SHALL 支持加载中、错误和空状态的统一显示。

#### Scenario: 加载状态显示

- **WHEN** `loading` 为 `true`
- **THEN** 系统显示加载中提示，内容为 `loading_text`
- **AND** 不渲染列表项
- **AND** 提示居中显示，使用 `muted_foreground` 颜色

#### Scenario: 错误状态显示

- **WHEN** `error` 为 `Some(message)`
- **THEN** 系统显示错误信息，内容为 `message`
- **AND** 不渲染列表项
- **AND** 错误信息居中显示，使用 `danger` 颜色

#### Scenario: 空状态显示

- **WHEN** `loading` 为 `false`、`error` 为 `None` 且 `items` 为空
- **THEN** 系统显示空状态提示，内容为 `empty_text`
- **AND** 提示居中显示，使用 `muted_foreground` 颜色

#### Scenario: 正常列表显示

- **WHEN** `loading` 为 `false`、`error` 为 `None` 且 `items` 非空
- **THEN** 系统渲染可见范围内的列表项
- **AND** 应用虚拟滚动逻辑

### Requirement: 性能优化特性

系统 SHALL 实现性能优化策略，确保大列表流畅渲染。

#### Scenario: 最小化重新渲染

- **WHEN** 滚动列表时
- **THEN** 系统仅在可见范围变化时重新渲染项目
- **AND** 使用 `ContentMask` 限制绘制区域，避免绘制不可见内容
- **AND** 复用项目元素状态（通过 GPUI 元素状态机制）

#### Scenario: 内存管理

- **WHEN** 列表包含大量项目
- **THEN** 系统仅为可见项目分配 DOM 元素（最多 `visible_range.len()` 个）
- **AND** 不可见项目不占用渲染资源
- **AND** 使用 `SmallVec<[AnyElement; 32]>` 优化小列表的堆分配

#### Scenario: 滚动性能

- **WHEN** 用户快速滚动列表
- **THEN** 系统通过预计算的 `origins` 向量实现 O(n) 可见范围查找（n 为项目总数）
- **AND** 避免每帧重新计算项目位置
- **AND** 滚动操作响应时间保持在 16ms 内（60fps）

### Requirement: GPUI 框架集成

系统 SHALL 正确实现 GPUI 的 `Element` trait，与框架生命周期集成。

#### Scenario: Element trait 实现

- **WHEN** `VirtualList` 作为 GPUI 元素使用
- **THEN** 系统实现 `IntoElement` 和 `Element` traits
- **AND** 正确处理 `request_layout`、`prepaint`、`paint` 生命周期方法
- **AND** 使用 `window.with_element_state` 管理持久化状态

#### Scenario: 元素状态管理

- **WHEN** 列表重新渲染
- **THEN** 系统通过 `VirtualListFrameState` 在渲染帧之间传递状态
- **AND** 缓存 `ItemSizeLayout`（包含 sizes、origins、content_size）
- **AND** 仅在 `item_sizes` 引用变化时重新计算布局

#### Scenario: 交互性支持

- **WHEN** 列表需要响应用户交互
- **THEN** 系统使用 GPUI 的 `Stateful<Div>` 作为基础容器
- **AND** 支持 `overflow_scroll()` 实现原生滚动
- **AND** 通过 `track_scroll(&scroll_handle)` 集成滚动句柄

### Requirement: 组件 API 一致性

系统 SHALL 保持与现有 `List` 组件的 API 一致性，便于开发者迁移。

#### Scenario: 相似的构建器模式

- **WHEN** 开发者从 `List` 迁移到 `VirtualList`
- **THEN** 系统提供相同的方法名称：`items()`、`render_item()`、`loading()`、`error()` 等
- **AND** 额外要求 `item_sizes()` 参数用于虚拟滚动
- **AND** 最小化迁移所需的代码更改

#### Scenario: 渲染回调签名扩展

- **WHEN** 开发者提供项目渲染回调
- **THEN** `VirtualList` 的回调签名为 `Fn(&T, usize, &Theme) -> AnyElement`
- **AND** 第二参数 `usize` 为项目索引，相比 `List` 的 `Fn(&T, &Theme)` 新增索引参数
- **AND** 索引参数用于实现索引相关逻辑（如斑马纹、选中状态）

#### Scenario: 主题集成

- **WHEN** 渲染列表项
- **THEN** 系统通过 `cx.global::<Theme>()` 访问全局主题
- **AND** 将主题传递给 `render_item` 回调，保持与 `List` 一致
- **AND** 支持状态文本使用主题颜色（`muted_foreground`、`danger`）

### Requirement: 模块组织和导出

系统 SHALL 在 `explorer-component` crate 的 `list` 模块中组织虚拟列表代码。

#### Scenario: 文件结构

- **WHEN** 实现虚拟列表组件
- **THEN** 在 `crates/explorer-component/src/list/` 创建 `virtual.rs` 文件
- **AND** 在 `list/mod.rs` 添加 `mod virtual;` 声明
- **AND** 在 `list/mod.rs` 添加 `pub use virtual::*;` 导出

#### Scenario: 公共 API 导出

- **WHEN** 外部代码需要使用虚拟列表
- **THEN** 系统通过 `crates/explorer-component/src/lib.rs` 的 `pub use list::*;` 导出组件
- **AND** 外部可通过 `use explorer_component::{VirtualList, VirtualListScrollHandle};` 导入
- **AND** 保持与现有 `List`、`ListItem` 组件相同的导出方式

#### Scenario: 跨 crate 使用

- **WHEN** `explorer-app` 需要使用虚拟列表
- **THEN** 通过 `use explorer_component::VirtualList;` 导入即可
- **AND** 不需要额外的依赖或配置
- **AND** 与现有组件使用方式一致
