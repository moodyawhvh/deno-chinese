# 包管理:`deno add` / `deno install`

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

这是 Deno 为项目添加依赖的完整地图:CLI 参数如何解析、配置文件如何改写、包如何真正装进 `node_modules` 和锁文件。动 `deno add`、`deno install <pkg>` 或控制依赖写入位置的参数(`--dev`、`--save-optional`、`--no-save`、`--save-exact`、`--package-json`)之前,先读这篇。

## 两套参数解析器

参数解析目前存在于**两处**,两者必须保持同步(第二处正在取代第一处,见 CLI-parser-split 工作):

- `cli/args/flags.rs` — 基于 `clap` 的旧解析器。`add` 子命令定义在 `add_subcommand()`;`install` 复用共享的参数构建器(`add_dev_arg()`、`add_optional_arg()`、`add_no_save_arg()`)。两者都汇入 `add_parse_inner()`,由它构建 `AddFlags` 结构体。
- `libs/cli_parser/` — 较新的手写解析器。命令形态声明在 `src/defs.rs`(`ADD_SUBCOMMAND`、`INSTALL_SUBCOMMAND`),在 `src/convert.rs` 转换为 flags(`add_parse`,以及产出 `InstallFlagsLocal::Add` 的 install 分支)。

`AddFlags` 本身定义在 `libs/cli_parser/src/flags.rs`,并通过 `crate::args` 再导出。新增字段时,你必须更新:结构体本身、两套解析器,以及所有 `AddFlags { .. }` 字面量(解析器测试套件 `cli/args/flags.rs` 和 `libs/cli_parser/src/tests_full.rs` 里都有字面量;共享测试用例是 `add_or_install_subcommand`,它会循环遍历 `add` 和 `install` 两个命令)。

有一条 lint(`tools/lint.js` 里的 `ensureNoNonPermissionCapitalLetterShortFlags`)禁止大写字母短参数,除非它在显式允许列表中并有文档化的先例。`-D`(dev)和 `-O`(save-optional)在列表里,依据都是 `npm install` 的对应短参数。

## 依赖写到哪里:配置写入器

`cli/tools/pm/mod.rs` 是核心。`add()` 入口先把每个请求的包解析到具体版本(`find_package_and_select_version_for_req`),再决定改写哪个配置文件。

可能涉及两种配置文件:`deno.json`(写入 `imports`)和 `package.json`(写入某个依赖区块)。`load_configs()` 负责发现它们,而且——重要——如果一个都不存在会_新建_一个,因为 Deno 需要配置文件来管理 `node_modules`。`prefer_npm_config` / `--package-json` / `preferPackageJson` 决定当两者都存在时,npm 包落进哪一个。

实际改写由 `ConfigUpdater::add(selected, kind)` 完成。`kind` 是 `DependencyKind` 枚举(`Normal` / `Dev` / `Optional`):

- `deno.json`:`kind` 被忽略——一切都写进 `imports`。
- `package.json`:`kind` 决定写入哪个区块(`dependencies`、`devDependencies`、`optionalDependencies`)。`add()` 还会把该包从另外两个区块中移除,避免重复声明;`new_dependency_section_index()` 以稳定顺序插入新建区块(`dependencies` → `devDependencies` → `optionalDependencies`)。

`ConfigUpdater::remove()` 与之对称,会清理全部三个区块。

## 包到底怎么安装

配置(可选地)改写并提交后,`add()` 调用 `npm_install_after_modification()`,后者构建一个全新的 `CliFactory`(从磁盘读取编辑后的配置),再调用 `cache_deps::cache_top_level_deps()`。

`cache_top_level_deps()`(位于 `cli/tools/pm/cache_deps.rs`)是 `add`、`remove`、`install`、`outdated`、`audit`、`x` 等命令共用的安装例程。它的模型是:从项目的 **import map**(deno.json 的 imports)和 **package.json 依赖**推导出模块图根集合,结合 npm 解析构建模块图,然后由 `cache_packages()` 把所有东西实体化到 `node_modules`。

### 坑:`optionalDependencies` 永远不会从 `package.json` 安装

安装器只看得见 `dependencies` 和 `devDependencies`。这是外部 `deno_package_json` crate 的限制:`PackageJsonDeps` / `resolve_local_package_json_deps()` 只暴露这两个映射——安装器使用的已解析依赖中没有 `optional_dependencies`。所以写进 `optionalDependencies` 的包**不会**被常规安装路径实体化,哪怕是普通的 `deno install`。

为了让 `--save-optional` 与 `--save-dev`(后者在 add 时_确实会_安装)行为对齐,`add()` 直接安装可选依赖,而不是依赖从配置推导的根。`CacheTopLevelDepsOptions` 有个 `additional_roots:
Vec<Url>` 字段:放进来的任何 specifier 都会加入模块图根并被安装,无论它是否出现在配置里。`add()` 为 `--save-optional` 和 `--no-save`(见下)都会填充它。正经的修法——让安装器尊重 `optionalDependencies`——是一个跨 crate 的独立改动,尚未完成。

## 参数全解

- `--dev` / `-D`:`DependencyKind::Dev`。写入 `devDependencies`,正常安装(dev 依赖在配置推导的根里)。
- `--save-optional` / `-O`:`DependencyKind::Optional`。写入 `optionalDependencies`;由于安装器忽略该区块,包还会被推进 `additional_roots`,保证 add 时就装上。
- `--no-save`:解析并把包安装进 `node_modules` 和锁文件,但**不**改写或提交任何配置文件。实现方式是跳过 `ConfigUpdater::add`/`commit` 调用,并把包推进 `additional_roots`。
- 这三个参数互斥(在两套解析器中都通过 `conflicts_with` 强制)。

`additional_roots` 同样贯穿 `cache_top_level_deps()`:只要存在 import map_或_additional roots,模块图构建段落就会执行,因此 `--no-save` 在只有 package.json 而没有 import map 的项目里也能工作。

## 测试

- Spec 测试在 `tests/specs/add/` 下。相关的有:`dev/`、`save_optional/`、`no_save/`、`package_json_flag/`、`exiting_dev_deps/`。用 `./x test-spec add::` 跑子集。只关心配置结果的 spec 测试(不想看到下载噪音)可以在 `add` 步骤用 `"output": "[WILDCARD]"`,然后在后续 `eval` 步骤里断言文件内容。
- 解析器单元测试:两处都有 `add_or_install_subcommand` —— `cli/args/flags.rs`(通过 `cargo test -p deno --lib add_or_install` 运行)和
  `libs/cli_parser/src/tests_full.rs`
  (`cargo test -p deno_cli_parser
  add_or_install`)。

## 相关工作 / 后续步骤

- 让安装器尊重 `package.json` 的 `optionalDependencies`(上游 `deno_package_json` 的 `resolve_local_package_json_deps` + npm 安装器),这样 `--save-optional` 就能走常规的配置推导路径,并去掉对应的 `additional_roots` 变通。
- 在没有任何配置文件的目录里,`--no-save` 仍会创建一个空配置文件(`load_configs` 的副作用),与"不保存"略有矛盾;常见场景(已有项目)不受影响。
