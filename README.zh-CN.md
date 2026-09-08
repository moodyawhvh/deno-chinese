<div align="center">

# deno 中文文档

[![原项目](https://img.shields.io/badge/原项目-denoland--deno-blue?style=flat-square&logo=github)](https://github.com/denoland/deno)
[![英文原版](https://img.shields.io/badge/英文原版-README-blue?style=flat-square)](https://github.com/denoland/deno#readme)
[![微信联系](https://img.shields.io/badge/微信-uaycar-brightgreen?style=flat-square&logo=wechat)](#)

</div>

---

> 本文档是 [denoland/deno](https://github.com/denoland/deno) 官方 README 的中文翻译版本。
> 完整源代码请访问原项目:https://github.com/denoland/deno

**代部署 / 定制服务 / 技术咨询 请添加微信:uaycar**

---

## 项目简介

[Deno](https://deno.com)(发音 `/ˈdiːnoʊ/`,读作 "dee-no")是一个支持 JavaScript、TypeScript 与 WebAssembly 的运行时,默认安全、开箱即用,开发者体验极佳。它基于 [V8](https://v8.dev/) 引擎构建,核心由 [Rust](https://www.rust-lang.org/) 编写,异步运行时采用 [Tokio](https://tokio.rs/)。

更多关于 Deno 运行时的内容,请阅读[官方文档](https://docs.deno.com/runtime/manual)。

## 安装

使用下面任意一条命令,即可在你的系统上安装 Deno 运行时。Deno 的安装方式有很多种,完整的安装选项列表见[官方安装文档](https://docs.deno.com/runtime/manual/getting_started/installation)。

Shell(Mac、Linux):

```sh
curl -fsSL https://deno.land/install.sh | sh
```

PowerShell(Windows):

```powershell
irm https://deno.land/install.ps1 | iex
```

[Homebrew](https://formulae.brew.sh/formula/deno)(Mac):

```sh
brew install deno
```

[Chocolatey](https://community.chocolatey.org/packages/deno)(Windows):

```powershell
choco install deno
```

[WinGet](https://winstall.app/apps/DenoLand.Deno)(Windows):

```powershell
winget install --id=DenoLand.Deno
```

[Scoop](https://scoop.sh/#/apps?q=deno&id=678d8fb557b611df996989c675b1099630a5bbee)(Windows):

```powershell
scoop install main/deno
```

### 从源码构建并安装

从源码构建 Deno 的完整说明,请参阅原项目 [CONTRIBUTING.md](https://github.com/denoland/deno/blob/main/.github/CONTRIBUTING.md#building-from-source) 中的 "Building from source" 章节。

## 你的第一个 Deno 程序

Deno 可以用于多种应用场景,其中最常见的用途是构建 Web 服务器。创建一个名为 `server.ts` 的文件,写入以下 TypeScript 代码:

```ts
Deno.serve((_req: Request) => {
  return new Response("Hello, world!");
});
```

使用以下命令启动服务器:

```sh
deno run --allow-net server.ts
```

随后 Deno 会在 [http://localhost:8000](http://localhost:8000) 上启动一个本地 Web 服务器。

更多编写与运行 Deno 程序的内容,请阅读[官方文档](https://docs.deno.com/runtime/manual)。

## 更多资源

- **[Deno Docs](https://docs.deno.com)**:Deno 运行时与 [Deno Deploy](https://deno.com/deploy) 等产品的官方指南与参考文档。
- **[Deno Standard Library](https://jsr.io/@std)**:官方支持、面向 Deno 程序的常用工具标准库。
- **[JSR](https://jsr.io/)**:面向现代 JavaScript 与 TypeScript 的开源包注册中心。
- **[Developer Blog](https://deno.com/blog)**:Deno 团队的官方博客,包含产品更新、教程等内容。

## 参与贡献

欢迎参与贡献!贡献之前,请先阅读原项目中的[贡献指南](https://github.com/denoland/deno/blob/main/.github/CONTRIBUTING.md)。

---

## 版权说明

本文档为 [denoland/deno](https://github.com/denoland/deno) 官方 README 的中文翻译版本,所有代码与原始内容版权归原项目作者所有,遵循其原始许可证。

**代部署 / 定制服务 / 技术咨询 请添加微信:uaycar**

**如果觉得有用,请给原项目 [denoland/deno](https://github.com/denoland/deno) 点个 Star!** ⭐
