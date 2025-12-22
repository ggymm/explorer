# 变更：初始化项目结构与基础布局

## 为什么

Explorer 项目当前处于空白状态，需要搭建完整的 Rust workspace 项目结构，实现基础的应用框架和核心 UI 布局，为后续功能开发奠定基础。这是项目开发的第一步，涉及构建系统、依赖管理、模块组织和基本 UI 框架的建立。

## 变更内容

### 项目结构
- 创建 Cargo workspace 配置，定义多个子模块（app、comps、storage、providers/local 等）
- 设置 app 模块作为应用主入口，负责应用生命周期管理
- 设置 comps 模块用于自定义 UI 组件的实现
- 设置 storage 模块定义存储抽象接口
- 设置 providers/local 模块实现本地文件系统访问
- 配置必要的依赖项（GPUI、tokio、async-trait、serde 等）

### 基础布局实现
- 实现左右分栏布局：
  - 左侧边栏：显示存储根节点列表（磁盘、挂载点等）和常用目录
  - 右侧主面板：显示文件列表区域
- 使用 GPUI 框架构建布局组件（SidebarLayout）
- 实现响应式布局，支持调整分栏宽度

### 存储抽象层
- 定义 StorageProvider trait，提供统一的存储访问接口
- 实现 LocalFileSystemProvider，支持本地文件系统访问
- 定义存储无关的数据结构（StorageEntry、StorageRoot 等）
- 为后续扩展网络存储、云盘等提供架构基础

### 文件列表准备
- 通过 StorageProvider 接口实现文件系统读取功能
- 能够获取指定路径的文件和子目录列表
- 定义通用的存储条目数据结构（名称、路径、大小、修改时间、类型等）
- 准备文件列表状态管理机制
- 为文件列表渲染做好数据准备

## 影响

### 受影响的规范
- **新增** `project-structure`：项目结构与模块组织
- **新增** `storage`：存储抽象层接口与实现
- **新增** `layout`：应用布局系统
- **新增** `file-list`：文件列表数据管理

### 受影响的代码
- 根目录：`Cargo.toml`（workspace 配置）
- `crates/app/`：应用主模块
- `crates/comps/`：UI 组件模块
- `crates/storage/`：存储抽象接口
- `crates/providers/local/`：本地文件系统实现
- 未来可能添加的模块：
  - `crates/providers/network/`：网络存储
  - `crates/providers/cloud/`：云盘集成
  - `crates/core/`：核心工具和数据结构

## 非目标

此变更**不包括**以下内容（将在后续提案中实现）：
- 文件操作功能（复制、移动、删除等）
- 多标签页功能
- 分栏浏览功能
- 键盘快捷键
- 搜索和过滤功能
- 完整的主题和样式系统
