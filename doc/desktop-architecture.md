# Deno Desktop — 内部机制

> 🌐 本文档由 [denoland/deno](https://github.com/denoland/deno) 翻译,英文原版见原项目。

`deno desktop` 是如何把一个原生窗口接到 Deno 运行时上的。本文跳过显而易见的内容(WebView 是什么、`Deno.serve` 干什么),专讲那些不显然的机制:加载模型、ABI 握手、两条传输通道,以及生命周期的边角。

## 倒置的结构:Deno 是库,不是宿主

常规的嵌入方式是应用去 `dlopen` 一个 UI 库。Desktop 反其道而行:**可执行文件是原生后端**(CEF 用 `laufey`,系统 WebView 用 `laufey_webview`);**Deno 运行时是一个名为 `libdenort` 的 cdylib**(`crate-type = ["cdylib"]`,`cli/rt_desktop/`)。后端以 `--runtime <path-to-dylib>` 启动,`dlopen` 该库,并通过 C ABI 调用它。

打包器写出的每个启动器本质上就是这一条调用:

```
exec "$DIR/laufey_webview" --runtime "$DIR/libdenort.so" "$@"   # desktop.rs:1407
```

所以持有 `main()`、事件循环和窗口的那个二进制_不是_Deno——Deno 是后端启动并驱动的客人。

## ABI 握手

这个 dylib 导出 Laufey C ABI —— `laufey_runtime_init` / `_start` / `_shutdown`,由 `laufey::main!(|| { … })` 宏生成(`lib.rs:1095`)。闭包体就是后端在 `init` 之后调用的运行时入口。

版本安全有两层:

- **编译期**(`lib.rs:47`):
  `const _: () = assert!(laufey::LAUFEY_API_VERSION == 26, …)`。如果链接的 `laufey` crate 的 ABI 版本与发行后端所讲的不一致,`cargo build` 会大声失败,而不是产出一个静默无法启动的 dylib。
- **构建期钉版**(`desktop.rs:29`,`LAUFEY_VERSION` 由 `cli/build.rs` 注入):后端二进制版本被钉死,下载会用仓库内的 SHA-256 摘要(`cli/laufey_sums.lock`、`LAUFEY_PINNED_SUMS`)做完整性校验——GitHub releases 页面上不搞 TOFU。`LaufeyBackendResolver` 的解析顺序:`LAUFEY_DEV_DIR` 检出 → 缓存的下载 → 全新下载。

dylib 通过对自身某个函数做 `dladdr` 在磁盘上找到_自己_(`get_dylib_path`,`lib.rs:986`)——自动更新哨兵和定位内嵌载荷都需要它。

## 内嵌载荷

用户代码和资源不是 dylib 旁边的文件,而是 **dylib 内部一个名为 `d3n0l4nd` 的 section**,用 `libsui::find_section_in_current_image` 读回(`find_section_in_dylib`,`lib.rs:1512`)。`extract_standalone_with_finder` 把它解析成 standalone 元数据 + VFS,与 `deno compile` 完全一致,只是来源是已加载的映像而非 argv0。

## 启动顺序(为什么这么严格)

在 `laufey::main!` 里,直到 tokio 运行时建立之前的一切都刻意保持单线程,因为这里发生两个进程级全局操作,一旦有线程存在就不再安全:

1. **运行时构建之前先发布端口**(`lib.rs:1222`):分配一个随机回环端口,然后 `set_var("DENO_SERVE_ADDRESS", "tcp:127.0.0.1:<port>")`。在 glibc 上 `setenv` 不是线程安全的(Rust 1.81+ 将其标记为 `unsafe`),所以必须发生在 mio IO 线程和 inspector 线程之前。用户的 `Deno.serve` / `export default { fetch }` 稍后直接绑定这个预设地址,无需任何协调。
2. **chdir 进入解压后的 VFS**(`lib.rs:1262`):这是进程级操作,必须发生在任何任务解析相对路径之前。各框架(Next 的 `.next/`、Vite 的 `dist/`)都相对 cwd 解析构建产物。

然后 `run_desktop` 在 `tokio::select!` 下并发运行两个 future(`lib.rs:1848`):`denort::run::run_with_options(…)`(Deno 运行时,带 `auto_serve: true`、`serve_port`、`op_state_init`)和 `laufey::run()`(原生事件循环)。第三个 spawn 出来的任务 `navigate_fut` 在两者之间搭桥。

## 导航:用真正的 GET 轮询,而不是测连接

`navigate_fut`(`lib.rs:1773`)在把窗口指过去之前先等服务就绪。它发一个完整的 `GET / HTTP/1.1` 并检查状态行是否为 `2xx`/`3xx`——**而不是**裸 TCP 连接——因为 Vite 这类开发服务器会在真正能服务之前就接受套接字。60 次 × 250ms,然后对初始窗口 id 调用 `Window::navigate(url)`。在 `--inspect-brk`/`--inspect-wait` 下,它先阻塞直到 DevTools 客户端接入多路复用器。

## 两条传输通道

应用内容和 JS↔原生调用走完全分离的路径。

**内容——回环 HTTP。** WebView 就是一个指向 `http://127.0.0.1:<port>/` 的真浏览器。没有任何特殊之处。

**绑定——mpsc 事件 + 每次调用一个 oneshot。** `Deno.desktop`/窗口 API 和 webview→Deno 的函数调用_不_走 HTTP:

- 原生 → 运行时:一条有界 `DesktopEvent` mpsc 通道(`ops/desktop.rs:119`),容量 1024。高频事件(鼠标移动、滚轮)使用 `try_send`,在背压时**直接丢弃**,而不是阻塞或把运行时撑爆(`DesktopEventSender::try_send`,`ops/desktop.rs:244`)。
- 单一 JS 消费者:`DESKTOP_JS` 里的一个异步循环,等待 `op_desktop_recv_event()`(`desktop.rs:702`),把每个事件分发给对应的 DOM 风格 `EventTarget`。op promise 经 `unrefOpPromise` 处理,所以这个泵永远不会单独把事件循环吊着不放。

**一次 bind 调用的往返**(最有意思的部分):

1. JS 注册处理器:`window.bind(name, fn)` 把 `fn` 存进每窗口的 `Map`,并调用 `laufey add_binding_async(name)`(`desktop.rs:229`,`lib.rs:361`)。
2. Webview 调用 `window.bindings.<name>(...)`。原生异步绑定序列化参数(`laufey::Value` → JSON),从全局 `AtomicU32` 分配一个 `call_id`,在它名下注册一个 `oneshot` sender(`register_bind_call`,`ops/desktop.rs:297`),然后发送 `DesktopEvent::BindCall { window_id, name, args, call_id }`。
3. JS 泵的 `"bindCall"` 分支查到处理器,`await` 它,然后调用 `op_desktop_resolve_bind_call(call_id, result)` / `op_desktop_reject_bind_call(call_id, err)`(`desktop.rs:736`)。
4. op 取出该 `call_id` 对应的 `oneshot` 并触发(`ops/desktop.rs:1051`);回到绑定 future,`resp_rx.await` 完成,调用 `js_call.resolve(...)` / `.reject(...)`(`lib.rs:391`)。

正是 `call_id` 键控让同一窗口的多个并发绑定调用能在一条事件通道上多路复用,并把应答准确路由回去。

Webview 看到的 JS 命名空间是 `bindings` —— `lib.rs:1212` 处的 `set_js_namespace("bindings")`,所以调用形如 `window.bindings.foo()`。

## HMR:三种结果,由 V8 裁决

`cli/rt/hmr.rs` 监视源码目录(`notify`,带防抖),把每次变更归类为 `ChangeOutcome`(`hmr.rs:331`),desktop 侧将其映射为 `ReloadKind` 回调(`lib.rs:1642`):

- **Replaced** —— 脚本编辑经 `deno_ast` 转译后被 V8 通过 `Debugger.setScriptSource` 接受。原地热修补,不重载。
- **SoftReload → ReloadKind::Soft** —— 静态资源变更,或某个模块被删除。对 `open_windows` 里的**每一个**窗口执行 `location.reload()`(`lib.rs:1648`),从仍在运行的服务器重新拉取。
- **Restart → ReloadKind::Restart** —— V8 将该编辑判定为顶层 ES 模块变更(imports / 导出绑定 / 顶层 `let` 变化)。模块图无法打补丁,重载也无济于事(旧运行时还在继续服务),于是 `restart_desktop_app()` 调用 `exit(75)`(`lib.rs:1537`)。`deno desktop --hmr` 监护进程持有子进程,捕获退出码 75 并重新拉起——用哨兵方式退出而不是重新 exec,可以把进程组、Ctrl-C 处理和临时入口清理都留给监护进程(`HMR_RESTART_EXIT_CODE`,`lib.rs:1529`)。

对框架开发服务器(`is_framework_dev`),Deno 层的 HMR 完全禁用——框架自己的 websocket HMR 在跑,并且 cwd 保持为源码目录,让框架的监视器能看到真实文件。

## 值得了解的生命周期边角

- **fork 重入守卫**(`lib.rs:1145`):框架开发服务器会 fork 出重新 exec _同一个 dylib_ 的 worker 进程。检测 worker 的依据是 argv 形如 `<exe> run … script.js` **且**存在父级 worker 环境变量(`NODE_CHANNEL_FD` / `NEXT_PRIVATE_WORKER`)——只看环境变量曾是个误报陷阱,因为用户 shell 里可能本来就设了它。检测到的 worker 无头运行(不开窗口)。
- **窗口句柄互操作**(`get_raw_window_handle`,`lib.rs:467`):laufey 的原生句柄按平台转换为 `raw-window-handle` 类型——AppKit / Win32 / X11 / **Wayland**(`LAUFEY_WINDOW_HANDLE_WAYLAND`,`lib.rs:518`)。Wayland 在运行时探测,从而用原生 Wayland 而非 XWayland。
- **`--inspect`**:`cli/tools/desktop_devtools.rs` 在父进程中运行一个 CDP 多路复用器,把 Deno inspector 和渲染进程的调试端口统一挂在一个 `/unified` websocket 后面。
