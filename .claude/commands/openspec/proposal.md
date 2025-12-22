---
name: OpenSpec 提案
description: 搭建新的 OpenSpec 变更并严格验证。
category: OpenSpec
tags: [openspec, change]
---
<!-- OPENSPEC:START -->
**护栏规则**
- 优先采用直接、最小化的实现方式，仅在明确要求或必要时才增加复杂性。
- 将变更严格限定在请求的结果范围内。
- 如需了解额外的 OpenSpec 约定或说明，请参考 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果找不到，可运行 `ls openspec` 或 `openspec update`）。
- 识别任何模糊或不明确的细节，并在编辑文件前提出必要的后续问题。
- 在提案阶段不要编写任何代码。仅创建设计文档（proposal.md、tasks.md、design.md 和规范增量）。实施将在批准后的应用阶段进行。

**步骤**
1. 查看 `openspec/project.md`，运行 `openspec list` 和 `openspec list --specs`，并检查相关代码或文档（例如通过 `rg`/`ls`），以便将提案建立在当前行为的基础上；记录任何需要澄清的空白。
2. 选择唯一的动词引导的 `change-id`，并在 `openspec/changes/<id>/` 下搭建 `proposal.md`、`tasks.md` 和 `design.md`（在需要时）。
3. 将变更映射到具体的功能或需求中，将多范围的工作拆分为具有明确关系和顺序的不同规范增量。
4. 当解决方案跨越多个系统、引入新模式或需要在提交规范之前讨论权衡时，在 `design.md` 中记录架构推理。
5. 在 `changes/<id>/specs/<capability>/spec.md` 中起草规范增量（每个功能一个文件夹），使用 `## ADDED|MODIFIED|REMOVED Requirements`，每个需求至少包含一个 `#### Scenario:`，并在相关时交叉引用相关功能。
6. 将 `tasks.md` 起草为有序的小型、可验证的工作项列表，这些工作项可提供用户可见的进度，包括验证（测试、工具），并突出依赖关系或可并行化的工作。
7. 使用 `openspec validate <id> --strict` 进行验证，并在共享提案之前解决每个问题。

**参考**
- 当验证失败时，使用 `openspec show <id> --json --deltas-only` 或 `openspec show <spec> --type spec` 检查详细信息。
- 在编写新需求之前，使用 `rg -n "Requirement:|Scenario:" openspec/specs` 搜索现有需求。
- 使用 `rg <keyword>`、`ls` 或直接文件读取探索代码库，以便提案与当前的实施现实保持一致。
<!-- OPENSPEC:END -->
