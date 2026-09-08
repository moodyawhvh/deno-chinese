# 代码库地图

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

一份按目录逐个介绍的仓库导览,并列出最值得先读的文件。想了解这种布局背后的概念分层,请看 [`architecture.md`](./architecture.md)。

## 顶层目录

| 目录           | 这里的内容                                                  |
| -------------- | ----------------------------------------------------------- |
| `cli/`         | `deno` 二进制:子命令、工具链、LSP、模块加载器               |
| `runtime/`     | `deno_runtime`:组装 JS 运行时与 worker                      |
| `ext/`         | 扩展——暴露给 JavaScript 的原生能力                          |
| `libs/`        | `deno_core` 以及配套的解析/打包 crate                       |
| `tests/`       | 全部测试套件(见 [`testing.md`](./testing.md))              |
| `tools/`       | 开发脚本:`format.js`、`lint.js`、CI 辅助、发布工具          |
| `third_party/` | 内置(vendored)依赖与测试夹具                                |
| `coverage/`    | 覆盖率输出                                                  |

## 优先理解的文件

1. `cli/main.rs` — 入口与命令路由。
2. `cli/args/flags.rs` — 全部 CLI 参数与子命令(`clap`)。
3. `runtime/worker.rs` — worker/运行时如何初始化。
4. `runtime/permissions/` — 为 op 把关的权限系统。
5. `cli/module_loader.rs` — 模块加载与解析。

## `cli/` 内部

- `cli/args/` — 参数解析(`flags.rs`)与解析后的配置。
- `cli/tools/` — 每个子命令一个模块。简单示例:`cli/tools/fmt.rs`;复杂示例:`cli/tools/test/` 目录。其他值得关注的工具还有 `compile.rs`、`bundle/`、`coverage/`、`lint/`、`pm/`(包管理)、`installer/`、`jupyter/`、`publish/`、`repl/` 和 `serve.rs`。
- `cli/lsp/` — 语言服务器。
- `cli/module_loader.rs`、`cli/graph_util.rs` — 模块图的构建与加载。

## `ext/` 内部

每个子目录是一个扩展(一个 Rust crate 加上它的 JavaScript)。按用途粗略分组:

- **Web 平台:** `web`、`fetch`、`url`、`crypto`、`console`、`webidl`、
  `websocket`、`webgpu`、`canvas`、`image`、`broadcast_channel`、`webstorage`。
- **系统访问:** `fs`、`net`、`io`、`os`、`process`、`signals`、`tls`。
- **Deno API:** `kv`、`cron`、`cache`、`ffi`、`napi`、`bundle`、`telemetry`。
- **Node 兼容:** `node`(大部分 `node:*` 内置模块)、`node_crypto`、
  `node_sqlite`。

扩展内部的惯例布局是:Rust op 放在 `lib.rs`(及子模块)中,JavaScript API 文件用数字前缀命名以控制加载顺序(`00_*.js`、`01_*.js`……)。

## `libs/` 内部

`deno_core` 基础层,以及 CLI 组合用于解析与打包的各 crate:

- **核心/V8 桥:** `core`、`core_testing`、`ops`、`serde_v8`、`dcore`。
- **解析与打包:** `resolver`、`node_resolver`、`npm`、`npm_cache`、
  `npm_installer`、`npmrc`、`package_json`、`lockfile`、`config`、`cli_parser`、
  `cache_dir`。
- **其他基础组件:** `crypto`、`dotenv`、`eszip`、`http_h1`、
  `inspector_server`、`maybe_sync`、`napi_sys`、`node_shim`。

## `tests/` 内部

- `tests/specs/` — 主要的集成测试(CLI 命令 + 断言输出),由 `__test__.jsonc` 文件驱动。
- `tests/unit/` — 运行时 API 的 JavaScript/TypeScript 单元测试(`*_test.ts`)。
- `tests/unit_node/` — `node:*` 兼容层的单元测试。
- `tests/node_compat/` — Node.js 自带的测试套件,针对 Deno 运行。
- `tests/wpt/` — Web Platform Tests。
- `tests/testdata/` — 各套件共享的测试夹具。

每个套件的运行命令见 [`testing.md`](./testing.md)。

## `tools/` 内部

开发与 CI 脚本,全部用 Deno 运行:

- `tools/format.js` — 格式化整棵代码树(`deno fmt` 加额外步骤)。
- `tools/lint.js` — lint Rust 与 JS/TS;传 `--js` 只处理 JS/TS。
- `tools/check_deno_core_changes.js`、`tools/check_docs_only_changes.js` — CI 辅助脚本,根据变更文件决定要跑哪些任务。
- `tools/release/` — 发布自动化。

仓库根目录的 `./x` 辅助脚本封装了常用构建/测试命令;运行 `./x --help` 查看功能。
