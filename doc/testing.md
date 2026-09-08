# 测试

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

Deno 有多个测试套件,各自面向不同的层。本页是一张地图:每个套件是干什么的、怎么跑。具体的操作命令,以仓库根目录的 `CLAUDE.md` 为准;本页补充的是"什么时候该用哪个套件"的上下文。

## 套件一览

| 套件        | 位置                 | 测试…                                  |
| ----------- | -------------------- | -------------------------------------- |
| Spec 测试   | `tests/specs/`       | CLI 命令端到端(主力套件)              |
| 单元测试    | `tests/unit/`        | 来自 JS/TS 的运行时/Web API            |
| unit_node   | `tests/unit_node/`   | `node:*` 兼容层                        |
| Node 兼容   | `tests/node_compat/` | Node 自带测试套件,跑在 Deno 上        |
| WPT         | `tests/wpt/`         | Web Platform Tests(Web 标准)         |
| Rust 测试   | 遍布各处             | crate 级 Rust 逻辑                     |

## Spec 测试 — 主力集成测试

Spec 测试在 `tests/specs/`。每个测试目录含一个 `__test__.jsonc` 文件,描述一个或多个步骤;一个步骤就是一次 `deno` 调用,其输出被捕获并与期望比对。测试名就是目录名。期望语言支持通配符(`[WILDCARD]`、`[WILDLINE]`)、无序块和行内注释;schema 见 `tests/specs/schema.json`。

只要改动能从命令行观察到——新参数、子命令行为、错误消息、解析结果——就该写 spec 测试。

创建步骤:

1. 在 `tests/specs/` 下建一个名字达意的目录。
2. 添加带步骤的 `__test__.jsonc`。
3. 加入测试所需的输入文件。
4. 把期望输出写在行内或 `.out` 文件里。

## 单元测试(`tests/unit/`)

针对运行时和 Web API 的 JavaScript/TypeScript 测试,命名为 `*_test.ts`。适合断言那些最好从运行时内部验证的行为(某个 Web API 的语义、某个 `Deno.*` 方法)。它们跑在 cargo test 测试架下,而不是裸 `deno test`,因为依赖测试架的初始化。

## Node 兼容测试

两回事,经常被混淆:

- `tests/unit_node/` — Deno 自己为 `node:*` 内置模块写的单元测试。
- `tests/node_compat/` — Node.js **自己的**测试文件,跑在 Deno 上以度量兼容性。运行哪些由 `tests/node_compat/config.jsonc` 控制。按操作系统跳过用各 OS 的标志(例如 `"windows": false`),而不是一刀切的 `"ignore"`。

## Web Platform Tests(`tests/wpt/`)

上游 Web Platform Tests,用于检查 Web API 的标准符合性。CI 中在 Linux 上运行。

## Rust 测试

标准 `cargo test`,CI 中对 `deno_core` / `libs/*` crate 另有 `cargo nextest`。`libs/` 下的底层 crate 设计为可独立测试;有些需要同时选中一个同级 crate 才能编译(具体坑见 `CLAUDE.md` 与项目记忆)。

## 运行套件

确切命令见 `CLAUDE.md`。简言之,`./x` 辅助脚本封装了常用项:

- `./x test-spec <name>` — spec 测试。
- `./x test-unit <name>` — 单元测试。
- `./x test-compat <name>` — node 兼容测试。
- `./x test-napi` — NAPI 测试。
- `cargo test unit_node::<module>` — `unit_node` 测试。

运行 `./x --help` 查看全部。推送前,运行 `tools/format.js` 和相应的 `tools/lint.js`(只改了 JS/TS 就加 `--js`)。

## CI 跑什么

在 pull request 上,CI 在所有受支持平台上构建 Deno,并运行 spec、unit、node-compat 和 WPT 套件,外加 `deno_core` crate 测试和 lint。例外是只编辑 `doc/` 的 PR:它只跑 `lint` 任务,跳过其余,因为 Markdown 改动不可能影响二进制。该决策如何做出见 [`ci.md`](./ci.md)。
