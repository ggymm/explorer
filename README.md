### 常用命令

```bash

# cargo install cargo-bloat
cargo bloat --release -n 50
cargo bloat --release --crates

# cargo install cargo-outdated
cargo outdated -R

# 打包
cargo clean
cargo build --release

# 格式化
cargo fmt

# 更新依赖
cargo clean
rm -f Cargo.lock
cargo update

```


```claude

/openspec:apply         实施已批准的 OpenSpec 变更并保持任务同步。 (project)
/openspec:archive       归档已部署的 OpenSpec 变更并更新规范。 (project)
/openspec:proposal      搭建新的 OpenSpec 变更并严格验证。 (project)

/openspec:proposal


/openspec:apply

/openspec:archive 存储变更，并且准备git提交文案

```