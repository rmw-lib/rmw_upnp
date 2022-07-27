<!-- EDIT /Users/z/rmw/upnp_daemon/doc/README.md -->

[English](#english-readme) | [中文说明](#中文说明)

---

## English Readme

<!-- EDIT /Users/z/rmw/upnp_daemon/doc/en/readme.md -->

upnp_daemon

### Use

[→ examples/main.rs](../examples/main.rs)

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


---

## 中文说明

<!-- EDIT /Users/z/rmw/upnp_daemon/doc/zh/readme.md -->

upnp_daemon : upnp 守护进程

### 使用

[→ examples/main.rs](../examples/main.rs)

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

