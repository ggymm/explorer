---
name: OpenSpec 应用
description: 实施已批准的 OpenSpec 变更并保持任务同步。
category: OpenSpec
tags: [openspec, apply]
---
<!-- OPENSPEC:START -->
**护栏规则**
- 优先采用直接、最小化的实现方式，仅在明确要求或必要时才增加复杂性。
- 将变更严格限定在请求的结果范围内。
- 如需了解额外的 OpenSpec 约定或说明，请参考 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果找不到，可运行 `ls openspec` 或 `openspec update`）。

**步骤**
将这些步骤作为 TODO 跟踪并逐一完成。
1. 阅读 `changes/<id>/proposal.md`、`design.md`（如果存在）和 `tasks.md` 以确认范围和验收标准。
2. 按顺序完成任务，保持编辑最小化并专注于请求的变更。
3. 在更新状态之前确认完成——确保 `tasks.md` 中的每一项都已完成。
4. 在所有工作完成后更新检查清单，以便每个任务都标记为 `- [x]` 并反映实际情况。
5. 当需要额外上下文时，参考 `openspec list` 或 `openspec show <item>`。

**参考**
- 在实施时如需从提案中获取额外上下文，使用 `openspec show <id> --json --deltas-only`。
<!-- OPENSPEC:END -->
