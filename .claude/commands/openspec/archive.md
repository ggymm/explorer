---
name: OpenSpec 归档
description: 归档已部署的 OpenSpec 变更并更新规范。
category: OpenSpec
tags: [openspec, archive]
---
<!-- OPENSPEC:START -->
**护栏规则**
- 优先采用直接、最小化的实现方式，仅在明确要求或必要时才增加复杂性。
- 将变更严格限定在请求的结果范围内。
- 如需了解额外的 OpenSpec 约定或说明，请参考 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果找不到，可运行 `ls openspec` 或 `openspec update`）。

**步骤**
1. 确定要归档的变更 ID：
   - 如果此提示已经包含特定的变更 ID（例如在由斜杠命令参数填充的 `<ChangeId>` 块内），则在修剪空格后使用该值。
   - 如果对话宽松地引用了一个变更（例如通过标题或摘要），运行 `openspec list` 以显示可能的 ID，分享相关候选项，并确认用户意图的是哪一个。
   - 否则，查看对话，运行 `openspec list`，并询问用户要归档哪个变更；在继续之前等待确认的变更 ID。
   - 如果仍然无法识别单个变更 ID，停止并告诉用户您还无法归档任何内容。
2. 通过运行 `openspec list`（或 `openspec show <id>`）验证变更 ID，如果变更缺失、已���档或其他原因未准备好归档，则停止。
3. 运行 `openspec archive <id> --yes`，以便 CLI 移动变更并在不提示的情况下应用规范更新（仅对仅工具工作使用 `--skip-specs`）。
4. 查看命令输出以确认目标规范已更新且变更已进入 `changes/archive/`。
5. 使用 `openspec validate --strict` 进行验证，如果有任何问题，使用 `openspec show <id>` 检查。

**参考**
- 在归档之前使用 `openspec list` 确认变更 ID。
- 使用 `openspec list --specs` 检查刷新的规范，并在移交之前解决任何验证问题。
<!-- OPENSPEC:END -->
