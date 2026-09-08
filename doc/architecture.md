# 架构总览

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

Deno 采用分层架构。每一层只依赖它下面的层,这使系统易于测试,也让底层可以在 `deno` 二进制之外被复用。自顶向下依次为:

```
+-----------------------------------------------------------+
|  cli/            the `deno` binary: subcommands, tooling   |
+-----------------------------------------------------------+
|  runtime/        deno_runtime: assembles the JS runtime    |
+-----------------------------------------------------------+
|  ext/*           extensions: native capabilities for JS    |
+-----------------------------------------------------------+
|  libs/*          deno_core + supporting crates (V8 bridge) |
+-----------------------------------------------------------+
|  V8 + Tokio      JavaScript engine and async runtime       |
+-----------------------------------------------------------+
```

## CLI 层(`cli/`)

`cli/` 中的 `deno` crate 是用户直接接触的一切。它负责参数解析、各子命令(`run`、`test`、`fmt`、`lint`、`compile`、`bundle`、`install`、`publish` 等)、包管理工具、LSP,以及把模块解析接入运行时的模块加载器。

关键入口:

- `cli/main.rs` — 进程入口与命令路由。
- `cli/args/flags.rs` — 完整的 `clap` 参数与子命令定义。新增参数或子命令从这里开始。
- `cli/tools/<tool>/` — 每个子命令一个模块(简单的命令如 `cli/tools/fmt.rs`,复杂的如 `cli/tools/test/`)。
- `cli/module_loader.rs` — 解析并加载模块,把解析器和模块图桥接到运行时。

CLI 层刻意做得很"重":它引入了 TypeScript 类型检查、npm 与 JSR 解析、锁文件以及打包器。下层绝对不允许反向依赖它。

## 运行时层(`runtime/`)

`deno_runtime` crate 用 `deno_core` 加上一组精选的扩展组装出一个可用的 JavaScript 运行时。当嵌入方想要"作为运行时的 Deno"而不想要"作为 CLI 的 Deno"时,用的就是这个部件。

关键文件:

- `runtime/worker.rs` — 构建主 worker:isolate、op 集合以及引导(bootstrap)流程。
- `runtime/web_worker.rs` — Web Worker 变体。
- `runtime/permissions/` — 权限模型,为每个敏感 op(读写文件、网络、环境变量、执行子进程、FFI、系统信息)把关。权限在 Rust 层的 op 边界上检查,绝不在 JavaScript 里检查。

## 扩展层(`ext/*`)

`ext/` 下的每个目录都是一个自包含的扩展:一个 Rust crate,定义 **op**(可从 JS 调用的原生函数),外加在这些 op 之上暴露更高层 API 的 JavaScript 代码。这个平台真正的"血肉"都在这里。例如:

- Web 平台:`ext/web`、`ext/fetch`、`ext/url`、`ext/crypto`、`ext/console`、
  `ext/webidl`、`ext/websocket`、`ext/webgpu`、`ext/canvas`。
- 系统访问:`ext/fs`、`ext/net`、`ext/io`、`ext/os`、`ext/process`、
  `ext/signals`、`ext/tls`。
- Deno 特有:`ext/kv`、`ext/cron`、`ext/cache`、`ext/ffi`、`ext/napi`、
  `ext/bundle`。
- Node 兼容:`ext/node`(`node:*` 内置模块的主体,既含 Rust op 也含 JavaScript polyfill),以及 `ext/node_crypto` 和 `ext/node_sqlite`。

扩展的典型形态是:

1. 若干 Rust `#[op2]` 函数,执行特权工作,并在需要处做权限检查。
2. 一组 `00_*.js` / `01_*.js` JavaScript 模块,构建公开 API,并通过 `Deno.core.ops` 调用 op。
3. 在 `runtime/worker.rs`(以及 CLI 的快照)中注册该扩展,使其成为组装后运行时的一部分。

添加原生功能时,请把 op 放进对应的 `ext/<name>/` crate;不要伸到 runtime 或 CLI 里去实现。

## 核心层(`libs/*`)

`libs/` 存放 `deno_core` 以及从原独立 `deno_core` 仓库合并进来的各个 crate。它是 Rust 与 V8 之间的桥梁:负责 op 基础设施、模块加载器 trait、快照机制、JsRuntime 事件循环以及 `serde_v8` 序列化层。

主要成员:

- `libs/core` — `deno_core` 本体:`JsRuntime`、op 注册、模块表、调试检查器集成。
- `libs/ops` — `#[op2]` 过程宏,生成 Rust/V8 胶水代码。
- `libs/serde_v8` — Rust 类型与 V8 值之间近乎零拷贝的序列化。
- `libs/resolver`、`libs/node_resolver`、`libs/npm`、`libs/npm_installer`、
  `libs/package_json`、`libs/lockfile`、`libs/config`、`libs/npmrc` — CLI 组合使用的模块解析与包管理基础组件。

这些 crate 刻意不掺入任何 CLI 逻辑,以便独立做单元测试,并被其他工具复用。

## 横切概念

- **Op** 是 JavaScript 触达原生代码的唯一途径。op 是暴露给 JS 的 Rust 函数;同步 op 立即返回,异步 op 返回一个在事件循环上完成的 future。
- **扩展(Extensions)** 把 op 与对应的 JavaScript 打包在一起,是运行时组合的基本单元。
- **Worker** 是相互隔离的 JavaScript 执行环境(主 worker 和 Web Worker);每个 worker 拥有独立的 V8 isolate。
- **资源(Resources)** 是由 `deno_core` 跟踪管理的句柄(打开的文件、套接字、reader 等),通过整数 id 跨越 Rust/JS 边界传递。
- **权限(Permissions)** 在 Rust 层的 op 边界强制执行。未授予的能力会让 op 在做任何实际工作之前直接报错。

## 改动去哪做

| 你想…                              | 从这里开始…                        |
| ------------------------------------ | --------------------------------- |
| 新增或修改 CLI 参数/子命令           | `cli/args/flags.rs`、`cli/tools/` |
| 给 JS 新增原生能力                   | `ext/<name>/`(op + JS)           |
| 修改运行时的组装方式                 | `runtime/worker.rs`               |
| 动 Rust/V8 桥或 op 宏                | `libs/core`、`libs/ops`           |
| 修改模块/npm/JSR 解析                | `libs/resolver`、`libs/npm`、CLI  |

更细粒度的目录地图见 [`codebase-map.md`](./codebase-map.md),各层如何测试见 [`testing.md`](./testing.md)。
