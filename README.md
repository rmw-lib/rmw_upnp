<!-- EDIT /Users/z/rmw/upnp_daemon/README.md -->

# upnp_daemon

<a href="https://docs.rs/upnp_daemon"><img src="https://img.shields.io/badge/RUST-API%20DOC-blue?style=for-the-badge&logo=docs.rs&labelColor=333" alt="Api Doc"></a>
<a href="https://github.com/rmw-lib/upnp_daemon/releases"><img src="https://img.shields.io/badge/Download-EXE-090?style=for-the-badge&logo=rust&labelColor=333" alt="Download"></a>

[English](#english-readme) | [中文说明](#中文说明)

---

## English Readme

<!-- EDIT /Users/z/rmw/upnp_daemon/doc/en/readme.md -->

upnp_daemon

### Use

[→ examples/main.rs](examples/main.rs)

```rust
use anyhow::Result;
use upnp_daemon::upnp_daemon;

#[async_std::main]
async fn main() -> Result<()> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Info)
    .level_for("surf", log::LevelFilter::Warn)
    .chain(std::io::stdout())
    .apply()?;

  let sleep_seconds = 60;
  let port = 12345;
  upnp_daemon("test_upnp_daemon", port, sleep_seconds).await;
  Ok(())
}
```


### About

This project is part of **[rmw.link](//rmw.link)** Code Project

![rmw.link logo](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)

---

## 中文说明

<!-- EDIT /Users/z/rmw/upnp_daemon/doc/zh/readme.md -->

upnp_daemon : upnp 守护进程

### 使用

[→ examples/main.rs](examples/main.rs)

```rust
use anyhow::Result;
use upnp_daemon::upnp_daemon;

#[async_std::main]
async fn main() -> Result<()> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Info)
    .level_for("surf", log::LevelFilter::Warn)
    .chain(std::io::stdout())
    .apply()?;

  let sleep_seconds = 60;
  let port = 12345;
  upnp_daemon("test_upnp_daemon", port, sleep_seconds).await;
  Ok(())
}
```


### 关于

本项目隶属于 **人民网络 ([rmw.link](//rmw.link))** 代码计划。

![人民网络海报](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
