# 变更：UI 增强 - 交互与主题系统

## Why

当前应用已经具备基础的项目结构和布局，但缺少用户交互功能和视觉主题系统。需要添加：
1. 静态资源管理系统（图标等）
2. 文件/文件夹交互功能（点击、双击）
3. 侧边栏快捷导航（用户文件夹）
4. 主题系统（亮色/暗色主题）

这些功能将使应用从"可以显示"进化到"可以使用"，提供完整的用户体验。

## What Changes

### 1. 静态资源管理（explorer-common）

**新增内容**：
- 在 `explorer-common` 中添加静态资源管理
- 使用 `rust-embed` 库嵌入静态资源
- 提供图标资源访问接口
- 在 `explorer-app` 中使用 GPUI 的 `Assets` 系统注册资源

**影响的规范**：
- **修改** `project-structure`: 添加 common 模块的静态资源职责
- **新增** `assets`: 静态资源管理规范

### 2. 文件列表交互

**新增内容**：
- 为文件列表添加双击事件处理
- 双击文件夹进入该文件夹
- 修改默认路径为用户主目录（跨平台）
- 添加加载状态提示

**影响的规范**：
- **修改** `file-list`: 添加交互行为规范

### 3. 侧边栏快捷导航

**新增内容**：
- 添加侧边栏点击事件
- 支持快捷访问用户文件夹：
  - 桌面（Desktop）
  - 文档（Documents）
  - 下载（Downloads）
  - 图片（Pictures）
  - 音乐（Music）
  - 视频（Videos）
- 跨平台支持（macOS 和 Windows）
- 使用系统图标或预设图标

**影响的规范**：
- **修改** `layout`: 添加侧边栏交互规范
- **新增** `quick-access`: 快捷访问规范

### 4. 主题系统（explorer-component）

**新增内容**：
- 在 `explorer-component` 中实现主题系统
- 支持亮色（Light）和暗色（Dark）主题
- 使用 shadcn/ui 的颜色值
- 主题包含：
  - 背景色（background, foreground）
  - 边框色（border）
  - 强调色（primary, secondary, accent）
  - 状态色（muted, destructive）
  - 间距（padding, margin, gap）
  - 圆角（border-radius）
- 所有组件从主题系统获取样式

**影响的规范**：
- **新增** `theme`: 主题系统规范
- **修改** 所有 component 规范：使用主题系统

## Non-Goals

此变更**不包括**：
- 文件预览功能
- 右键菜单
- 文件操作（复制、移动、删除等）
- 主题切换 UI（将在后续实现）
- Linux 平台的快捷文件夹支持（后续实现）
- 自定义图标上传功能
