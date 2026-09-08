# 更新 TypeScript 版本

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

本文档概述如何更新 Deno CLI 内置的 TypeScript 版本。

## 预检清单

- [ ] git 与本仓库的本地克隆
- [ ] 已安装并可用的 npm 包管理器

## 背景知识

TypeScript 引擎(`/cli/tsc/00_typescript.js`)为 Deno CLI 提供类型检查与转换服务。

构建期间(`/cli/build.rs`),TypeScript 引擎与集成代码(`/cli/tsc/99_main_compiler.js`)被载入一个 V8 isolate,该 isolate 再加载多个基线类型库(`/cli/dts/*.d.ts`)以及扩展提供的类型库(`/ext/*`)。随后这个 isolate 被做成快照,包含进 CLI 的构建产物——这就是 _COMPILER_ 快照。

有几个类型库不会载入 isolate,因为它们在运行时不是"默认"库,但我们希望它们能按需动态加载给用户。构建过程中它们作为文本资源打进二进制。

由于 Deno 常常先于 TypeScript 支持某些 Web 标准,我们不得不面对一个不理想的情况:必须"修补" TypeScript 随包发布的类型库。另外,为了让 TypeScript 能访问发行版未包含的类型库,我们还必须修补 TypeScript 内部的库列表,把它们暴露给用户。

尽管理论上这个流程大部分可以自动化,但 Deno 和 TypeScript 双方都可能发生变化,而且往往直到你亲手走一遍流程才发现。鉴于 TypeScript 更新频率不高,而该流程又需要相当多的上下文判断,作者认为花 10-15 分钟手工更新、确保一切符合预期,是最佳选择。

## 更新步骤

1. 如果没有,先在 Deno 源码树之外建一个空的 npm 项目(例如本例中的 `ts`)。
2. 在空项目里安装你想更新到的 TypeScript 版本(例如 `npm i typescript@latest`)。

   - TypeScript 通常也会更新 `beta` 和 `rc` 发布标签,但你要自己核实,确保拿到的是期望的版本。

3. 把 `ts/node_modules/typescript/lib/typescript.js` 复制为
   `deno/cli/tsc/00_typescript.js`。
4. 把 `ts/node_modules/typescript/lib/*.d.ts` 复制到 `deno/cli/dts/`。
5. 删除 `deno/cli/dts/protocol.d.ts`、`deno/cli/dts/tsserverlibrary.d.ts`、
   `deno/cli/dts/typescriptServices.d.ts`,因为 CLI 不会以任何形式包含它们。
6. 分析 `deno/cli/dts` 的 diff:

   - 很多 diff 只是行尾差异,git 提交时会自动归一化。
   - 注意新增的 `lib.es*.d.ts` 文件。每年 TypeScript 通常会在新年后首个版本加入新的 ES target。此外,标准进入 Stage 3 后通常会被加进 `lib.esnext.*.d.ts`。
   - 删除已经"死亡"的 lib 文件。目前 TypeScript 包含
     `lib.esnext.promise.d.ts`、`lib.esnext.string.d.ts`、
     `lib.esnext.weakref.d.ts`,其中定义早已标准化并合入其他库,连
     `lib.esnext.d.ts` 和 TypeScript 编译器本身都不再引用它们。🤷
   - 还原/合并那些提供前向支持的 lib 文件的变更。这是"最难"的部分:你需要判断新版 TypeScript 是否已经包含了我们前向支持的类型定义。目前有:

     - `lib.esnext.array.d.ts` 包含额外的数组 API。它们未来可能挪进 ES2022,但目前只加了 `Array.prototype.at`。你可能需要还原 `lib.esnext.d.ts` 中对该 lib 的删除。
     - 我们添加了 `lib.dom.asynciterable.d.ts`,因为不知为何 TypeScript 一直没把它们并入内置库。(见:
       https://github.com/microsoft/TypeScript/issues/29867)
     - 我们添加了 `lib.dom.extras.d.ts`,因为 TypeScript 对某些 Deno 支持的 DOM 标准跟进较慢,当用户配置 `lib: ["dom", "deno.ns"]` 再使用依赖这些标准的库时会收到令人费解的报错。我们把该库加进 `lib.dom.d.ts`,这样在 Deno 下使用 `dom` lib 时会自动包含。
     - cli/dts/lib.dom.d.ts 中的 Response 额外增加:
       `json(data: unknown, init?: ResponseInit): Response;`

7. 根据 lib 文件的变更,你需要编辑 TypeScript 编译器中库名到文件的映射表
   (`deno/cli/tsc/00_typescript.js`)。该映射目前叫 `libEntries`,大致长这样:

   ```js
   var libEntries = [
     // JavaScript only
     ["es5", "lib.es5.d.ts"],
     ["es6", "lib.es2015.d.ts"],
     ["es2015", "lib.es2015.d.ts"],
     ["es7", "lib.es2016.d.ts"],
     // ...
   ];
   ```

   要确保它与磁盘内容一致。TypeScript 经常把 `esnext.*` 值映射到已标准化版本以降低"破坏性",所以你要确认诸如 `esnext.array` 指向
   `lib.esnext.array.d.ts`。同时要还原 `dom.asynciterable` 和 `dom.extras` 的删除。

8. 对任何新增但未包含进快照的 lib 文件(例如 `lib.es####.full.d.ts`),把它们加进 `deno/cli/tsc.rs` 的 `STATIC_ASSETS`。
9. 对任何将作为快照一部分加载的新 lib 文件(例如 `lib.es####.d.ts`),把它们加进 `deno/cli/build.rs` 的 `libs`。
10. 对我们曾做过前向支持、如今已进入官方库的 API,编辑
    `deno/cli/tests/unit/esnext_test.ts` 删除对应测试,让我们只测自己的修改。
11. 构建并测试这些改动。特别是 `esnext_test.ts` 应当通过,它失败就说明哪里不对。
12. 提交改动并提交 PR。
13. 拍拍自己的背,活干得漂亮。
