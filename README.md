<!-- EDIT /Users/z/rmw/rmw_upnp/README.md -->

# rmw_upnp

<a href="https://docs.rs/rmw_upnp"><img src="https://img.shields.io/badge/RUST-API%20DOC-blue?style=for-the-badge&logo=docs.rs&labelColor=333" alt="Api Doc"></a>

[English](#english-readme) | [中文说明](#中文说明)

---

## English Readme

<!-- EDIT /Users/z/rmw/rmw_upnp/doc/en/readme.md -->

rmw_upnp : upnp port map daemon

### Use

[→ examples/main.rs](examples/main.rs)

```rust
use anyhow::Result;
use rmw_upnp::daemon;

#[async_std::main]
async fn main() -> Result<()> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Info)
    .level_for("surf", log::LevelFilter::Warn)
    .chain(std::io::stdout())
    .apply()?;

  let sleep_seconds = 60;
  let port = 12345;
  daemon("test_rmw_upnp", port, sleep_seconds).await;
  Ok(())
}
```


### About

This project is part of **[rmw.link](//rmw.link)** Code Project

![rmw.link logo](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)

---

## 中文说明

<!-- EDIT /Users/z/rmw/rmw_upnp/doc/zh/readme.md -->

rmw_upnp : upnp 守护进程

### 使用

[→ examples/main.rs](examples/main.rs)

```rust
use anyhow::Result;
use rmw_upnp::daemon;

#[async_std::main]
async fn main() -> Result<()> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Info)
    .level_for("surf", log::LevelFilter::Warn)
    .chain(std::io::stdout())
    .apply()?;

  let sleep_seconds = 60;
  let port = 12345;
  daemon("test_rmw_upnp", port, sleep_seconds).await;
  Ok(())
}
```


### 关于

本项目隶属于 **人民网络 ([rmw.link](//rmw.link))** 代码计划。

![人民网络海报](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
