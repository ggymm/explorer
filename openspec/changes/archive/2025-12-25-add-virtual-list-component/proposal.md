# 变更：添加虚拟滚动列表组件

## 为什么

当前的文件列表组件（`List` 和 `GroupedList`）采用一次性渲染所有列表项的方式，在处理大目录时会导致性能问题：

1. **渲染性能瓶颈**：包含数千个文件的目录会同时渲染所有项目，导致首次渲染缓慢，UI 卡顿
2. **内存占用过高**：所有列表项的 DOM 元素同时存在于内存中，增加内存压力
3. **滚动性能差**：大量 DOM 元素影响滚动性能和响应速度

项目设计文档 (project.md) 明确要求"大目录快速加载（延迟加载、虚拟滚动）"，现有实现未满足此需求。需要引入虚拟滚动技术，只渲染可见区域的列表项，以支持高性能的大文件列表展示。

## 变更内容

- 在 `explorer-component` crate 的 `list` 模块中新增 `VirtualList` 组件
- 实现基于 GPUI 框架的虚拟滚动列表，支持不同高度的列表项
- 提供纵向虚拟滚动功能（优先支持文件列表场景）
- 与现有 `List` 和 `ListItem` 组件保持 API 一致性
- 支持自定义项目渲染器、选中状态、交互回调
- 支持滚动到指定项、滚动句柄管理
- 实现性能优化：只渲染可见范围内的项目，最小化 DOM 操作

## 影响

- **受影响的规范**：
  - `ui-components`：新增虚拟列表组件需求
  - `file-list`：可选择性地使用虚拟列表优化大目录性能（不影响现有功能）

- **受影响的代码**：
  - `crates/explorer-component/src/list/mod.rs`：新增 `virtual.rs` 子模块
  - `crates/explorer-component/src/list/virtual.rs`：新文件，实现虚拟列表组件
  - `crates/explorer-component/src/lib.rs`：导出新组件
  - `crates/explorer-app/src/main.rs`：未来可以使用虚拟列表替代普通列表（可选）

- **非破坏性变更**：现有 `List` 和 `GroupedList` 组件保持不变，`VirtualList` 作为新增选项提供
