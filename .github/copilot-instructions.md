# GitHub Copilot 的 Deno 开发指南

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

> **译者注:** 本文件原文超过 10000 字符,此处翻译核心章节(网络访问、高层概览、构建、代码质量、测试、PR 标题规范)。Pull Request 审查细则、调试、故障排查等其余章节请参阅英文原版。

## 网络访问

本仓库的开发工具需要网络访问才能工作。运行 `tools/format.js` 或 `tools/lint.js` 时,请确保以下域名可达:

- `https://jsr.io` — Deno 的包注册表,用于 `@std/*` 导入
- `https://dprint.dev` 与 `https://plugins.dprint.dev` — 格式化器(`tools/format.js`)运行 `npm:dprint`,会从 `plugins.dprint.dev` 下载 WASM 插件

如果你在沙箱环境中运行,必须把这些域名加入允许列表,否则工具会报网络错误。

## 高层概览

用户可见的界面与高层集成位于 `deno` crate(`./cli`)。

包括参数解析、子命令、包管理工具等。参数解析在 `cli/args/flags.rs`。各工具在 `cli/tools/<tool>`。

`deno_runtime` crate(`./runtime`)组装 JavaScript 运行时,包括全部"扩展"(暴露给 JavaScript 的原生功能)。扩展本身位于 `ext/` 目录,为 JavaScript 提供系统访问——例如文件系统操作与网络。

### 关键目录

- `cli/` — 面向用户的 CLI 实现、子命令与工具
- `runtime/` — JavaScript 运行时的组装与集成
- `ext/` — 为 JS 提供原生功能的扩展(fs、net 等)
- `libs/` — 共享 Rust crate(core、resolver、npm、node_resolver、serde_v8
  等)
- `tests/specs/` — 集成测试(spec 测试)
- `tests/unit/` — 单元测试
- `tests/testdata/` — 测试夹具与数据文件

### 优先理解的文件

1. `cli/main.rs` — 入口,命令路由
2. `cli/args/flags.rs` — CLI 参数解析与结构
3. `runtime/worker.rs` — worker/运行时初始化
4. `runtime/permissions.rs` — 权限系统
5. `cli/module_loader.rs` — 模块加载与解析

### 常见模式

- **Ops** — 暴露给 JavaScript 的 Rust 函数(在各 `ext/` 目录)
- **Extensions** — op 与 JS 代码的功能集合
- **Workers** — JavaScript 执行环境(主 worker、web worker)
- **Resources** — 在 Rust 与 JS 之间传递的托管对象(文件、套接字等)

## 构建

```bash
# Check for compilation errors (fast, no binary output)
cargo check

# Build debug binary
cargo build --bin deno

# Build release version (slow, optimized)
cargo build --release

# Run the dev build
./target/debug/deno eval 'console.log("Hello from dev build")'
```

## 代码质量

提交前务必运行格式化器和 linter:

```bash
# Format all code (uses dprint under the hood)
./tools/format.js

# Lint all code (JS + Rust via clippy)
./tools/lint.js

# Lint only JS/TS (faster, skips clippy)
./tools/lint.js --js

# Lint only Rust
./tools/lint.js --rs
```

格式化器(`tools/format.js`)经由 `npm:dprint@0.47.2` 运行 `dprint`,格式化 TypeScript、JavaScript、JSON、Markdown、YAML 与 Rust(经 `rustfmt`)。配置在 `.dprint.json`。

linter(`tools/lint.js`)对 JS/TS 运行 `deno lint`,对 Rust 运行 `cargo clippy`。

## 测试

```bash
# Run all tests
cargo test

# Filter tests by name
cargo test <nameOfTest>

# Run tests in a specific package
cargo test -p deno_core

# Run just the CLI integration tests
cargo test --bin deno

# Run spec tests only
cargo test specs

# Run a specific spec test
cargo test spec::test_name
```

### 测试组织

- **Spec 测试**(`tests/specs/`)— 主力集成测试
- **单元测试** — 内联在各模块源码中
- **集成测试**(`tests/integration/`)— 其他集成测试
- **WPT**(`tests/wpt/`)— 检查 Web 标准符合性的 Web Platform Tests

### Spec 测试

集成测试的主要形式是 `tests/specs/` 中的 "spec" 测试。每个测试有一个 `__test__.jsonc` 文件,描述要运行的 CLI 命令与期望输出。schema 见 `tests/specs/schema.json`。

输出断言支持通配符:`[WILDCARD]`(任意字符,可跨行)、`[WILDLINE]`(到行尾)、`[WILDCHAR]`(单个字符)、`[WILDCHARS(N)]`(N 个字符)、`[UNORDERED_START]` / `[UNORDERED_END]`(行序无关)。

## Git 工作流与 Pull Request

### PR 标题检查

PR 标题由 CI 校验(见 `.github/workflows/pr.ts`)。标题必须遵循 [Conventional Commits](https://www.conventionalcommits.org),并以下列前缀之一开头:`feat:`、`fix:`、`chore:`、`perf:`、`ci:`、`cleanup:`、`docs:`、`bench:`、`build:`、`refactor:`、`test:`、`Revert`、`Reland`、`BREAKING`。

此外,deno_core/v8 升级**不得**用 `chore:` —— 应使用 `feat:`、`fix:` 或 `refactor:`,标题需描述实际变更。发布 PR(标题形如 `X.Y.Z`)同样有效。

校验脚本在 `tools/verify_pr_title.js`。

### 工作流规则

- 用达意的名称建 feature 分支
- 提交信息清晰、描述性强
- 永远不要 force push —— 合并时所有提交会被 squash
- 改动保持最小、聚焦;避免顺手改无关代码
- 提交前运行 `tools/format.js` 与 `tools/lint.js`

---

其余章节(Development Workflows、Debugging、Pull Request Reviews、Troubleshooting)未翻译,请参阅 [英文原版](https://github.com/denoland/deno/blob/main/.github/copilot-instructions.md)。
