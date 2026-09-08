# Deno 开发指南

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

> **译者注:** 本文件原文超过 10000 字符,此处翻译核心章节(高层概览、快速开始、命令、测试、spec 测试)。Git 工作流细节、调试技巧、故障排查等其余章节请参阅英文原版。

## 高层概览

用户可见的界面与高层集成位于 `deno` crate(`./cli`)。

包括参数解析、子命令、包管理工具等。参数解析在 `cli/args/flags.rs`。各工具在 `cli/tools/<tool>`。

`deno_runtime` crate(`./runtime`)组装 JavaScript 运行时,包括全部"扩展"(暴露给 JavaScript 的原生功能)。扩展本身位于 `ext/` 目录,为 JavaScript 提供系统访问——例如文件系统操作与网络。

### 关键目录

- `cli/` - 面向用户的 CLI 实现、子命令与工具
- `runtime/` - JavaScript 运行时的组装与集成
- `ext/` - 为 JS 提供原生功能的扩展(fs、net 等)
- `tests/specs/` - 集成测试(spec 测试)
- `tests/unit/` - 单元测试
- `tests/testdata/` - 测试夹具与数据文件

## 快速开始

构建之前,请安装所需的前置依赖(Rust、原生编译器、cmake、protobuf 等),并按
[`.github/CONTRIBUTING.md`](.github/CONTRIBUTING.md#building-from-source)
的说明用 `--recurse-submodules` 克隆。

### 构建 Deno

改动之后编译:

```bash
cargo build
```

开发期更快迭代(较少优化):

```bash
cargo build --bin deno
```

运行你的开发版构建:

```bash
./target/debug/deno eval 'console.log("Hello from dev build")'
```

### 带着改动运行

```bash
# Run a local file
./target/debug/deno run path/to/file.ts

# Run with permissions
./target/debug/deno run --allow-net --allow-read script.ts

# Run the REPL
./target/debug/deno
```

## 命令

### 编译与检查

```bash
# Check for compilation errors (fast, no binary output)
cargo check

# Check specific package
cargo check -p deno_runtime

# Build release version (slow, optimized)
cargo build --release
```

### 代码质量

```bash
# Lint the code
./tools/lint.js

# Format the code
./tools/format.js

# Both lint and format
./tools/format.js && ./tools/lint.js
```

## 测试

### 运行测试

```bash
# Run all tests (this takes a while)
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

### 单元测试(`tests/unit/`)

JavaScript/TypeScript 单元测试位于 `tests/unit/`,以 `*_test.ts` 文件存在。通过 `cargo test` 运行:

```bash
# Run all unit tests in a specific file
cargo test unit::webcrypto_test

# Run all unit tests
cargo test unit::

# Run Node.js compatibility unit tests (tests/unit_node/)
cargo test unit_node::crypto_test

# Run all Node.js compat unit tests
cargo test unit_node::
```

不要直接用 `./target/debug/deno test` 跑它们——它们依赖 cargo test 测试架做正确的初始化。

### 测试组织

- **Spec 测试**(`tests/specs/`)- 主力集成测试,执行 CLI 命令并校验输出
- **单元测试**(`tests/unit/`)- 运行时 API 的 JavaScript/TypeScript 单元测试
- **集成测试**(`tests/integration/`)- 其他集成测试
- **WPT**(`tests/wpt/`)- 检查 Web 标准符合性的 Web Platform Tests

## "spec" 测试

Deno 中集成测试的主要形式是 "spec" 测试,位于 `tests/specs`。核心思路是一个 `__test__.jsonc` 文件编排一个或多个测试:每个测试是一条要执行的 CLI 命令,其输出被捕获并做断言。

测试名取自 `__test__.jsonc` 所在目录的名称。

### 新建 spec 测试

1. 在 `tests/specs/` 下建一个名字达意的目录
2. 添加描述测试步骤的 `__test__.jsonc` 文件
3. 加入测试所需的输入文件
4. 添加 `.out` 期望输出文件(或直接内联在 `__test__.jsonc` 中)

示例:

```
tests/specs/my_feature/
  __test__.jsonc
  main.ts
  expected.out
```

### 输出断言

期望输出支持一个小型匹配语言:`[WILDCARD]` 匹配任意字符(可跨行)、`[WILDLINE]` 匹配到行尾、`[UNORDERED_START]`/`[UNORDERED_END]` 之间的行可以任意顺序匹配,`[# ...]` 为行注释。完整 schema 见 `tests/specs/schema.json`。

---

其余章节(Git workflow、Development Workflows、Debugging、Codebase Navigation、Troubleshooting)未翻译,请参阅 [英文原版](https://github.com/denoland/deno/blob/main/CLAUDE.md)。
