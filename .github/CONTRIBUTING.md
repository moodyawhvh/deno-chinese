# 贡献指南

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

> **AI 辅助贡献:** 如果你在撰写贡献内容时使用了 AI 工具(如 Copilot、ChatGPT、Claude、Cursor 等),**必须在 PR 描述中明确声明**。使用 AI 工具本身没有任何惩罚,但一旦被怀疑隐瞒了 AI 的使用,PR 将被拒绝。

> **垃圾信息警告:** 如果你批量创建 issue 或 PR,或者在 issue/PR 下发表多条明显由 AI 生成且毫无实质内容的评论,你的账号可能会被封禁。

本仓库是提供 `deno` CLI 的主仓库。

如果你想修复 `deno` 的 bug 或为其添加新功能,就该往这个仓库贡献。

部分子系统——包括 Node.js 兼容层的很大一部分——是用 JavaScript 和 TypeScript 模块实现的。如果你想做出第一次贡献,这些是绝佳的起点。

[这里](https://node-test-viewer.deno.deno.net/results/latest)列出了 Node.js 测试用例,包括通过和失败的用例。研读这些用例能让你深入了解兼容层在实践中的运作方式,以及哪些地方需要改进。它们也可以作为指南,帮你找到最有价值的贡献方向。

## `./x` 工具

Deno 使用 `./x` 开发者 CLI 来执行常见开发任务,例如构建、测试、格式化和 lint。运行 `./x --help` 可查看所有可用命令。

```sh
./x build       # 构建 deno 二进制文件(调试模式)
./x check       # 快速编译检查(不链接)
./x fmt         # 格式化所有代码
./x lint        # 对所有代码做 lint(JS/TS + Rust)
./x lint-js     # 仅对 JavaScript/TypeScript 做 lint
./x verify      # 提交前快速校验(fmt + lint-js)
./x test        # 运行运行时单元测试
./x node-test   # 运行 Node.js API 单元测试
./x node-compat # 运行 Node.js 兼容性测试
./x spec        # 运行 spec(集成)测试
./x napi        # 运行 NAPI(原生插件)测试
```

## 热模块替换(HMR)模式

在迭代 JavaScript/TypeScript 模块时,建议在 `cargo` 参数中加上 `--features hmr`。这是一种特殊的开发模式:JS/TS 源码不打包进二进制文件,而是在运行时读取,这意味着修改这些源码后无需重新构建二进制。

```sh
# cargo build
cargo build --features hmr

# cargo run -- run hello.ts
cargo run --features hmr -- run hello.ts

# cargo test integration::node_unit_tests::os_test
cargo test --features hmr integration::node_unit_tests::os_test
```

同时记得在编辑器设置里引用这个 feature 开关。VSCode 用户可以把以下内容合并进工作区配置文件:

```jsonc
{
  "settings": {
    "rust-analyzer.cargo.features": ["hmr"],
    // 添加对内部 `ext:*` 模块的解析支持
    "deno.importMap": "tools/core_import_map.json"
  }
}
```

在 VSCode 中使用开发版 LSP:

1. 安装并启用
   [Deno VSCode 扩展](https://marketplace.visualstudio.com/items?itemName=denoland.vscode-deno)
2. 修改 VSCode 设置,把 `deno.path` 指向你的开发版二进制:

```jsonc
// .vscode/settings.json
{
  "deno.path": "/path/to/your/deno/target/debug/deno"
}
```

## 提交 PR

提交 pull request 之前,请确保:

1. `./x fmt` 通过且不改动任何文件
2. `./x lint` 通过(如果只改了 JS/TS,跑 `./x lint-js` 即可)
3. 相关测试通过(使用 `./x test`、`./x spec` 等)

你可以把 `./x verify` 当作提交前的快速检查——它一步完成格式化和 JS/TS lint。

## 从源码构建

以下是从此仓库源码构建 Deno 的说明。如果你只是想使用 Deno,直接下载预编译的可执行文件即可(详见 [`快速开始`](https://docs.deno.com/runtime) 章节)。

### 克隆仓库

> Deno 使用了子模块,克隆时务必加上 `--recurse-submodules`。

**Linux(Debian)**/**Mac**/**WSL**:

```shell
git clone --recurse-submodules https://github.com/denoland/deno.git
```

**Windows**:

1. [启用"开发者模式"](https://www.google.com/search?q=windows+enable+developer+mode)
   (否则创建符号链接需要管理员权限)。
2. 确保你的 git 版本不低于 2.19.2.windows.1。
3. 在检出之前设置 `core.symlinks=true`:

   ```shell
   git config --global core.symlinks true
   git clone --recurse-submodules https://github.com/denoland/deno.git
   ```

### 前置条件

#### Rust

> Deno 要求使用特定版本的 Rust。其他版本或 Rust Nightly 版本可能无法构建。每个版本所需的 Rust 版本在 `rust-toolchain.toml` 文件中指定。

[安装或更新 Rust](https://www.rust-lang.org/tools/install)。检查 Rust 是否安装/更新成功:

```console
rustc -V
cargo -V
```

#### 原生编译器与链接器

Deno 的许多组件需要原生编译器来构建优化过的原生函数。

##### Linux(Debian)/WSL

```shell
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
./llvm.sh 17
apt install --install-recommends -y cmake libglib2.0-dev
```

##### Mac

Mac 用户必须安装 _XCode Command Line Tools_。
([XCode](https://developer.apple.com/xcode/) 已自带 _XCode Command Line Tools_。不装 XCode 的话,运行 `xcode-select --install` 即可单独安装。)

还需要 [CMake](https://cmake.org/),它不随 _Command Line Tools_ 一起提供。

```console
brew install cmake
```

##### Mac M1/M2

Apple aarch64 用户必须安装 `lld`。

```console
brew install llvm lld
# 将 /opt/homebrew/opt/llvm/bin/ 加入 $PATH
```

##### Windows

1. 安装 [VS Community 2019](https://www.visualstudio.com/downloads/),勾选"使用 C++ 的桌面开发"工具集,并确保选中下列必需工具以及全部 C++ 工具。

   - Visual C++ tools for CMake
   - Windows 10 SDK (10.0.17763.0)
   - Testing tools core features - Build Tools
   - Visual C++ ATL for x86 and x64
   - Visual C++ MFC for x86 and x64
   - C++/CLI support
   - VC++ 2015.3 v14.00 (v140) toolset for desktop

2. 启用"Debugging Tools for Windows"。
   - 进入"控制面板" → "程序" → "程序和功能"
   - 选择 "Windows Software Development Kit - Windows 10"
   - → "更改" → "更改" → 勾选 "Debugging Tools For Windows" → "更改"
     →"完成"。
   - 或者使用:
     [Debugging Tools for Windows](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/)
     (注意:它会先下载文件,你需要手动安装
     `X64 Debuggers And Tools-x64_en-us.msi`。)

3. 确保 [CMake](https://cmake.org/download/) 已安装并在 `PATH` 中。一些原生依赖(例如通过 rustls 引入的 `aws-lc-sys`)需要用 CMake 编译 C 代码。Visual Studio 自带的 CMake 只能在"Developer Command Prompt"里使用,在别处运行的 `cargo` 或 rust-analyzer 看不到它,因此建议在 `PATH` 中安装独立版 CMake。

### Python 3

> Deno 需要 [Python 3](https://www.python.org/downloads) 来运行 WPT 测试。请确保 `PATH` 中存在不带后缀的 `python`/`python.exe`,并且它指向 Python 3。

### 构建 Deno

_使用 WSL 时,请确保 `.wslconfig` 分配了足够的内存。建议至少分配 16GB。_

推荐的构建方式是使用 `./x` 工具:

```console
./x build
```

这会构建出调试版二进制 `./target/debug/deno`。你也可以直接用 cargo:

```console
cargo build -vv
```

如果你想从源码构建 Deno 和 V8(用于更底层的 V8 开发,或在没有预编译 V8 的平台上):

```console
V8_FROM_SOURCE=1 cargo build -vv
```

从源码构建 V8 时可能需要更多依赖。V8 构建的更多细节参见
[rusty_v8 的 README](https://github.com/denoland/rusty_v8)。

### 构建

使用 `./x` 工具或 Cargo 构建:

```shell
# 构建:
./x build

# 或直接用 cargo:
cargo build -vv

# 构建报错?先确保你在最新 main 上再试一次,如果仍然失败,尝试:
cargo clean && cargo build -vv

# 运行:
./target/debug/deno run tests/testdata/run/002_hello.ts
```

### 运行测试

使用 `./x` 工具运行测试:

```shell
# 运行运行时单元测试(需要过滤参数):
./x test <filter>

# 运行 spec(集成)测试:
./x spec <filter>

# 运行 Node.js 兼容性测试:
./x node-compat <filter>

# 列出可用测试:
./x test --list
./x spec --list
```

你也可以直接用 cargo 运行测试:

```shell
# 运行全部测试(耗时较长):
cargo test -vv

# 运行特定包的测试:
cargo test -p deno_core
```

### 同时开发多个 crate

如果一次改动跨越多个 Deno crate,你可能需要同时构建多个 crate。建议把所有相关 crate 检出到同一目录下并列存放。例如:

```shell
- denoland/
  - deno/
  - deno_core/
  - deno_ast/
  - ...
```

然后使用
[Cargo 的 patch 特性](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html)
覆盖默认依赖路径:

```shell
cargo build --config 'patch.crates-io.deno_ast.path="../deno_ast"'
```

如果这个改动要持续开发几天,你可能更愿意把 patch 写进 `Cargo.toml` 文件(注意:提交改动前记得删掉):

```sh
[patch.crates-io]
deno_ast = { path = "../deno_ast" }
```

这样会从本地路径构建 `deno_ast` crate 并链接该版本,而不是从 `crates.io` 拉取。

**注意**:`Cargo.toml` 中的依赖版本必须与磁盘上的依赖版本一致。

可以用 `cargo search <dependency_name>` 查看版本信息。
