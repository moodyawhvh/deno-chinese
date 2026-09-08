# 发布文档模板

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

- [ ] Fork 这个 gist 并按其中的说明操作。

## 预检

**在此流程期间,$BRANCH_NAME 分支应被冻结,发布完成前不得合入任何提交。**

- [ ] 确保以下仓库的 fork 和本地克隆就绪:
  - [`denoland/deno`](https://github.com/denoland/deno/)
  - [`denoland/dotcom`](https://github.com/denoland/dotcom/)
  - [`denoland/deno_docker`](https://github.com/denoland/deno_docker/)
  - [`denoland/deno-docs`](https://github.com/denoland/deno-docs)
- [ ] 查看 https://deno.land/benchmarks?-100 ,确认近期没有性能回退。
- [ ] 在公司 `#cli` 频道发消息:

```
:lock:

@here

Deno v$VERSION is now getting released.

`denoland/deno` is now locked.

*DO NOT LAND ANY PRs*

Release checklist: <LINK TO THIS FORKED GIST GOES HERE>
```

## 更新 `deno`

### 阶段 1:提升版本号

- [ ] 进入 CLI 仓库 actions 中的 "version_bump" 工作流:
      https://github.com/denoland/deno/actions/workflows/version_bump.generated.yml
  1. 点击 "Run workflow" 按钮。
  1. 在下拉框中选择 `main` 分支。
  1. 发布类型选择 `patch` 或 `minor`。
  1. 运行工作流。

- [ ] 等待工作流完成并自动打开 pull request。审查该 PR,做必要修改后合并。
  - ⛔ **不要**手动创建 release tag,那会自动完成。

  <details>
     <summary>失败时的步骤</summary>

  1. 检出本次发布所在的分支。
  2. 手动运行 `./tools/release/01_bump_crate_versions.ts`
     1. 确认各 crate 版本号已正确提升
     2. 确认 `Releases.md` 已正确更新
  3. 用这些改动开一个 PR,然后继续下面的步骤。
  </details>

### 阶段 2:发布

- [ ] 进入 CLI 仓库 actions 中的 "cargo_publish" 工作流:
      https://github.com/denoland/deno/actions/workflows/cargo_publish.generated.yml
  1. 在刚才同一个分支上运行,等待完成。

  <details>
     <summary>失败时的步骤</summary>

  1. 该工作流设计为可重启,先尝试重启。
  2. 如果不行,按下面操作:
     1. 检出 `v$MINOR_VERSION` 分支。
     2. 如果 `cargo publish` 尚未完成,运行
        `./tools/release/03_publish_crates.ts`
        - 注意这需要 crates.io 权限,可能会失败。
     3. 如果 `cargo publish` 成功但 release tag 未创建,则在 `v$MINOR_VERSION`
        分支上手动创建并推送 `v$VERSION` tag。
  </details>

- [ ] 这次 CI 运行会创建一个 tag,触发第二轮 CI,发布 GitHub 草稿 release。

  CI 流水线会在 GitHub 上创建 release 草稿
  (https://github.com/denoland/deno/releases)。

- ⛔ 核实:
  - [ ] v$VERSION 的 [GitHub release 草稿](https://github.com/denoland/deno/releases/) 有 46 个资产。
  - [ ] [dl.deno.land](https://dash.cloudflare.com/895762025d37fc687ecd72d7cc80204a/r2/default/buckets/dl-deno-land?prefix=release%2Fv$VERSION) 上该版本有 48 个 zip 文件。

- [ ] 在 GitHub 上正式发布 release

## 更新 https://deno.com

- [ ] 运行
      https://github.com/denoland/dotcom/actions/workflows/update_version.yml
      自动打开 PR。
  - [ ] 合并该 PR。

## 更新 https://docs.deno.com

- [ ] 运行
      https://github.com/denoland/deno-docs/actions/workflows/update_versions.yml
      自动打开 PR。
  - [ ] 合并该 PR。

## 更新 `deno_docker`

- [ ] 运行版本提升工作流:
      https://github.com/denoland/deno_docker/actions/workflows/version_bump.yml
- [ ] 它会打开一个 PR。审查并合并。
- [ ] 创建 `$VERSION` tag(**不带** `v` 前缀)。
- [ ] 这会触发发布 CI。确认其成功完成。

## 更新 `deno_pypi`

- [ ] 运行版本提升工作流:
      https://github.com/denoland/deno_pypi/actions/workflows/version-bump.yml
- [ ] 它会打开一个 PR。审查并合并。
- [ ] 运行发布工作流:
      https://github.com/denoland/deno_pypi/actions/workflows/release.yml
- [ ] 这会触发发布 CI。确认其成功完成,并且新版本已出现在
      https://pypi.org/project/deno/ 。

## 更新 MDN

- [ ] 如果本次发布新增或启用了 JavaScript/Web API,确保
      https://github.com/mdn/browser-compat-data 已更新以反映 API 变化。拿不准就联系 @bartlomieju 并跳过此步。

## 添加 `deno upgrade` 横幅

- [ ] 你可以(可选)添加一个横幅,在用户运行
      `deno
      upgrade` 时打印。适用于需要告知用户"运行某命令才能用上新功能"或"存在破坏性变更"的场景。
  - 创建 `banner.txt` 文件,写入你想打印的内容——_必须是纯文本_。
  - 把文件上传到
    https://dash.cloudflare.com/895762025d37fc687ecd72d7cc80204a/r2/default/buckets/dl-deno-land?prefix=release%2Fv$VERSION/banner.txt 。

## 全部完成!

- [ ] 在公司 #cli 频道发消息:

```
:unlock:

@here

`denoland/deno` is now unlocked.

*You can land PRs now*

Deno v$VERSION has been released.
```

## 回滚

万一出了问题:

1. 把 https://dl.deno.land/release-latest.txt 更新回上一个发布版本。
1. 还原 [dotcom 仓库](https://github.com/denoland/dotcom/) 的 PR,避免
   [`setup-deno`](https://github.com/denoland/setup-deno)
   GH action 拉取新版本。
