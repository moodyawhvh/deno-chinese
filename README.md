<div align="center">

# deno 中文翻译版

**[中文版] deno — 现代 JavaScript / TypeScript / WebAssembly 运行时**

[![原项目](https://img.shields.io/badge/原项目-denoland--deno-blue?style=flat-square&logo=github)](https://github.com/denoland/deno)
[![中文文档](https://img.shields.io/badge/中文文档-README.zh--CN.md-orange?style=flat-square)](README.zh-CN.md)
[![GitHub Stars](https://img.shields.io/github/stars/denoland/deno?style=flat-square&label=原项目Stars)](https://github.com/denoland/deno/stargazers)
[![微信联系](https://img.shields.io/badge/微信-uaycar-brightgreen?style=flat-square&logo=wechat)](#)

</div>

---

> 这是 [denoland/deno](https://github.com/denoland/deno) 的中文翻译版本。
> 完整源代码请访问原项目:https://github.com/denoland/deno

**代部署 / 定制服务 / 技术咨询 请添加微信:uaycar**

---

## 📖 项目简介

Deno(发音 "dee-no")是一个面向 JavaScript、TypeScript 与 WebAssembly 的现代运行时,以"默认安全"和出色的开发者体验著称。它基于 V8 引擎构建,核心使用 Rust 编写,异步运行时基于 Tokio。Deno 内置测试、格式化、Lint、打包等一整套工具链,无需任何配置即可直接运行 TypeScript,同时兼容 npm 生态,是构建 Web 服务器与命令行工具的理想选择。

本仓库是 deno 的中文翻译介绍仓库,包含中文简介与详细中文文档,完整源代码请访问原项目。

## ✨ 主要特性

- 默认安全:文件、网络、环境变量等访问均需显式授权(如 `--allow-net`),按最小权限运行
- 开箱即用的 TypeScript:零配置直接运行 `.ts` 文件,无需额外编译步骤
- 遵循现代 Web 标准:原生提供 fetch、WebSocket、WebGPU 等与浏览器一致的 Web API
- 内置完整工具链:`deno test`、`deno fmt`、`deno lint`、打包器、任务脚本一步到位
- 高性能架构:基于 V8 + Rust + Tokio,发布为单个可执行文件
- npm 兼容:可直接引入 npm 包,无缝衔接海量现有生态
- 官方标准库:常用工具开箱即用(jsr.io/@std)
- JSR 包注册中心:面向现代 JavaScript 与 TypeScript 的开源包管理

## 📁 文件说明

| 文件 | 说明 |
|:-----|:-----|
| README.md | 本文件(中文简介) |
| README.zh-CN.md | 详细中文文档(完整汉化) |

## 🚀 快速开始

1. 安装(Mac / Linux,Shell):

```sh
curl -fsSL https://deno.land/install.sh | sh
```

2. 安装(Windows,PowerShell):

```powershell
irm https://deno.land/install.ps1 | iex
```

3. 也可使用包管理器安装,例如 Homebrew(Mac)或 WinGet(Windows):

```sh
brew install deno
```

```powershell
winget install --id=DenoLand.Deno
```

4. 创建第一个程序 `server.ts`:

```ts
Deno.serve((_req: Request) => {
  return new Response("Hello, world!");
});
```

5. 启动服务器:

```sh
deno run --allow-net server.ts
```

6. 浏览器打开 http://localhost:8000 ,看到 "Hello, world!" 即安装运行成功。更多安装方式与进阶用法见 `README.zh-CN.md`。

完整源代码与最新版本请访问原项目:https://github.com/denoland/deno

## 📞 联系方式

**代部署 / 定制服务 / 技术咨询 请添加微信:uaycar**

---

本项目为 [denoland/deno](https://github.com/denoland/deno) 的中文翻译版本,所有代码版权归原项目作者所有,遵循其原始许可证。

**如果觉得有用,请给原项目点个 Star!** ⭐
